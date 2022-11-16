use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LinkFree{
  pub name: String,
  pub account_type: String,
  pub display_stats: bool,
  pub bio: String,
  pub avatar: String,
  pub tags: Vec<String>,
  pub socials: Vec<Socials>,
  pub links: Vec<Links>,
  pub milestones: Vec<MileStones>
}

#[derive(Debug,Serialize)]
pub struct Socials{
  pub platform: String,
  pub url: String
}

#[derive(Debug, Serialize)]
pub struct Links{
  pub name: String,
  pub url: String,
  pub icon: String
}

#[derive(Debug, Serialize)]
pub struct MileStones{
  pub title: String,
  pub date: String,
  pub icon: String,
  pub color: String,
  pub description: String,
  pub url: String
}