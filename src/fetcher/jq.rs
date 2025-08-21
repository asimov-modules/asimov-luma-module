// This is free and unencumbered software released into the public domain.

pub use jq::*;

macro_rules! json_filter {
    ($name:ident) => {
        // #[cfg(feature = "std")]
        pub fn $name() -> &'static JsonFilter {
            use std::sync::OnceLock;
            static ONCE: OnceLock<JsonFilter> = OnceLock::new();
            ONCE.get_or_init(|| {
                include_str!(concat!("jq/", stringify!($name), ".jq"))
                    .parse()
                    .unwrap()
            })
        }

        // #[cfg(not(feature = "std"))]
        // pub fn $name() -> JsonFilter {
        //     include_str!(concat!("jq/", stringify!($name), ".jq"))
        //         .parse()
        //         .unwrap()
        // }
    };
}

json_filter!(discover);
json_filter!(category);
json_filter!(place);
