pub mod metric;
pub mod network;

pub mod prelude {
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
}
