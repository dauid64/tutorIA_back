use super::Error;
use crate::model::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{
    base::{self, DbBmc},
    ModelManager,
};

#[derive(FromRow, Serialize)]
pub struct Materia {
    pub created_at: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub nome: String,
    pub descricao: String,
    pub professor_nome: String,
    pub conteudos: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct MateriaForCreate {
    pub nome: String,
    pub descricao: String,
    pub professor_id: Uuid,
    pub conteudos: Vec<String>,
}

impl MateriaForCreate {
    fn validate(&self) -> Result<()> {
        let nome = self.nome.trim();
        let descricao = self.descricao.trim();
        let professor_id = self.professor_id;
        let conteudos = &self.conteudos;

        if nome.is_empty() {
            return Err(Error::ValidateFail("Nome da matéria em branco"));
        }

        if descricao.is_empty() {
            return Err(Error::ValidateFail("Descrição da matéria em branco"));
        }

        if professor_id.is_nil() {
            return Err(Error::ValidateFail("Nenhum professor selecionado"));
        }

        if conteudos.is_empty() {
            return Err(Error::ValidateFail("Nenhum conteúdo selecionado"));
        }

        Ok(())
    }
}

pub struct MateriaBmc;

impl DbBmc for MateriaBmc {
    const TABLE: &'static str = "materia";
}

impl MateriaBmc {
    pub async fn validate(materia_c: &MateriaForCreate) -> Result<()> {
        materia_c.validate()
    }

    pub async fn create(mm: &ModelManager, materia_c: MateriaForCreate) -> Result<Uuid> {
        let db = mm.db();

        let query = sqlx::query!(
            r#"
                INSERT INTO materia (nome, descricao, professor_id, conteudos)
                VALUES ($1, $2, $3, $4)
                RETURNING id
            "#,
            materia_c.nome,
            materia_c.descricao,
            materia_c.professor_id,
            &materia_c.conteudos
        )
        .fetch_one(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(query.id)
    }
}
