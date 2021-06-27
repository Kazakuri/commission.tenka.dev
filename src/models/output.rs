
#[derive(serde::Serialize, Clone)]
pub struct Output {
  pub date: toml::value::Datetime,
  pub title: String,
  pub slug: String,
  pub extra: Extra,
}

#[derive(serde::Serialize, Clone)]
pub struct Extra {
  pub accepted_date: toml::value::Datetime,
  pub nsfw: bool,
  pub skeb_handle: Option<String>,
  pub twitter_handle: Option<String>,
  pub pixiv_id: Option<String>,
  pub variations: Vec<String>,
}

impl From<&crate::models::Commission> for Output {
  fn from(commission: &crate::models::Commission) -> Self {
    Output {
      date: commission.request.completed.clone(),
      title: format!("{}", commission.creator.name),
      slug: format!("{}-{}", commission.request.completed.to_string().replace(":", "-"), commission.creator.handle),
      extra: Extra {
        accepted_date: commission.request.approved.clone(),
        nsfw: commission.request.nsfw,
        skeb_handle: commission.creator.skeb.clone(),
        twitter_handle: commission.creator.twitter.clone(),
        pixiv_id: commission.creator.pixiv.map(|i| i.to_string()),
        variations: commission.assets.images
          .iter()
          .map(|i| i
            .rsplit_once("/")
            .unwrap_or_else(|| ("", ""))
            .1
            .into()
          )
          .collect(),
      }
    }
  }
}