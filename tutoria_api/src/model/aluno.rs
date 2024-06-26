use crate::{manager::TutorIAManager, model::Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::{prelude::Type, FromRow};
use uuid::Uuid;

use super::{
    base::{self, DbBmc},
    Error,
};

#[derive(FromRow, Serialize, Type)]
#[sqlx(type_name = "aluno")]
pub struct Aluno {
    pub created_at: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub nome: String,
}

#[derive(FromRow, Serialize)]
pub struct AlunoWithUser {
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

    pub async fn create(tutoria_manager: &TutorIAManager, aluno_c: AlunoForCreate) -> Result<Uuid> {
        base::create::<Self, _>(tutoria_manager, aluno_c).await
    }

    pub async fn search_with_join_user(
        tutoria_manager: &TutorIAManager,
    ) -> Result<Vec<AlunoWithUser>> {
        let db = tutoria_manager.db();

        let alunos = sqlx::query_as!(
            AlunoWithUser,
            r#"
            SELECT
                aluno.created_at as created_at,
                aluno.id as id,
                aluno.nome as nome,
                usuario.username as username
            FROM aluno INNER JOIN usuario ON aluno.usuario_id = usuario.id
            "#
        )
        .fetch_all(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(alunos)
    }

    pub async fn find_by_user_Id(
        tutoria_manager: &TutorIAManager,
        user_id: Uuid,
    ) -> Result<Option<AlunoWithUser>> {
        let db = tutoria_manager.db();

        let aluno = sqlx::query_as!(
            AlunoWithUser,
            r#"
            SELECT
                aluno.created_at as created_at,
                aluno.id as id,
                aluno.nome as nome,
                usuario.username as username
            FROM aluno INNER JOIN usuario ON aluno.usuario_id = usuario.id
            WHERE usuario.id = $1
            "#,
            user_id
        )
        .fetch_optional(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(aluno)
    }
}
