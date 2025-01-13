use serde::{Serialize, Deserialize};
#[derive(Debug,Serialize,Deserialize)]
pub struct GoogleUser {
    pub sub: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email: String,
}


#[derive(Debug,Serialize,Deserialize)]
pub struct FacebookUser {
    pub sub: String,
    pub name:Option< String>,
    pub picture: Option<String>,
    pub email: Option<String>,
}


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SocialUser {
    pub identifier: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub username:String,
    pub provider:String
}

impl  SocialUser {
    pub fn from_google(user:GoogleUser)->SocialUser{
        SocialUser{
            identifier:user.sub,
            name:user.name,
            picture:user.picture,
            email:Some(user.email.clone()),
            username:user.email,
            provider:"Google".to_string()
        }
    }
    pub fn from_facebook(user:FacebookUser)->SocialUser{
        SocialUser{
            identifier:user.sub,
            name:user.name,
            picture:user.picture,
            email:user.email.clone(),
            username:user.email.unwrap(),
            provider:"Facebook".to_string()
        }
    }
    
}