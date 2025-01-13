

use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]

pub enum LoginProvider{
    Google, 
    Facebook
}
#[derive(Debug,Serialize,Deserialize,validator::Validate)]
pub struct  SocialLoginRequest{
    #[validate(length(min = 10))]
    pub token:String,
    pub provider:LoginProvider
}