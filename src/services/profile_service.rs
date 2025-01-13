use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{models::auth_user::{AuthUser, UpdateAuthUser}, utils::app_error::AppError, AppState};
use  crate::models::schema::auth_users::dsl::*;


#[derive(Clone)]
pub struct  ProfileService{
    pub app_state:AppState
}

impl  ProfileService {
  pub  async fn update_profile(&self,user_id:Uuid, payload:UpdateAuthUser)->Result<AuthUser,AppError>{
        let conn=&mut self.app_state.db();
        let result=diesel::update(auth_users.filter(id.eq(user_id))).set(payload).get_result(conn); 
        match result{
            Ok(user)=>Ok(user),
            Err(e)=>Err(AppError::new(e.to_string()))
        }
    }
    
}