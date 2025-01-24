use std::env;
use crate::{models::random_match::{NewRandomMatch, RandomMatch}, utils::app_error::AppError, AppState};
use livekit_api::access_token;
use uuid::Uuid;
#[derive(Clone)]
pub struct  RandomMatchService{
   pub  app_state:AppState
}
impl  RandomMatchService {
    pub async fn get_status(&self,u_id:Uuid)->Result<Option<RandomMatch>,AppError>{
        let conn=self.app_state.db_pool();
        let result=  sqlx::query_as!(RandomMatch,"select * from random_matches where user_id=$1",u_id).fetch_optional(&conn).await;
        match result{
            Ok(user)=>Ok(user),  
            Err(e)=>Err(AppError::new(e.to_string()))
        }
    }

    pub async  fn create_random_match(&self,user:Uuid)->Result<RandomMatch,AppError>{
        let previous_match=self.get_status(user).await;
       match  previous_match{
           Ok(random_match)=>{
               if random_match.is_some(){
                 let sender=self.app_state.broadcaster.as_ref();
                 sender.send(random_match.clone().unwrap()).unwrap();
                   return Ok(random_match.unwrap());
               }
           },
           Err(_)=>{}
           
       }
        let randm_match=NewRandomMatch{
            user_id :user,
        };
        let db_con= self.app_state.db_pool();
        let result=  sqlx::query_as!(RandomMatch,"insert into  random_matches(user_id) values($1) returning *",randm_match.user_id).fetch_one(&db_con).await;
        match result{
            Ok(user)=>Ok(user),
            Err(e)=>Err(AppError::new(e.to_string()))
        }
    }
    pub async  fn exit_match(&self,user:Uuid)->Result<(),AppError>{
        let db_con= self.app_state.db_pool();
        let result=sqlx::query("delete from random_matches where user_id=$1").bind(user).execute(&db_con).await;
        match result{
            Ok(_)=>Ok(()),
            Err(e)=>Err(AppError::new(e.to_string()))
        }
    }
  pub async  fn create_token() -> Result<String, access_token::AccessTokenError> {
        let api_key = env::var("LIVEKIT_API_KEY").expect("LIVEKIT_API_KEY is not set");
        let api_secret = env::var("LIVEKIT_API_SECRET").expect("LIVEKIT_API_SECRET is not set");
        let token = access_token::AccessToken::with_api_key(&api_key, &api_secret)
           .with_identity("identity")
           .with_name("name")
           .with_grants(access_token::VideoGrants {
              room_join: true,
              room: "my-room".to_string(),
              ..Default::default()
           })
           .to_jwt();
        return token
     }

}