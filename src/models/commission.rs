
#[derive(serde::Deserialize, Clone)]
pub struct Commission {
  pub request: Request,
  pub payment: Payment,
  pub creator: Creator,

  #[serde(skip_deserializing)]
  pub assets: Assets,
  
  #[serde(skip_deserializing)]
  pub original_path: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct Request {
  pub approved: ::toml::value::Datetime,
  pub completed: ::toml::value::Datetime,
  pub nsfw: bool,
  pub body_english: String,
  pub body_japanese: String,
  pub boost_english: String,
  pub boost_japanese: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct Payment {
  pub amount: u64,
  pub boost: u64,
}

#[derive(serde::Deserialize, Clone)]
pub struct Creator {
  pub name: String,
  pub handle: String,
  pub twitter: Option<String>,
  pub skeb: Option<String>,
  pub pixiv: Option<u64>,
}

#[derive(Clone, Default)]
pub struct Assets {
  pub cover: Option<String>,
  pub banner: Option<String>,
  pub images: Vec<String>,
}