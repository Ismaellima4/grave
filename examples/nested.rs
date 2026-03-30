use grave::app;

app! {
    Config => {
        port: 8081,
        host: "127.0.0.1"
    },

    NESTED "/hello" => {
        GET "/:name" => |Path(name): Path<String>| {
            format!("Hello, {}!", name)
        }
    }
}
