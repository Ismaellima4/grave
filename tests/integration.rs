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

    NESTED "/users" => {
        POST "/" => |grave::Json(body): grave::Json<CreateUser>| {
            grave::Json(User::new(body.name))
        },

        GET "/:name" => |Path(name): Path<String>| {
            format!("User: {}", name)
        },
    },

    GET "/hello/:name" => |Path(name): Path<String>| {
        format!("Hello, {}!", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http::Request;
    use http::StatusCode;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_hello_returns_hello_world() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello world");
    }

    #[tokio::test]
    async fn test_get_state_returns_state_message() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/state")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello from State!");
    }

    #[tokio::test]
    async fn test_post_users_returns_json() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"name":"grave"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let user: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(user["name"], "grave");
    }

    #[tokio::test]
    async fn test_get_hello_name_returns_greeting() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello/world")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, world!");
    }

    #[tokio::test]
    async fn test_group_get_route() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users/grave")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"User: grave");
    }

    #[tokio::test]
    async fn test_nested_path_extraction() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users/antigravity")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"User: antigravity");
    }

    #[tokio::test]
    async fn test_not_found_returns_404() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
