use lazy_static::lazy_static;
use regex::Regex;

#[derive(serde::Serialize, Clone)]
pub struct BannerOutput {
  pub date: toml::value::Datetime,
  pub title: String,
  pub slug: String,
  pub extra: BannerExtra,
}

#[derive(serde::Serialize, Clone)]
pub struct BannerExtra {
  pub slugs: Vec<String>,
}

lazy_static! {
  static ref TIME_REGEX: Regex = Regex::new("T.*?Z$").unwrap();
}

impl From<&crate::models::Banner> for BannerOutput {
  fn from(banner: &crate::models::Banner) -> Self {
    let timestamp = banner.time.to_string().replace(":", "-");
    let timestamp = TIME_REGEX.replace(&timestamp, "");

    BannerOutput {
      date: banner.time.clone(),
      title: format!("{}", timestamp),
      slug: format!("{}", timestamp),
      extra: BannerExtra {
        slugs: banner.slugs.clone()
      }
    }
  }
}