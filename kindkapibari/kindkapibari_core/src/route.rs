#[macro_export]
macro_rules! route {
    { $( $path:literal => $handler:expr ),* } => {
        #[must_use]
        pub fn routes() -> axum::Router {
            axum::Router::new()
                $( .route($path, $handler) )*
        }
    };
}
