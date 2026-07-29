use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Quote {
    pub id: i64,
    pub text: String,
    pub source: Option<String>,
    pub rating: f64,
    pub likes: i64,
}
