#[cfg(feature = "server")]
pub use paste::paste;

#[macro_export]
macro_rules! make_caches {
    {$($name:ident : $key:ty : $value:ty),*} => {
        $crate::mokacaches::paste! {
            pub struct Caches {
                $(
                    pub  [<$name _cache>] : moka::future::Cache< $key , Option<$value> >,
                )*
            }
        }
    };
}
