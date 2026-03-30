/// Helper macro: converts HTTP method ident to axum routing function.
#[macro_export]
#[doc(hidden)]
macro_rules! __to_method {
    (GET) => {
        $crate::axum::routing::get
    };
    (POST) => {
        $crate::axum::routing::post
    };
    (PUT) => {
        $crate::axum::routing::put
    };
    (DELETE) => {
        $crate::axum::routing::delete
    };
    (PATCH) => {
        $crate::axum::routing::patch
    };
    (HEAD) => {
        $crate::axum::routing::head
    };
}

/// Helper macro: parses closure params via tt-munching until the closing `|`.
#[macro_export]
#[doc(hidden)]
macro_rules! __parse_handler {
    // Base case: found closing `|` followed by `{ body }`, optional comma, and rest of routes.
    ($router:expr, $method:ident, $path:literal, [$($params:tt)*], | { $($body:tt)* } , $($rest:tt)*) => {
        $crate::__add_routes!(
            $router.route(
                $path,
                $crate::__to_method!($method)(| $($params)* | async move { $($body)* })
            ),
            $($rest)*
        )
    };
    // Base case: last route (no trailing comma).
    ($router:expr, $method:ident, $path:literal, [$($params:tt)*], | { $($body:tt)* }) => {
        $router.route(
            $path,
            $crate::__to_method!($method)(| $($params)* | async move { $($body)* })
        )
    };
    // Recursive case: consume one token tree and continue.
    ($router:expr, $method:ident, $path:literal, [$($params:tt)*], $tok:tt $($rest:tt)*) => {
        $crate::__parse_handler!($router, $method, $path, [$($params)* $tok], $($rest)*)
    };
}

/// Helper macro: dispatches each route definition.
#[macro_export]
#[doc(hidden)]
macro_rules! __add_routes {
    // Base cases: no more routes.
    ($router:expr ,) => { $router };
    ($router:expr) => { $router };

    // NESTED with trailing comma: NESTED "/prefix" => { routes... } , rest...
    ($router:expr, NESTED $prefix:literal => { $($nested_routes:tt)* } , $($rest:tt)*) => {{
        let nested = $crate::axum::Router::new();
        let nested = $crate::__add_routes!(nested, $($nested_routes)*);
        $crate::__add_routes!($router.nest($prefix, nested), $($rest)*)
    }};
    // NESTED as last item: NESTED "/prefix" => { routes... }
    ($router:expr, NESTED $prefix:literal => { $($nested_routes:tt)* }) => {{
        let nested = $crate::axum::Router::new();
        let nested = $crate::__add_routes!(nested, $($nested_routes)*);
        $router.nest($prefix, nested)
    }};

    // Block handler (no closure params): METHOD "/path" => { body }
    ($router:expr, $method:ident $path:literal => { $($body:tt)* } , $($rest:tt)*) => {
        $crate::__add_routes!(
            $router.route($path, $crate::__to_method!($method)(|| async { $($body)* })),
            $($rest)*
        )
    };
    // Block handler — last route (no trailing comma).
    ($router:expr, $method:ident $path:literal => { $($body:tt)* }) => {
        $router.route($path, $crate::__to_method!($method)(|| async { $($body)* }))
    };

    // Closure handler: METHOD "/path" => |params| { body }
    ($router:expr, $method:ident $path:literal => | $($rest:tt)*) => {
        $crate::__parse_handler!($router, $method, $path, [], $($rest)*)
    };
}

/// The main `app!` macro — defines routes, configuration, and generates
/// `create_app()` (for testing) and `main()` (for running).
///
/// # With state
/// ```ignore
/// app! {
///     Config => {
///         port: 8080,
///         host: "127.0.0.1",
///         state: AppState { field: value }
///     },
///     GET "/hello" => { "Hello world" },
/// }
/// ```
///
/// # Without state
/// ```ignore
/// app! {
///     Config => {
///         port: 8080,
///         host: "127.0.0.1"
///     },
///     GET "/hello" => { "Hello world" },
/// }
/// ```
#[macro_export]
macro_rules! app {
    // ── With state ──────────────────────────────────────────────────────
    (
        Config => {
            port: $port:expr,
            host: $host:expr,
            state: $state_type:ident $state_init:tt
        },

        $($routes:tt)*
    ) => {
        pub fn create_app() -> $crate::Router {
            #[allow(unused_imports)]
            use $crate::extract::Path;

            let state = $state_type $state_init;
            let router = $crate::axum::Router::new();
            $crate::__add_routes!(router, $($routes)*).with_state(state)
        }

        #[cfg(not(test))]
        #[$crate::tokio::main]
        async fn main() {
            let addr = format!("{}:{}", $host, $port);
            println!("🪦 Grave listening on {}", addr);
            let listener = $crate::tokio::net::TcpListener::bind(&addr)
                .await
                .unwrap();
            $crate::axum::serve(listener, create_app()).await.unwrap();
        }
    };

    // ── Without state ───────────────────────────────────────────────────
    (
        Config => {
            port: $port:expr,
            host: $host:expr
        },

        $($routes:tt)*
    ) => {
        pub fn create_app() -> $crate::Router {
            #[allow(unused_imports)]
            use $crate::extract::Path;

            let router = $crate::axum::Router::new();
            $crate::__add_routes!(router, $($routes)*)
        }

        #[cfg(not(test))]
        #[$crate::tokio::main]
        async fn main() {
            let addr = format!("{}:{}", $host, $port);
            println!("🪦 Grave listening on {}", addr);
            let listener = $crate::tokio::net::TcpListener::bind(&addr)
                .await
                .unwrap();
            $crate::axum::serve(listener, create_app()).await.unwrap();
        }
    };
}
