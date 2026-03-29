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
