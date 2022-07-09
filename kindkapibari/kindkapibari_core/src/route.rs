#[macro_export]
macro_rules! route {
    { $( $path:literal => $handler:expr ),* } => {
        #[must_use]
        pub fn routes() -> axum::Router {
            axum::Router::new()
                $( .route($path, $handler) )*
        }
    };
    { $( $path:literal => $handler:path ),* } => {
        #[must_use]
        pub fn routes() -> axum::Router {
            axum::Router::new()
                $( .nest($path, $handler::routes()) )*
        }
    };
    { $( $handler:path ),* } => {
        #[must_use]
        pub fn routes() -> axum::Router {
            axum::Router::new()
                $( .merge($handler::routes()) )*
        }
    };
}
