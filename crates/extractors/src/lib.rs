#![no_std]

mod gogocdn;
mod vidstreaming;
mod rapidcloud;

pub use gogocdn::GogoCDN;
pub use vidstreaming::VidStreaming;
pub use rapidcloud::RapidCloud;