use reqwest::Client;

pub async fn check_user(username: &String) -> bool{
  let client = Client::new()
                .get(format!("https://api.github.com/users/{username}"))
                .header("User-Agent", "rust")
                .send()
                .await
                .unwrap();
  if client.status().is_success(){
    return true;
  }
  else{
    return false;
  }
}