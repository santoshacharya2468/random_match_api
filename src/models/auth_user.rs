
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::models::schema::auth_users;
#[derive(Queryable, Selectable,Deserialize,Serialize,Debug,Clone)]
#[diesel(table_name =  auth_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
#[derive(AsChangeset)]
#[diesel(table_name = auth_users)]
#[derive(Serialize,Deserialize,Debug,validator::Validate)]
pub struct  UpdateAuthUser{
  pub email: Option<String>,
  pub username:Option<String> ,
}