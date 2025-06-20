use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Org {
    pub id: String,
    pub name: String,
}
