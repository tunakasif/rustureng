pub mod parser;
cfg_if::cfg_if! {
    if #[cfg(feature = "isahc")] {
        pub mod retriever_isahc;
    } else if #[cfg(feature = "reqwest")] {
        pub mod retriever_reqwest;
    } else {
        pub mod retriever_ureq;
    }
}
