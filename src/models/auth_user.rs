
use serde::{Deserialize, Serialize};
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct  AuthUser{
  pub id: uuid::Uuid,
  pub email: Option<String>,
  pub username:String ,
  #[serde(skip_serializing)]
  pub password: Option<String>,

  pub provider:String,
  pub external_id:Option<String>,
  pub created_at:Option< chrono::NaiveDateTime>,
}
#[derive(Serialize,Deserialize,Debug,validator::Validate)]
pub struct  UpdateAuthUser{
  pub email: Option<String>,
  pub username:Option<String> ,
}