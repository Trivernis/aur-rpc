use serde::{Deserialize, Serialize};

/// Represents the basic information about a package
/// that can be retrieved from a search or by
/// fetching the package info.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PackageMetadata {
    pub description: String,
    pub first_submitted: u64,
    #[serde(alias = "ID")]
    pub id: u32,
    pub last_modified: u64,
    pub maintainer: String,
    pub name: String,
    pub num_votes: u32,
    pub out_of_date: Option<u64>,
    pub package_base: String,
    #[serde(alias = "PackageBaseID")]
    pub package_base_id: u32,
    pub popularity: f64,
    #[serde(alias = "URL")]
    pub url: Option<String>,
    #[serde(alias = "URLPath")]
    pub url_path: String,
    pub version: String,
}

/// Represents the information about a package that can
/// be retrieved with the info rpc
#[derive(Clone, Debug, Deserialize)]
#[serde(from = "PackageInfoRaw")]
pub struct PackageInfo {
    pub metadata: PackageMetadata,
    pub keywords: Vec<String>,
    pub license: Vec<String>,
    pub make_depends: Vec<String>,
}

impl From<PackageInfoRaw> for PackageInfo {
    fn from(info: PackageInfoRaw) -> Self {
        PackageInfo {
            metadata: PackageMetadata {
                description: info.description,
                first_submitted: info.first_submitted,
                id: info.id,
                last_modified: info.last_modified,
                maintainer: info.maintainer,
                name: info.name,
                num_votes: info.num_votes,
                out_of_date: info.out_of_date,
                package_base: info.package_base,
                package_base_id: info.package_base_id,
                popularity: info.popularity,
                url: info.url,
                url_path: info.url_path,
                version: info.version,
            },
            keywords: info.keywords,
            license: info.license,
            make_depends: info.make_depends,
        }
    }
}

/// Full packge info with all fields as a workaround
/// as serde doesn't support aliases in flattened structs
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PackageInfoRaw {
    pub keywords: Vec<String>,
    pub license: Vec<String>,
    pub make_depends: Vec<String>,
    pub description: String,
    pub first_submitted: u64,
    #[serde(alias = "ID")]
    pub id: u32,
    pub last_modified: u64,
    pub maintainer: String,
    pub name: String,
    pub num_votes: u32,
    pub out_of_date: Option<u64>,
    pub package_base: String,
    #[serde(alias = "PackageBaseID")]
    pub package_base_id: u32,
    pub popularity: f64,
    #[serde(alias = "URL")]
    pub url: Option<String>,
    #[serde(alias = "URLPath")]
    pub url_path: String,
    pub version: String,
}

/// Represents a basic AUR response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct AURResponse<T> {
    pub version: u8,
    #[serde(alias = "type")]
    pub response_type: ResponseType,
    pub resultcount: u32,
    pub results: Vec<T>,
    #[serde(default)]
    pub error: Option<String>,
}

/// Represents the type of aur response
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ResponseType {
    Error,
    Search,
    Info,
    Multiinfo,
}
