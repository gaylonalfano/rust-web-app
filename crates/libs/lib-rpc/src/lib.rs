// region:    --- Modules

mod error;
mod params;
mod resources;
mod response;
mod rpcs;

pub mod router;

pub use self::error::{Error, Result};
pub use params::*;
pub use resources::RpcResources;
pub use router::RpcRequest;

pub use rpcs::*;

// endregion: --- Modules
