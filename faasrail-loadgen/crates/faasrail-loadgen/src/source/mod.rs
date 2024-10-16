pub mod backend;
mod client;
mod error;
mod iat;
mod minuterange;
mod worker;

pub use backend::Backend as SourceBackend;
pub use client::SourceClient;
pub use error::Error;
pub use iat::Equidistant;
pub use iat::IatGenerator;
pub use iat::Poisson;
pub use iat::Uniform;
pub use minuterange::MinuteRange;

#[derive(Debug, ::serde::Deserialize)]
pub struct FunctionRow {
    #[serde(rename = "avg")]
    pavg: f64,
    mapped_wreq: String,
    rpm: Vec<u32>,
}

#[cfg(test)]
mod tests {}
