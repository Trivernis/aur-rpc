use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
mod info;
mod search;

pub use info::*;
pub use search::*;

use crate::{error::RPCResult, models::AURResponse};
use std::fmt::Debug;

const AUR_URL: &str = "https://aur.archlinux.org/rpc/";

#[tracing::instrument(level = "debug")]
async fn call_aur<Q: Serialize + Debug + ?Sized, R: DeserializeOwned>(
    args: &Q,
) -> RPCResult<AURResponse<R>> {
    let client = Client::new();
    let response = client.get(AUR_URL).query(args).send().await?.json().await?;

    Ok(response)
}
