pub use prost;

include!("../include/_.rs");

pub mod proto {

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/include/mod.rs"));
}
