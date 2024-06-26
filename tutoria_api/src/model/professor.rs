use crate::manager::TutorIAManager;

use super::{
    base::{self, DbBmc},
    Error, Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;
use uuid::Uuid;

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

    pub async fn create(tutoria_manager: &TutorIAManager, professor_c: ProfessorForCreate) -> Result<Uuid> {
        base::create::<Self, _>(tutoria_manager, professor_c).await
    }

    pub async fn find_by_user_id(tutoria_manager: &TutorIAManager, user_id: Uuid) -> Result<Option<Professor>> {
        let db = tutoria_manager.db();

        let professor = sqlx::query_as!(
            Professor,
            "SELECT 
                professor.created_at as created_at,
                professor.id as id,
                professor.nome as nome,
                usuario.username as username
            FROM usuario 
            INNER JOIN professor ON usuario.id = professor.usuario_id
            WHERE usuario.id = $1",
            user_id
        )
        .fetch_optional(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(professor)
    }
}
