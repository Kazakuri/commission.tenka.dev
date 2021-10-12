use std::io::Read;
use async_walkdir::WalkDir;
use futures::StreamExt;
use std::path::PathBuf;

mod commission;
pub use commission::Commission;

mod output;
pub use output::Output;

mod banner;
pub use banner::Banner;

mod banner_output;
pub use banner_output::BannerOutput;

impl Commission {
  pub async fn new(path: std::path::PathBuf) -> anyhow::Result<Self> {
    let mut file = std::fs::File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut commission: Self = toml::from_str(&content)?;
    commission.load_assets(&path).await?;
    commission.original_path = path.to_string_lossy().into();

    Ok(commission)
  }

  async fn load_assets(&mut self, path: &std::path::PathBuf) -> anyhow::Result<()> {
    let mut asset_path: std::path::PathBuf = path
      .to_string_lossy()
      .replace("./commissions", "./assets")
      .into();

    asset_path.set_extension("");

    let mut walker = WalkDir::new(&asset_path);
    while let Some(file) = walker.next().await {
      let file = file?;
      let path = file.path().to_string_lossy().into();

      match file.file_name().to_str() {
        Some("cover.jpg") => self.assets.cover = Some(path),
        Some("banner.jpg") => self.assets.banner = Some(path),
        Some(_) => self.assets.images.push(path),
        None => ()
      }
    }
    self.assets.images.sort();

    Ok(())
  }
}

impl Banner {
  pub async fn new(path: std::path::PathBuf) -> anyhow::Result<Self> {
    let mut file = std::fs::File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut banner: Self = toml::from_str(&content)?;

    banner.slugs = banner.slugs.iter_mut().map(|s| {
      let path = format!("commissions/{}.toml", s);
      println!("{}", path);
      let mut file = std::fs::File::open(&path).unwrap();

      let mut content = String::new();
      file.read_to_string(&mut content).unwrap();

      let commission: Commission = toml::from_str(&content).unwrap();
      let output = Output::from(&commission);
      let mut path: PathBuf = path.into();
      
      path.pop();
      path.push(&output.slug);

      path
        .to_string_lossy()
        .replace("commissions", "artwork")
    }).collect();

    Ok(banner)
  }
}