use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

impl LinkFree{
  pub fn new() -> Self{
    Self{
      name: String::new(),
      account_type: String::new(),
      display_stats: false,
      bio: String::new(),
      avatar: String::new(),
      tags: Vec::new(),
      socials: Vec::new(),
      links: Vec::new(),
      milestones: Vec::new()
    }
  }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Socials{
  pub platform: String,
  pub url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links{
  pub name: String,
  pub url: String,
  pub icon: String
}

impl Links{
  pub fn new() -> Self{
    Self{
      name: String::new(),
      url: String::new(),
      icon: String::new()
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MileStones{
  pub title: String,
  pub date: String,
  pub icon: String,
  pub color: String,
  pub description: String,
  pub url: String
}