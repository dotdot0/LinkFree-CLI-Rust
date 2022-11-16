use inquire::Text;

pub fn data() -> String{
  let username = Text::new("What is yout Github username?").prompt().unwrap();
  username
}