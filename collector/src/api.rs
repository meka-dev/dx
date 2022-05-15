use std::{io::BufRead as _, sync::Arc};

use axum::{
    extract::{Extension, Path, TypedHeader},
    routing::post,
    Router,
};
use bytes::Bytes;
use common::{
    metrics::{self, Registry, RequestDuration},
    Context as _,
    Result,
};
use hyper::{server::accept::Accept, Server, StatusCode};
use prometheus_client::registry::Unit;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::Mutex,
};
use tower_http::trace::TraceLayer;
use tracing::instrument;

use crate::{
    error::Error,
    event::Event,
    namespace::Namespace,
    producer::Producer,
    segment::{Segment, Sink},
};

pub struct Api {
    request_duration: RequestDuration,
}

impl Api {
    pub fn new(registry: &mut Registry) -> Self {
        let request_duration = metrics::new_request_duration();
        registry.register_with_unit(
            "api_request_duration",
            "API HTTP request durations",
            Unit::Seconds,
            Box::new(request_duration.clone()),
        );

        Self { request_duration }
    }

    pub async fn bind<I>(self, incoming: I, segments: Sink) -> Result<()>
    where
        I: Accept,
        I::Conn: AsyncRead + AsyncWrite + Unpin + Send + 'static,
        I::Error: Send + Sync + std::error::Error + 'static,
    {
        let request_duration = Arc::new(Mutex::new(self.request_duration));
        let router = Router::new()
            .nest("/v1", v1_routes(segments))
            .layer(TraceLayer::new_for_http())
            .layer(axum::middleware::from_fn({
                let request_duration = Arc::clone(&request_duration);
                move |req, next| metrics::middleware(req, next, Arc::clone(&request_duration))
            }));

        Server::builder(incoming)
            .serve(router.into_make_service())
            .await
            .context("failed to serve API")
    }
}

fn v1_routes(segments: Sink) -> Router {
    Router::new()
        .route("/events/:namespace", post(v1_events_handler))
        .layer(Extension(segments))
}

#[instrument(skip(segments, payload), err(Debug), ret)]
async fn v1_events_handler(
    Extension(segments): Extension<Sink>,
    TypedHeader(producer): TypedHeader<Producer>,
    Path(namespace): Path<String>,
    payload: Bytes,
) -> Result<StatusCode, Error> {
    if payload.is_empty() {
        return Err(Error::EmptyPayload);
    }

    let namespace = Namespace::try_from(namespace).map_err(|err| Error::InvalidNamespace {
        reason: err.to_string(),
    })?;

    // Extract all events from newline delimited lines. Abort early in case an error is
    // encountered.
    let lines = payload
        .lines()
        .enumerate()
        .map(|(index, res)| (index + 1, res))
        .map(|(line_number, res)| match res {
            Err(_err) => Err(Error::ParseLines { line_number }),
            Ok(input) => Ok((line_number, input)),
        })
        .collect::<Result<Vec<(usize, String)>, Error>>()?;

    // Attempt to transform inputs captured from lines into events, abort on error and return
    // contextual information in the response.
    let events = lines
        .into_iter()
        .map(|(line_number, input)| {
            Event::try_from(input).map_err(|err| Error::InvalidHex {
                line_number,
                reason: err.to_string(),
            })
        })
        .collect::<Result<Vec<Event>, Error>>()?;

    segments
        .try_send(Segment::new(namespace, producer, events))
        .map_err(Error::from)?;

    Ok(StatusCode::CREATED)
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use common::{metrics::Registry, Result};
    use hyper::{Body, Client, Request, StatusCode};
    use hyperlocal::{SocketIncoming, UnixClientExt as _, Uri};
    use lazy_static::lazy_static;
    use tempfile::tempdir;
    use tokio::sync::mpsc::{self, error::TryRecvError};
    use tower::ServiceExt as _;

    use super::{v1_routes, Api, Error};
    use crate::{event::Event, namespace::Namespace, producer, segment::Segment};

    lazy_static! {
        static ref TEST_NAMESPACE: Namespace =
            Namespace::try_from("mekatek-collector_test-event_v2".to_string()).unwrap();
        static ref TEST_PRODUCER: producer::Producer =
            producer::Producer::try_from("producer").unwrap();
        static ref TEST_PAYLOAD: String = hex::encode("msg=test level=info");
    }

    #[tokio::test]
    async fn bound() -> Result<()> {
        let dir = tempdir()?;
        let socket_path = dir.path().join("api.sock");
        let socket = SocketIncoming::bind(socket_path.clone())?;
        let (segments, mut rx) = mpsc::channel::<Segment>(1);
        let mut registry = Registry::new("test-collector");
        let api = Api::new(&mut registry);

        let url = Uri::new(
            socket_path.clone(),
            &format!("/v1/events/{}", **TEST_NAMESPACE),
        );

        tokio::spawn(api.bind(socket, segments));

        let client = Client::unix();

        let res = client
            .request(
                Request::builder()
                    .method("POST")
                    .header(producer::HEADER_NAME, TEST_PRODUCER.to_string())
                    .uri(url)
                    .body(TEST_PAYLOAD.clone().into())?,
            )
            .await?;

        assert_eq!(
            res.status(),
            StatusCode::CREATED,
            "returns correct status code"
        );

        let segment = rx.try_recv()?;
        assert_eq!(segment.namespace, *TEST_NAMESPACE);
        assert_eq!(segment.producer, *TEST_PRODUCER);
        assert_eq!(segment.events[0].1, Event::try_from(TEST_PAYLOAD.clone())?);

        Ok(())
    }

    #[tokio::test]
    async fn v1_events_success() -> Result<()> {
        let (segments, mut rx) = mpsc::channel::<Segment>(1);
        let router = v1_routes(segments);

        let res = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header(producer::HEADER_NAME, TEST_PRODUCER.to_string())
                    .uri(format!("/events/{}", **TEST_NAMESPACE))
                    .body(TEST_PAYLOAD.clone().into())?,
            )
            .await?;

        assert_eq!(res.status(), StatusCode::CREATED);

        let segment = rx.try_recv()?;
        assert_eq!(segment.namespace, *TEST_NAMESPACE);
        assert_eq!(segment.producer, *TEST_PRODUCER);
        assert_eq!(segment.events[0].1, Event::try_from(TEST_PAYLOAD.clone())?);

        Ok(())
    }

    #[tokio::test]
    async fn v1_events_empty_payload() -> Result<()> {
        let (segments, mut rx) = mpsc::channel::<Segment>(1);
        let router = v1_routes(segments);

        let res = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header(producer::HEADER_NAME, TEST_PRODUCER.to_string())
                    .uri(format!("/events/{}", **TEST_NAMESPACE))
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        matches!(rx.try_recv(), Err(TryRecvError::Empty));

        Ok(())
    }

    #[tokio::test]
    async fn v1_events_invalid_hex() -> Result<()> {
        let (events, _rx) = mpsc::channel(1);
        let router = v1_routes(events);

        let res = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header(producer::HEADER_NAME, TEST_PRODUCER.to_string())
                    .uri(format!("/events/{}", **TEST_NAMESPACE))
                    .body(
                        [TEST_PAYLOAD.to_string(), "foobar".to_string()]
                            .join("\n")
                            .clone()
                            .into(),
                    )?,
            )
            .await?;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let error = serde_json::from_slice::<Error>(&bytes)?;
        assert_matches!(error, Error::InvalidHex { line_number, ..  } if line_number == 2);

        Ok(())
    }

    #[tokio::test]
    async fn v1_events_sink_closed() -> Result<()> {
        let (segments, mut rx) = mpsc::channel::<Segment>(1);
        let router = v1_routes(segments);

        rx.close();

        let res = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header(producer::HEADER_NAME, TEST_PRODUCER.to_string())
                    .uri(format!("/events/{}", **TEST_NAMESPACE))
                    .body(TEST_PAYLOAD.clone().into())?,
            )
            .await?;

        assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);

        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let error = serde_json::from_slice::<Error>(&bytes)?;
        assert_matches!(error, Error::SinkClosed);

        Ok(())
    }

    #[tokio::test]
    async fn v1_events_sink_full() -> Result<()> {
        let (segments, _rx) = mpsc::channel::<Segment>(1);
        let router = v1_routes(segments.clone());

        segments
            .send(Segment::new(
                TEST_NAMESPACE.clone(),
                TEST_PRODUCER.clone(),
                Vec::new(),
            ))
            .await?;

        let res = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header(producer::HEADER_NAME, TEST_PRODUCER.to_string())
                    .uri(format!("/events/{}", **TEST_NAMESPACE))
                    .body(TEST_PAYLOAD.clone().into())?,
            )
            .await?;

        assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);

        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let error = serde_json::from_slice::<Error>(&bytes)?;
        assert_matches!(error, Error::SinkFull);

        Ok(())
    }
}
