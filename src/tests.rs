use crate::{info, search, search_by, SearchField};

#[tokio::test]
async fn it_searches_by_name() {
    let packages = search("yay").await.unwrap();
    assert!(packages.len() > 0)
}

#[tokio::test]
async fn it_searches_by_maintainer() {
    let packages = search_by(SearchField::Maintainer, "wcasanova")
        .await
        .unwrap();
    assert!(packages.len() > 0)
}

#[tokio::test]
async fn it_returns_information() {
    let packages = info(["yay"]).await.unwrap();
    assert!(packages.len() > 0)
}

#[tokio::test]
async fn it_returns_information_2() {
    let packages = info(["ros-melodic-desktop"]).await.unwrap();
    assert!(packages.len() > 0)
}

#[tokio::test]
async fn it_returns_information_3() {
    let packages = info(["perl-dist-zilla-plugin-github"]).await.unwrap();
    assert!(packages.len() > 0)
}
