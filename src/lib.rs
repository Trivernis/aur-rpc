//! # aur-rpc
//! This crate offers abstractions over the rpcs provided by the
//! Arch Linux User Repository (AUR).
//!
//! ## Retrieving user information
//! ```rust
//! #[tokio::main]
//! pub async fn main() {
//!     let packages = aur_rpc::search("yay").await.unwrap();
//!
//!     for package in packages {
//!         println!("{} - {}", package.name, package.maintainer);
//!     }
//!     
//!     let mut infos = aur_rpc::info(["mediarepo"]).await.unwrap();
//!     let info = infos.pop().expect("package not found");
//!     println!("{}", info.metadata.popularity);
//! }
//! ```
pub(crate) mod models;
mod rpcs;
pub use models::*;
pub use rpcs::*;
pub mod error;
#[cfg(test)]
pub mod tests;
