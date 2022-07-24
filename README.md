# aur-rpc

This crate is an async wrapper for calls to the Arch Linx User Repository.

## Example

```rust
#[tokio::main]
pub async fn main() {
   let packages = aur_rpc::search("yay").await.unwrap();

   for package in packages {
       println!("{} - {}", package.name, package.maintainer);
   }
   
   let mut infos = aur_rpc::info(["mediarepo"]).await.unwrap();
   let info = infos.pop().expect("package not found");
   println!("{}", info.metadata.popularity);
}
```

## License

Apache 2.0
