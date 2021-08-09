use std::{io::Write, path::PathBuf};

use futures::StreamExt;

use commission::{log, models::BannerOutput};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  log::init();
  
  let mut banners = commission::banners::get().await;

  while let Some(banner) = banners.next().await {
    match banner.await {
      Ok(banner) => {
        let output = BannerOutput::from(&banner);
        let output_folder = format!("content/banners/{}", output.slug);
        let output_str = toml::to_string_pretty(&output)?;
        
        std::fs::create_dir_all(&output_folder)?;

        let markdown = format!("+++\n{}+++\n\n", output_str);

        let file: PathBuf = [ &output_folder, "index.md" ].iter().collect();
        let mut file = std::fs::File::create(file)?;
        file.write_all(markdown.as_bytes())?;

        let file: PathBuf = [ &output_folder, "index.ja.md" ].iter().collect();
        let mut file = std::fs::File::create(file)?;
        file.write_all(markdown.as_bytes())?;

        let banner_path = format!("assets/banners/{}.png", output.slug);

        let options = fs_extra::file::CopyOptions::new();
        let _ = fs_extra::file::copy(&banner_path, format!("{}/banner.png", output_folder), &options);
      },
      Err(e) => log::error!("{}", e),
    }
  }
  
  Ok(())
}
