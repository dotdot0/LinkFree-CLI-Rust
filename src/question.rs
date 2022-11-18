use std::{collections::HashMap, fmt::format};

use inquire::{Text, Confirm, Select, DateSelect};
use reqwest::{Client, get};
use serde::{Serialize, Deserialize};
use user_error::{UserFacingError, UFE};
use crate::{check_user::check_user, linkfree_struct::{Links, MileStones, Socials, LinkFree}};

pub async fn data() -> (LinkFree, String){
  let mut linkfree = LinkFree::new();

  let username = Text::new("What is yout Github username?").prompt().unwrap();
 
  let user_status = check_user(&username).await;
 
  if user_status{ 
    let name = Text::new("What is your name (e.g. Jone Doe)?").prompt().unwrap();
    
    let display_stats = Confirm::new("Do you want to display your stats publicly (e.g. Profile views)?").prompt().unwrap(); 
    
    let account_type = Select::new("Your profile type?", vec!["Personal", "Community"]).prompt().unwrap().to_owned();
    
    let bio = Text::new("Add a short bio about yourself").prompt().unwrap();
    
    let tags: Vec<String> = Text::new("Add your tags seperated by ','(e.g. rust, javascript, python):").prompt().unwrap().split(",").map(|t|{
      t.to_owned()
    }).collect();
    
    let add_link = Confirm::new("Do you want to add a link?").prompt().unwrap(); 
    
    let links = add_links(add_link).await;
    
    let add_milestone = Confirm::new("Do you want to add a milestone?").prompt().unwrap();
    
    let milestones = add_milestones(add_milestone).await;

    let add_social = Confirm::new("Do you want to add socials?").prompt().unwrap();

    let socials = add_socials(add_social).await;
    
    linkfree = LinkFree{
      name,
      account_type,
      display_stats,
      bio,
      avatar: format!("https://github.com/{username}.png"),
      tags,
      socials,
      links,
      milestones
    };
  }
  else{
    UserFacingError::new(format!("User not found with the username: {username}"))
        .reason("The username you provided is not valid github username")
        .help("Try again with a valid username")
        .print();
  }
  (linkfree, username)
}

pub async fn get_icons() -> Vec<String>{
  let ignore_value = vec!["{".to_owned(), "}".to_owned(), "".to_owned()];
  let json_data = Client::new()
      .get("https://raw.githubusercontent.com/EddieHubCommunity/LinkFree/main/config/icons.json")
      .send()
      .await
      .unwrap()
      .text()
      .await
      .unwrap();
  let mut icons_hash:HashMap<String, String> = HashMap::new();
  let icons_data: Vec<String> = json_data.split("\n").map(|j|{
    j.to_owned()
  }).collect();
  for icon in icons_data{
    if ignore_value.contains(&icon){
      continue;
    }else{
    let i: Vec<String> = icon.split(":").map(|s| {
      s.to_owned()
    }).collect();
    icons_hash.insert(i.get(0).unwrap().to_owned(), i.get(1).unwrap().to_owned());
  }
}
  let mut icons: Vec<String> = icons_hash.into_keys().collect();
  icons.sort();

  icons
}

async fn add_links(add_link: bool) -> Vec<Links> {
  let mut link: Links = Links::new();

  let mut links: Vec<Links> = Vec::new();

  while(add_link){
    let name = Text::new("What is the name of the link?").prompt().unwrap();

    let url = Text::new("What is the url of the link?").prompt().unwrap();

    let icon = Select::new("Choose an icon", get_icons().await).prompt().unwrap();

    let add_link_fur = Confirm::new("Do you want to add another link?").prompt().unwrap();

    links.push(
      Links{
        name,
        url,
        icon
      }
    );
    if !add_link_fur{
      break;
    }
  }
  links
}

async fn add_socials(add_social: bool) -> Vec<Socials>{
  let mut socials: Vec<Socials> = Vec::new();

  while (add_social) {
      let platform: String = Select::new("Select the platform", get_icons().await).prompt().unwrap();
  
      let url: String = Text::new("What is the url for the social?").prompt().unwrap();
  
      let add_ano_social = Confirm::new("Do you want to add another social?").prompt().unwrap();
  
      socials.push(
        Socials{
          platform,
          url
        }
      );
  
      if !add_ano_social{
        break;
      }
  }
  socials
}

async fn add_milestones(add_milestone: bool) -> Vec<MileStones>{
  let mut milestones: Vec<MileStones> = Vec::new();
 
  while(add_milestone){
    let title = Text::new("What is the title of the milestone?").prompt().unwrap();
 
    let date = DateSelect::new("Date started?").prompt().unwrap().to_string();
 
    let icon = Select::new("Choose an icon", get_icons().await).prompt().unwrap();
 
    let color = Text::new("Add a color:").prompt().unwrap();
 
    let description = Text::new("Add a description for the milestone:").prompt().unwrap();
 
    let url = Text::new("Add a link for the milestone:").prompt().unwrap();
 
    let add_anot_milestone = Confirm::new("Do you want to add another milestone?").prompt().unwrap();
 
    milestones.push(
      MileStones{
        title,
        date,
        icon,
        color,
        description,
        url
      }
    );
 
    if !add_anot_milestone{
      break;
    }
  }
  milestones
}