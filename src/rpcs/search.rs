use crate::{error::RPCResult, models::PackageMetadata};

/// Searches for packages by name
pub async fn search<S: AsRef<str>>(keywords: S) -> RPCResult<Vec<PackageMetadata>> {
    search_by(SearchField::Name, keywords).await
}

/// Searches for packages by a specific field
pub async fn search_by<S: AsRef<str>>(
    field: SearchField,
    keywords: S,
) -> RPCResult<Vec<PackageMetadata>> {
    let args: Vec<(&str, &str)> = vec![
        ("v", "5"),
        ("type", "search"),
        ("by", field.to_string()),
        ("arg", keywords.as_ref()),
    ];
    let response = super::call_aur::<_, PackageMetadata>(&args).await?;

    Ok(response.results)
}

/// Represents a field to search by
#[derive(Debug, Clone, Copy)]
pub enum SearchField {
    /// Searches by name
    Name,
    /// Searches name and description
    NameDesc,
    /// Searches by package maintainer
    Maintainer,
    /// Searches for packages that depend on the given keywords
    Depends,
    /// Searches for packages that require the given keywords to be build
    MakeDepends,
    /// Searches for packages that optionally depend on the given keywords
    OptDepends,
    /// Searches for packages that require the given keywords to be present
    CheckDepends,
}

impl SearchField {
    pub(crate) fn to_string(&self) -> &'static str {
        match self {
            SearchField::Name => "name",
            SearchField::NameDesc => "name-desc",
            SearchField::Maintainer => "maintainer",
            SearchField::Depends => "depends",
            SearchField::MakeDepends => "makedepends",
            SearchField::OptDepends => "optdepends",
            SearchField::CheckDepends => "checkdepends",
        }
    }
}
