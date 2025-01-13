
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
#[derive(Deserialize,Serialize,Debug,FromRow)]
pub struct RandomMatch {
 pub id:i32,
 pub created_at:Option<chrono::NaiveDateTime>,
 pub updated_at:Option<chrono::NaiveDateTime>,
 pub user_id:Uuid,
 pub matched_user_id:Option<Uuid>,

}
pub struct NewRandomMatch {
 pub user_id:Uuid,
}