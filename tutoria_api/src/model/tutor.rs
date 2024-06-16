use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Tutor {
    pub created_at: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub nome: String,
}

