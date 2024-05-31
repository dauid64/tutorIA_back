use crate::{model::Result, utils::time::format_time_for_br_format};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;
use uuid::Uuid;

use super::{
    base::{self, DbBmc},
    Error, ModelManager,
};

#[derive(FromRow, Serialize)]
pub struct Aluno {
    pub created_at: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub username: String,
    pub nome: String,
}

#[derive(Deserialize, Fields)]
pub struct AlunoForCreate {
    pub nome: String,
    pub usuario_id: Uuid,
}

impl AlunoForCreate {
    fn validate(&self) -> Result<()> {
        let nome = self.nome.trim();

        if nome.is_empty() {
            return Err(Error::ValidateFail("Nome não pode estar branco"));
        }

        Ok(())
    }
}

pub struct AlunoBmc;

impl DbBmc for AlunoBmc {
    const TABLE: &'static str = "aluno";
}

impl AlunoBmc {
    pub async fn validate(aluno_c: &AlunoForCreate) -> Result<()> {
        aluno_c.validate()
    }

    pub async fn create(mm: &ModelManager, aluno_c: AlunoForCreate) -> Result<Uuid> {
        base::create::<Self, _>(mm, aluno_c).await
    }

    pub async fn search_with_join_user(mm: &ModelManager) -> Result<Vec<Aluno>> {
        let db = mm.db();

        let alunos = sqlx::query_as!(
            Aluno,
            "
            SELECT
                aluno.created_at as created_at,
                aluno.id as id,
                aluno.nome as nome,
                usuario.username as username
            FROM aluno INNER JOIN usuario ON aluno.usuario_id = usuario.id
            "
        )
        .fetch_all(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(alunos)
    }
}
