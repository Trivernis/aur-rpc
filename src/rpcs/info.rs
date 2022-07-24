use crate::{error::RPCResult, models::PackageInfo};

/// Returns information about the given list of packages
pub async fn info<I: IntoIterator<Item = S>, S: AsRef<str>>(
    packages: I,
) -> RPCResult<Vec<PackageInfo>> {
    let mut args: Vec<(&str, String)> = packages
        .into_iter()
        .map(|p| ("arg[]", p.as_ref().to_string()))
        .collect();
    args.push(("v", String::from("5")));
    args.push(("type", String::from("info")));
    let response = super::call_aur::<_, PackageInfo>(&args).await?;

    Ok(response.results)
}
