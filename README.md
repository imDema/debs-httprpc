# HTTP REST to gRPC proxy for DEBS

```
USAGE: proxy BIND_ADDRESS BACKEND_ADDRESS
```

### Changing

Uses the protobuf defined in `proto/challenger.proto`.

Edit the `tonic::include_proto!("challenger");` in `proxy.rs` if filename changes.
Edit `build.rs` to add serde derive macros to types if names change.
Edit the endpoints in the last part of `proxy.rs` to map the REST endpoints to gRPC calls.
Edit the routes in `async fn main` where the `Router` is created

### API

The REST api is defined in the `openapi.yml` (update it if the API changes)
It can be rendered using https://editor.swagger.io/, https://github.com/Redocly/redoc, https://docs.scalar.com/swagger-editor

To deploy it I'd recommend the guide in https://github.com/scalar/scalar using their CDN