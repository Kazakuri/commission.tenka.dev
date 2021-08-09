use async_walkdir::{DirEntry, Filtering, WalkDir};
use futures::{Stream, StreamExt};
use std::future::Future;

use crate::models::Banner;

pub async fn get() -> impl Stream<Item = impl Future<Output = anyhow::Result<crate::models::Banner>>> {
  WalkDir::new("./banners")
    .filter(is_banner)
    .map(|e| async move { Banner::new(e?.path()).await })
}

async fn is_banner(entry: DirEntry) -> Filtering {
  let check = |entry: DirEntry| {
    if entry.path().extension()? == "toml" {
      Some(Filtering::Continue)
    } else {
      None
    }
  };

  match check(entry) {
    Some(f) => f,
    None => Filtering::Ignore,
  }
}
