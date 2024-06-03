use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;
use uuid::Uuid;
use super::{base::{self, DbBmc}, Error, ModelManager, Result};

#[derive(FromRow, Serialize)]
pub struct Professor {
    pub created_at: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub username: String,
    pub nome: String,
}

#[derive(Deserialize, Fields)]
pub struct ProfessorForCreate {
    pub nome: String,
    pub usuario_id: Uuid,
}

impl ProfessorForCreate {
    fn validate(&self) -> Result<()> {
        let nome = self.nome.trim();

        if nome.is_empty() {
            return Err(Error::ValidateFail("Nome não pode estar branco"));
        }

        Ok(())
    }
}

pub struct ProfessorBmc;

impl DbBmc for ProfessorBmc {
    const TABLE: &'static str = "professor";
}

impl ProfessorBmc {
    pub async fn validate(professor_c: &ProfessorForCreate) -> Result<()> {
        professor_c.validate()
    }

    pub async fn create(mm: &ModelManager, professor_c: ProfessorForCreate) -> Result<Uuid> {
        base::create::<Self, _>(mm, professor_c).await
    }
}