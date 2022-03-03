use std::fs;
use std::io::Cursor;
use std::io::{self, Read as _};
use std::sync::Arc;

use anyhow::{anyhow, Context as _, Error, Result};
use cosmwasm_std::Uint128;
use futures::FutureExt as _;
use futures::TryFutureExt;
use reqwest::{ClientBuilder, StatusCode, Url};
use serde::Deserialize;
use tame_gcs::objects::{DownloadObjectResponse, ListOptional, ListResponse};
use tame_gcs::{http, objects::Object, types::ObjectIdentifier, ApiResponse, BucketName, Scopes};
use tame_oauth::gcp::{self as oauth, TokenProvider as _};

mod config;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug, Deserialize)]
struct PoolsResponse {
    dex: String,
    factory: String,
    height: u64,
    pools: Vec<Pool>,
}

#[derive(Debug, Deserialize)]
struct Pool {
    contract_addr: String,
    liquidity_token: String,
    total_share: Uint128,
    assets: [Asset; 2],
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    kind: Kind,
    address: String,
    amount: Uint128,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Kind {
    Asset,
    Native,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args: config::Args = argh::from_env();

    let bucket_name = BucketName::try_from(args.bucket).context("failed to parse bucket name")?;
    let account_info = oauth::ServiceAccountInfo::deserialize(
        fs::read(args.service_account).context("failed to read service account file")?,
    )
    .context("failed to parse service account info")?;
    let token_provider = oauth::TokenProviderWrapper::ServiceAccount(
        oauth::ServiceAccountProvider::new(account_info)
            .context("failed to create token provider")?,
    );
    let client = ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()
        .context("failed to construct HTTP client")?;

    let ctx = RequestContext {
        auth: Arc::new(token_provider),
        client,
    };

    let mut page_token: Option<String> = None;
    loop {
        let req = Object::list(
            &bucket_name,
            Some(ListOptional {
                delimiter: None,
                prefix: Some("raw/"),
                page_token: page_token.as_ref().map(|pt| pt.as_ref()),
                ..Default::default()
            }),
        )?;
        let res: ListResponse = execute(&ctx, req).await?;
        for o in &res.objects {
            dbg!(&o.id, &o.name);

            let oid = gs_url_to_object_id(&Url::parse(&format!(
                "gs://m-etl-bucket-dev/{}",
                o.name.as_ref().unwrap()
            ))?)?;
            let req = Object::download(&(oid.bucket(), oid.object().unwrap()), None)?;
            let res: DownloadObjectResponse = execute(&ctx, req).await?;

            let pools = serde_json::from_slice::<PoolsResponse>(&res.consume())?;

            dbg!(&pools);
        }
        dbg!(&res.prefixes);
        dbg!(&res.page_token);

        page_token = res.page_token;
        if page_token.is_none() {
            break;
        }
    }

    todo!()
}

pub struct RequestContext {
    pub client: reqwest::Client,
    pub auth: Arc<oauth::TokenProviderWrapper>,
}

/// Executes a GCS request via a reqwest client and returns the parsed response/API error
pub async fn execute<B, R>(ctx: &RequestContext, mut req: http::Request<B>) -> Result<R, Error>
where
    R: ApiResponse<bytes::Bytes>,
    B: io::Read + Send + 'static,
{
    // First, get our oauth token, which can mean we have to do an additional
    // request if we've never retrieved one yet, or the one we are using has expired
    let token = match ctx.auth.get_token(&[tame_gcs::Scopes::FullControl])? {
        oauth::TokenOrRequest::Token(token) => token,
        oauth::TokenOrRequest::Request {
            request,
            scope_hash,
            ..
        } => {
            let (parts, body) = request.into_parts();
            let read_body = std::io::Cursor::new(body);
            let new_request = http::Request::from_parts(parts, read_body);

            let req = convert_request(new_request, &ctx.client)
                .await
                .context("failed to create token request")?;
            let res = ctx
                .client
                .execute(req)
                .await
                .context("failed to send token request")?;

            let response = convert_response(res)
                .await
                .context("failed to convert token response")?;

            ctx.auth
                .parse_token_response(scope_hash, response)
                .context("failed to parse token response")?
        },
    };

    // Add the authorization token, note that the tame-oauth crate will automatically
    // set the HeaderValue correctly, in the GCP case this is usually "Bearer <token>"
    req.headers_mut()
        .insert(http::header::AUTHORIZATION, token.try_into()?);

    let request = convert_request(req, &ctx.client).await?;
    let response = ctx.client.execute(request).await?;
    let response = convert_response(response)
        .await
        .context("failed to convert response")?;

    Ok(R::try_from_parts(response)?)
}

/// Converts a `reqwest::Response` into a vanilla `http::Response`. This currently copies
/// the entire response body into a single buffer with no streaming
async fn convert_response(res: reqwest::Response) -> Result<http::Response<bytes::Bytes>, Error> {
    let mut builder = http::Response::builder()
        .status(res.status())
        .version(res.version());

    let headers = builder
        .headers_mut()
        .ok_or_else(|| anyhow!("failed to convert response headers"))?;

    headers.extend(
        res.headers()
            .into_iter()
            .map(|(k, v)| (k.clone(), v.clone())),
    );

    let content_len = tame_gcs::util::get_content_length(headers).unwrap_or_default();
    let mut buffer = bytes::BytesMut::with_capacity(content_len);

    let mut stream = res.bytes_stream();

    use bytes::BufMut;
    use futures_util::StreamExt;

    while let Some(item) = stream.next().await {
        buffer.put(item?);
    }

    Ok(builder.body(buffer.freeze())?)
}

/// Converts a vanilla `http::Request` into a `reqwest::Request`
async fn convert_request<B>(
    req: http::Request<B>,
    client: &reqwest::Client,
) -> Result<reqwest::Request, Error>
where
    B: std::io::Read + Send + 'static,
{
    let (parts, mut body) = req.into_parts();

    let uri = parts.uri.to_string();

    let builder = match parts.method {
        http::Method::GET => client.get(&uri),
        http::Method::POST => client.post(&uri),
        http::Method::DELETE => client.delete(&uri),
        http::Method::PATCH => client.patch(&uri),
        http::Method::PUT => client.put(&uri),
        method => return Err(anyhow!("{} not implemented", method)),
    };

    let content_len = tame_gcs::util::get_content_length(&parts.headers).unwrap_or(0);
    let mut buffer = bytes::BytesMut::with_capacity(content_len);

    let mut block = [0u8; 8 * 1024];

    loop {
        let read = body.read(&mut block)?;

        if read > 0 {
            buffer.extend_from_slice(&block[..read]);
        } else {
            break;
        }
    }

    Ok(builder
        .headers(parts.headers)
        .body(buffer.freeze())
        .build()?)
}

#[derive(Debug)]
pub struct GsUrl {
    bucket_name: tame_gcs::BucketName<'static>,
    obj_name: Option<tame_gcs::ObjectName<'static>>,
}

impl GsUrl {
    pub fn bucket(&self) -> &tame_gcs::BucketName<'_> {
        &self.bucket_name
    }

    pub fn object(&self) -> Option<&tame_gcs::ObjectName<'_>> {
        self.obj_name.as_ref()
    }
}

/// Converts a `gs://<bucket_name>/<object_name>` url into a regular object identifer
pub fn gs_url_to_object_id(url: &Url) -> Result<GsUrl, Error> {
    match url.scheme() {
        "gs" => {
            let bucket_name = url
                .host_str()
                .ok_or_else(|| anyhow!("no bucket specified"))?;
            // Skip first /
            let object_name = &url.path()[1..];

            Ok(GsUrl {
                bucket_name: tame_gcs::BucketName::try_from(String::from(bucket_name))?,
                obj_name: tame_gcs::ObjectName::try_from(String::from(object_name)).ok(),
            })
        },
        scheme => Err(anyhow!("invalid url scheme: {}", scheme)),
    }
}
