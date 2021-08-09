
#[derive(serde::Deserialize, Clone)]
pub struct Banner {
  pub time: ::toml::value::Datetime,
  pub slugs: Vec<String>,
}