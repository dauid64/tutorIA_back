use super::{
    base::{self, DbBmc},
    Error, ModelManager, Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::{postgres::PgRow, FromRow};
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

pub trait ProfessorBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

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

    pub async fn find_by_user_id(mm: &ModelManager, user_id: Uuid) -> Result<Option<Professor>> {
        let db = mm.db();

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
