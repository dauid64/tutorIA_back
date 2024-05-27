use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;
use uuid::Uuid;
use crate::model::Result;

use super::{base::{self, DbBmc}, ModelManager};


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

pub struct AlunoBmc;

impl DbBmc for AlunoBmc {
    const TABLE: &'static str = "aluno";
}

impl AlunoBmc {
    pub async fn create(mm: &ModelManager, aluno_c: AlunoForCreate) -> Result<Uuid> {
        base::create::<Self, _>(mm, aluno_c).await
    }
}