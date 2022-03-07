use std::{fs, io};

use anyhow::{anyhow, Context as _, Error, Result};
use axum::http::HeaderValue;
use bytes::BufMut as _;
use futures_lite::stream::StreamExt as _;
use reqwest::{Client, ClientBuilder};
use tame_gcs::{
    http,
    objects::{InsertObjectOptional, InsertResponse, Metadata, Object},
    ApiResponse, BucketName,
};
use tame_oauth::gcp::{self as oauth, TokenProvider as _};

use crate::storage::bucket::{Backend, Bucket, Id, Labeled};

pub struct Gcs<'bucket> {
    auth: oauth::TokenProviderWrapper,
    bucket_name: BucketName<'bucket>,
    client: Client,
}

#[async_trait::async_trait]
impl<'bucket> Bucket for Gcs<'bucket> {
    async fn insert<'body, B>(
        &self,
        name: String,
        content_length: u64,
        content_type: Option<String>,
        body: B,
    ) -> Result<Option<Id>>
    where
        B: io::Read + Send + 'body,
    {
        let req = Object::insert_multipart(
            &self.bucket_name,
            body,
            content_length,
            &Metadata {
                name: Some(name),
                content_encoding: Some("identity".to_owned()),
                ..Default::default()
            },
            Some(InsertObjectOptional {
                content_type: content_type.as_deref(),
                ..Default::default()
            }),
        )
        .context("failed to construct insert request")?;
        let res: InsertResponse = self.execute(req).await?;

        Ok(res.metadata.id.map(Id::from))
    }
}

impl<'bucket> Labeled for Gcs<'bucket> {
    const BACKEND: Backend = Backend::Gcs;

    fn bucket_name(&self) -> String {
        self.bucket_name.to_string()
    }
}

impl<'bucket> Gcs<'bucket> {
    pub fn new<U>(bucket_name: String, service_account_file: String, user_agent: U) -> Result<Self>
    where
        U: TryInto<HeaderValue>,
        U::Error: Into<http::Error>,
    {
        let bucket_name =
            BucketName::try_from(bucket_name).context("failed to parse bucket name")?;

        let account_info = oauth::ServiceAccountInfo::deserialize(
            fs::read(service_account_file).context("failed to read service account file")?,
        )
        .context("failed to parse service account info")?;
        let auth = oauth::TokenProviderWrapper::ServiceAccount(
            oauth::ServiceAccountProvider::new(account_info)
                .context("failed to create token provider")?,
        );

        let client = ClientBuilder::new()
            .user_agent(user_agent)
            .build()
            .context("failed to construct HTTP client")?;

        Ok(Self {
            auth,
            bucket_name,
            client,
        })
    }

    /// Executes a GCS request via a reqwest client and returns the parsed response/API error
    pub async fn execute<'body, B, R>(&self, mut req: http::Request<B>) -> Result<R, Error>
    where
        R: ApiResponse<bytes::Bytes>,
        B: io::Read + Send + 'body,
    {
        // First, get our oauth token, which can mean we have to do an additional
        // request if we've never retrieved one yet, or the one we are using has expired
        let token = match self.auth.get_token(&[tame_gcs::Scopes::FullControl])? {
            oauth::TokenOrRequest::Token(token) => token,
            oauth::TokenOrRequest::Request {
                request,
                scope_hash,
                ..
            } => {
                let (parts, body) = request.into_parts();
                let read_body = std::io::Cursor::new(body);
                let new_request = http::Request::from_parts(parts, read_body);

                let req = self
                    .convert_request(new_request)
                    .await
                    .context("failed to create token request")?;
                let res = self
                    .client
                    .execute(req)
                    .await
                    .context("failed to send token request")?;

                let response = self
                    .convert_response(res)
                    .await
                    .context("failed to convert token response")?;

                self.auth
                    .parse_token_response(scope_hash, response)
                    .context("failed to parse token response")?
            },
        };

        // Add the authorization token, note that the tame-oauth crate will automatically
        // set the HeaderValue correctly, in the GCP case this is usually "Bearer <token>"
        req.headers_mut()
            .insert(http::header::AUTHORIZATION, token.try_into()?);

        let request = self.convert_request(req).await?;
        let response = self.client.execute(request).await?;
        let response = self
            .convert_response(response)
            .await
            .context("failed to convert response")?;

        Ok(R::try_from_parts(response)?)
    }

    /// Converts a `reqwest::Response` into a vanilla `http::Response`. This currently copies
    /// the entire response body into a single buffer with no streaming
    async fn convert_response(
        &self,
        res: reqwest::Response,
    ) -> Result<http::Response<bytes::Bytes>, Error> {
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

        while let Some(item) = stream.next().await {
            buffer.put(item?);
        }

        Ok(builder.body(buffer.freeze())?)
    }

    /// Converts a vanilla `http::Request` into a `reqwest::Request`
    async fn convert_request<'body, B>(
        &self,
        req: http::Request<B>,
    ) -> Result<reqwest::Request, Error>
    where
        B: std::io::Read + Send + 'body,
    {
        let (parts, mut body) = req.into_parts();

        let uri = parts.uri.to_string();

        let builder = match parts.method {
            http::Method::GET => self.client.get(&uri),
            http::Method::POST => self.client.post(&uri),
            http::Method::DELETE => self.client.delete(&uri),
            http::Method::PATCH => self.client.patch(&uri),
            http::Method::PUT => self.client.put(&uri),
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
}
