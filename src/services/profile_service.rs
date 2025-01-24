
use uuid::Uuid;

use crate::{models::auth_user::{AuthUser, UpdateAuthUser}, utils::app_error::AppError, AppState};


#[derive(Clone)]
pub struct  ProfileService{
    pub app_state:AppState
}

impl  ProfileService {
  pub  async fn update_profile(&self,user_id:Uuid, payload:UpdateAuthUser)->Result<AuthUser,AppError>{
        let conn=& self.app_state.db_pool();
        let result=  sqlx::query_as!(AuthUser,"update auth_users set username=$1, email=$2 where id=$3 returning *",payload.username,payload.email,user_id).fetch_one(conn).await;
        match result{
            Ok(user)=>Ok(user),
            Err(e)=>Err(AppError::new(e.to_string()))
        }
    }
    
}