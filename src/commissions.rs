use async_walkdir::{DirEntry, Filtering, WalkDir};
use futures::{Stream, StreamExt};
use std::future::Future;

use crate::models::Commission;

pub async fn get() -> impl Stream<Item = impl Future<Output = anyhow::Result<crate::models::Commission>>> {
  WalkDir::new("./commissions")
    .filter(is_commission)
    .map(|e| async move { Commission::new(e?.path()).await })
}

async fn is_commission(entry: DirEntry) -> Filtering {
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
