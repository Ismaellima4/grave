# 🪦 Grave

A lightweight Rust web framework built on top of [Axum](https://docs.rs/axum)
that provides a declarative `app!` macro for defining routes and server
configuration.

## Quick Start

```rust
use grave::app;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

#[derive(Serialize)]
struct User {
    name: String,
}

impl User {
    fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Clone)]
struct AppState {
    message: String,
}

app! {
    Config => {
        port: 8081,
        host: "127.0.0.1",
        state: AppState {
            message: "Hello from State!".to_string()
        }
    },

    GET "/hello" => { "Hello world" },

    GET "/state" => |grave::State(state): grave::State<AppState>| {
        state.message.clone()
    },

    POST "/users" => |grave::Json(body): grave::Json<CreateUser>| {
        grave::Json(User::new(body.name))
    },

    GET "/hello/:name" => |Path(name): Path<String>| {
        format!("Hello, {}!", name)
    }
}
```

## Features

- **Declarative routing** — define routes with `GET "/path" => { handler }`
- **Application state** — shared state via `grave::State<T>`
- **JSON support** — request/response serialization with `grave::Json<T>`
- **Path parameters** — extract URL params with `Path<T>`
- **Testable** — generates a `create_app()` function for integration testing
- **Built on Axum** — full compatibility with the Axum ecosystem

## Supported HTTP Methods

`GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`

## Handler Types

### Block handler (no extractors)

```rust
GET "/hello" => { "Hello world" }
```

### Closure handler (with extractors)

```rust
GET "/users/:id" => |Path(id): Path<u32>| {
    format!("User #{}", id)
}
```

## Testing

The `app!` macro generates a `pub fn create_app() -> Router` that you can use
with `tower::ServiceExt::oneshot` for integration testing:

```rust
#[tokio::test]
async fn test_hello() {
    let app = create_app();
    let response = app
        .oneshot(Request::builder().uri("/hello").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

Run tests:

```bash
cargo test
```

## Running

```bash
cargo run --example hello_world
```

## License

MIT
