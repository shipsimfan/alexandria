/// Defines a handle that references another type
///
/// Internally, this will produce a type that uses an `Arc`
#[macro_export]
macro_rules! define_handle {
    (
        $(#[$meta: meta])*
        $vis: vis $outer: ident -> $inner: ty
    ) => {
        $(#[$meta])*
        #[derive(Clone)]
        $vis struct $outer {
            /// The handle to the underlying item
            inner: std::sync::Arc<$inner>,
        }

        impl $outer {
            #[doc = std::concat!("Create a new [`", std::stringify!($outer), "`] from an [`", std::stringify!($inner), "`]")]
            pub(crate) fn from_inner(inner: $inner) -> $outer {
                $outer {
                    inner: std::sync::Arc::new(inner),
                }
            }
        }

        impl std::ops::Deref for $outer {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                self.inner.as_ref()
            }
        }
    };
}
