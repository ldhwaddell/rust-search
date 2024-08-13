pub mod algorithms;
mod among;
mod snowball_env;

pub use crate::snowball::among::Among;
pub use crate::snowball::snowball_env::SnowballEnv;

pub enum StemmingAlgorithm {
    Porter,
    Porter2,
}

pub mod stem;