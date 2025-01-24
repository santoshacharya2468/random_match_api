use std::env;
use crate::{dtos::{ auth_tokens::AuthTokens, social_login_requet::{LoginProvider, SocialLoginRequest}, social_user::{FacebookUser, GoogleUser, SocialUser}}, 
models::auth_user::AuthUser, utils::app_error::AppError, AppState};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
struct UserClaims {
    identifier: Uuid,
    exp: usize,
}

#[derive(Clone)]
pub struct  AuthService{
    pub app_state:AppState
}

impl  AuthService {
    pub async fn social_login(&self, request: SocialLoginRequest)->Result<AuthTokens,AppError> {
    let result: Result<SocialUser, AppError>=  match  request.provider{
          LoginProvider::Google=>self.get_google_user(request.token.clone()).await.map(SocialUser::from_google),
          LoginProvider::Facebook=>self.get_facebook_user(request.token).await.map(SocialUser::from_facebook)
      };
     
        match result{
            Ok(user)=>{
               let auth_user=self.get_or_create_auth_user(user.clone()).await;
               match auth_user{
                   Ok(user)=>{
                       self.create_auth_tokens(user).await
                   },
                   Err(e)=>Err(e)
               }
              }
            Err(e)=>Err(e)
        }
    }

    async fn get_google_user(&self,token:String)->Result<GoogleUser,AppError>{
      let oauth_url=env::var("GOOGLE_OAUTH_URL").unwrap();
      let result=reqwest::get(oauth_url+"?access_token="+&token+"").await;
      match result{
          Ok(response)=>{
              match response.json::<GoogleUser>().await{
                  Ok(user)=>Ok(user),
                  Err(e)=>Err(AppError::new(e.to_string()))
              }
          },
          Err(e)=>Err(AppError::new(e.to_string()))
          
      }
     
    }
    async  fn get_facebook_user(&self,token:String)->Result<FacebookUser,AppError>{
        let oauth_url=env::var("FACEBOOK_OAUTH_URL").unwrap();
        let result=reqwest::get(oauth_url+"?access_token="+&token+"").await;
        match result{
            Ok(response)=>{
                match response.json::<FacebookUser>().await{
                    Ok(user)=>Ok(user),
                    Err(e)=>Err(AppError::new(e.to_string()))
                }
            },
            Err(e)=>Err(AppError::new(e.to_string()))
            
        }
    }
    async fn get_or_create_auth_user(&self,social_user:SocialUser)->Result<AuthUser,AppError>{
       let conn=& self.app_state.db_pool();
       let result=sqlx::query_as!(AuthUser,"select * from auth_users where external_id=$1 and provider=$2",social_user.identifier,social_user.provider).fetch_one(conn).await;
       match result{
           Ok(user)=>Ok(user),
           Err(_)=>{
              self.create_auth_user(social_user).await
       
          }
       }
    }
    async  fn create_auth_user(&self,user:SocialUser)->Result<AuthUser,AppError>{
        let conn=& self.app_state.db_pool();
        let result= sqlx::query_as!(AuthUser,"insert into auth_users(username,email,provider,external_id) values($1,$2,$3,$4) returning *",user.username,user.email,user.provider,user.identifier).fetch_one(conn).await;
        match result{
            Ok(user)=>Ok(user),
            Err(e)=>Err(AppError::new(e.to_string()))
        }
    }
    async fn create_auth_tokens(&self,user:AuthUser)->Result<AuthTokens,AppError>{
      let expiry=(chrono::Utc::now()+chrono::Duration::days(1)).timestamp() as usize;
        let claim=&UserClaims{
            identifier:user.id,
            exp:expiry
        };
        let refresh_token_expiry=(chrono::Utc::now()+chrono::Duration::days(30)).timestamp() as usize;
        let expiry_claim=&UserClaims{
            identifier:user.id,
            exp:refresh_token_expiry
        };
        let access_token=jsonwebtoken::encode(&Header::new(Algorithm::HS256), claim, &jsonwebtoken::EncodingKey::from_secret("secret".as_ref())).unwrap();
        let refresh_token=jsonwebtoken::encode(&Header::new(Algorithm::HS256), expiry_claim, &EncodingKey::from_secret("secret".as_ref())).unwrap();
      Ok(AuthTokens{
            access_token,
            refresh_token
        })
    }

   
}
pub async  fn verify_token(token:&str,app_state:AppState)->Result<AuthUser,AppError>{
  let claim=jsonwebtoken::decode::<UserClaims>(&token, &DecodingKey::from_secret("secret".as_ref()), &jsonwebtoken::Validation::default());
  if claim.is_err(){
      return Err(AppError::new("Invalid Token".to_string()));
  }
  let claim=claim.unwrap();
  let conn=& app_state.db_pool();
   let result=  sqlx::query_as!(AuthUser,"select * from auth_users where id=$1",claim.claims.identifier).fetch_one(conn).await;
  match result{
      Ok(user)=>Ok(user),
      Err(e)=>Err(AppError::new(e.to_string()))
  }
}