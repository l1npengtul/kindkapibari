#[cfg(feature = "server")]
pub use paste::paste;

#[macro_export]
macro_rules! make_caches {
    {$($name:ident : $key:ty : $value:ty),*} => {
        $crate::makeconfig_macros::paste! {
            #[derive(Debug)]
            pub struct Caches {
                $(
                    pub  [<$name _cache>] : moka::future::Cache< $key , $value >,
                )*
            }
        }
    };
}
