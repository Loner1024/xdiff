pub use config::{DiffConfig, DiffProfile, RequestProfile, ResponseProfile};

pub mod cli;

mod config;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}
