# params-derive

Motivating this change is the need to expose subsets of configuration of
binaries dynamically so they can change over time. Possible features
range from kill switches for business critical components to
fine-grained controls of parameters to tune runtime behaviour.

While this can be achieved in a low-touch fashion with a stringly typed
map, it's giving up a lot of what the type system can offer. As
reflection is not really a concept in Rust turning to more idiomatic
primitives by ways of macros is the approach taking in this changeset.

Consider the following example importing the proc macro implemented in
this change-set:

``` rust
use params_derive::params;

/// Such params, much joy
#[params]
pub struct Params {
    /// for curious eyes only.
    pub readable: String,

    /// change things dynamically at runtime.
    #[value(dynamic)]
    pub toggle: bool,
}
```

will produce the following replacement:

``` rust
/// Such params, much joy
pub struct Params {
    /// for curious eyes only.
    pub readable: String,

    /// change things dynamically at runtime.
    pub toggle: std::sync::Arc<std::sync::RwLock<bool>>,
}
```

Wrapping dynamic fields with an `RwLock` ensures internal mutability to
avoid global locking of the entire struct and only serialise access to
the value for the write pass.

To enable the update of dynamic values the current approach is to offer
an HTTP API which can be plugged on a per need basis.

For the struct above the following extra functionality will be
generated:

``` rust
impl Params {
    pub fn router(self) -> (Params, axum::Router) {
        use std::sync::Arc;
        let params = Arc::new(self.clone());
        let router = axum::Router::new()
            .route(
                "/toggle",
                axum::routing::put({
                    let param = Arc::clone(&self.toggle);
                    move |body| Self::put_param::<bool>(body, param)
                }),
            )
            .route(
                "/",
                axum::routing::get({
                    let params = Arc::clone(&params);
                    move || Self::get_params(Arc::clone(&params))
                }),
            );
        (self, router)
    }
    async fn get_params(params: std::sync::Arc<Params>) -> axum::http::Response<axum::body::Body> {
        let body = serde_json::to_vec(&params).unwrap();
        axum::http::Response::builder().body(body.into()).unwrap()
    }
    async fn put_param<V>(
        axum::Json(value): axum::Json<V>,
        param: std::sync::Arc<std::sync::RwLock<V>>,
    ) -> impl axum::response::IntoResponse {
        let mut param = param.write().unwrap();
        *param = value;
        axum::http::StatusCode::NO_CONTENT
    }
}
```

which when bound to a server in the consuming binary offers the
following:

```
|> curl --silent localhost:9999/ | jq .
{
  "readable": {
    "dynamic": false,
    "value": "foo"
  },
  "toggle": {
    "dynamic": true,
    "value": false
  }
}

|> curl -i -XPUT -H "Content-Type: application/json" -d true localhost:9999/toggle
HTTP/1.1 204 No Content
date: Mon, 14 Mar 2022 09:32:18 GMT

|> curl --silent localhost:9999/ | jq .
{
  "readable": {
    "dynamic": false,
    "value": "foo"
  },
  "toggle": {
    "dynamic": true,
    "value": true
  }
}
```
