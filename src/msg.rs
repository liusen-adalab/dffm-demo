use serde::{Deserialize, Serialize};

type ID = i32;
#[derive(Serialize, Deserialize)]
pub enum Requst {
    GetUserById(ID),
}
