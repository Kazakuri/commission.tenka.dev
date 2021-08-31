use std::{io::Write, path::PathBuf};

use futures::StreamExt;

use commission::log;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  log::init();
  
  let mut commissions = commission::commissions::get().await;

  let mut amount = 0;
  let mut boost = 0;

  while let Some(commission) = commissions.next().await {
    match commission.await {
      Ok(commission) => {
        amount += commission.payment.amount;
        boost += commission.payment.boost;
      },
      Err(e) => log::error!("{}", e),
    }
  }

  amount = (amount as f64 / 100000_f64).floor() as u64 * 100000;
  boost = (boost as f64 / 100000_f64).floor() as u64 * 100000;

  let output_folder = "content/stats/";
  
  std::fs::create_dir_all(&output_folder)?;

  let extra = format!("[extra]\namount = {}\nboost = {}", amount, boost);

  let markdown_en = format!("+++\ntitle = \"Stats\"\ntemplate = \"stats.html\"\n\n{}\n+++\n\n", extra);

  let file: PathBuf = [ &output_folder, "_index.md" ].iter().collect();
  let mut file = std::fs::File::create(file)?;
  file.write_all(markdown_en.as_bytes())?;

  let markdown_ja = format!("+++\ntitle = \"スタッツ\"\ntemplate = \"stats.html\"\n\n{}\n+++\n\n", extra);
  let file: PathBuf = [ &output_folder, "_index.ja.md" ].iter().collect();
  let mut file = std::fs::File::create(file)?;
  file.write_all(markdown_ja.as_bytes())?;
  
  Ok(())
}
