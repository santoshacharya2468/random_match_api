use serde::{Deserialize, Serialize};


#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct  PageOptions<T> {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
    pub query: Option<T>
}
fn default_page()->i64{
    1
}
fn default_page_size()->i64{
    10
}
impl<T> PageOptions<T> {
  
   
    pub fn skip(&self) -> i64 {
        (self.page - 1) * self.page_size
    }
}