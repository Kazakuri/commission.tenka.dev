use std::{io::Write, path::PathBuf};

use futures::StreamExt;

use commission::{log, models::Output};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  log::init();
  
  let mut commissions = commission::commissions::get().await;

  while let Some(commission) = commissions.next().await {
    match commission.await {
      Ok(commission) => {
        let body_english = commission.request.body_english.clone();
        let body_japanese = commission.request.body_japanese.clone();
        let mut output_folder: PathBuf = commission.original_path.clone().into();
        
        let output = Output::from(&commission);
        
        output_folder.pop();
        output_folder.push(&output.slug);
        let output_folder = output_folder.to_string_lossy().replace("./commissions", "./content/artwork");

        let output_str = toml::to_string_pretty(&output)?;

        std::fs::create_dir_all(&output_folder)?;

        let markdown = format!("+++\n{}+++\n\n{{% request() %}}\n{}{{% end %}}\n", output_str, body_english);
        let file: PathBuf = [ &output_folder, "index.md" ].iter().collect();
        let mut file = std::fs::File::create(file)?;
        file.write_all(markdown.as_bytes())?;

        let markdown = format!("+++\n{}+++\n\n{{% request() %}}\n{}{{% end %}}\n", output_str, body_japanese);
        let file: PathBuf = [ &output_folder, "index.ja.md" ].iter().collect();
        let mut file = std::fs::File::create(file)?;
        file.write_all(markdown.as_bytes())?;

        let options = fs_extra::dir::CopyOptions::new();
        fs_extra::copy_items(&commission.assets.images, &output_folder, &options)?;

        let first_image = match commission.assets.images.iter().next() {
          Some(i) => i.clone(),
          None => "res/unknown.png".to_owned(),
        };

        match &commission.assets.banner {
          Some(banner) => {
            fs_extra::copy_items(&vec! [ &banner ], &output_folder, &options)?;
          },
          None => {
            let blur = if commission.request.nsfw { 15.0 } else { 0.0 };
            commission::image::resize_to_fill(&first_image, 1200, 628)?
              .blur(blur)
              .save(format!("{}/banner.jpg", &output_folder))?;
          },
        };

        match &commission.assets.cover {
          Some(cover) => {
            fs_extra::copy_items(&vec! [ &cover ] , &output_folder, &options)?;
          },
          None => {
            commission::image::resize_to_fill(&first_image, 512, 512)?
              .save(format!("{}/cover.jpg", &output_folder))?;
          },
        };

      },
      Err(e) => log::error!("{}", e),
    }
  }
  
  Ok(())
}
