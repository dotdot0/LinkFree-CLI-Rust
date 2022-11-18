use serde_json;

use crate::linkfree_struct::LinkFree;

pub fn create_user(user: LinkFree) -> String{
  let user_json = serde_json::to_string(&user).unwrap();
  user_json
}