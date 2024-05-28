use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;
use uuid::Uuid;
use crate::model::Result;

use super::{base::{self, DbBmc}, Error, ModelManager};


#[derive(Fields, FromRow, Serialize)]
pub struct Aluno {
    pub id: Uuid,
    pub username: String,
    pub nome: String
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
            return Err(Error::ValidateFail("Nome não pode estar branco"))
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
}