use serde::Serialize;


#[derive(Debug,Serialize,serde::Deserialize)]
pub struct  AuthTokens{
    pub access_token: String,
    pub refresh_token: String
}