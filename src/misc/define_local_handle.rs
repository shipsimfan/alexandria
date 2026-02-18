/// Defines a handle that references another type
///
/// Internally, this will produce a type that uses an `Rc`
#[macro_export]
macro_rules! define_local_handle {
    (
        $(#[$meta: meta])*
        $vis: vis $outer: ident$(<$generic_arg_name: ident$(: $generic_arg_type: ident)*$( = $generic_arg_default: ty)*>)* -> $inner: ty
    ) => {
        $(#[$meta])*
        $vis struct $outer$(<$generic_arg_name$(: $generic_arg_type)*$( = $generic_arg_default)*>)* {
            /// The handle to the underlying item
            inner: std::rc::Rc<$inner>,
        }

        impl$(<$generic_arg_name$(: $generic_arg_type)*>)* $outer$(<$generic_arg_name>)* {
            #[doc = std::concat!("Create a new [`", std::stringify!($outer), "`] from an [`", std::stringify!($inner), "`]")]
            #[allow(unused)]
            fn from_inner(inner: $inner) -> $outer$(<$generic_arg_name>)* {
                $outer {
                    inner: std::rc::Rc::new(inner),
                }
            }
        }

        impl$(<$generic_arg_name$(: $generic_arg_type)*>)* Clone for $outer$(<$generic_arg_name>)* {
            fn clone(&self) -> $outer$(<$generic_arg_name>)* {
                $outer {
                    inner: self.inner.clone(),
                }
            }
        }
    };
}
