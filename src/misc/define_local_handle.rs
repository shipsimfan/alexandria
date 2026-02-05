/// Defines a handle that references another type
///
/// Internally, this will produce a type that uses an `Rc`
#[macro_export]
macro_rules! define_local_handle {
    (
        $(#[$meta: meta])*
        $vis: vis $outer: ident -> $inner: ty
    ) => {
        $(#[$meta])*
        #[derive(Clone)]
        $vis struct $outer {
            /// The handle to the underlying item
            inner: std::rc::Rc<$inner>,
        }

        impl $outer {
            #[doc = std::concat!("Create a new [`", std::stringify!($outer), "`] from an [`", std::stringify!($inner), "`]")]
            #[allow(unused)]
            pub(crate) fn from_inner(inner: $inner) -> $outer {
                $outer {
                    inner: std::rc::Rc::new(inner),
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
