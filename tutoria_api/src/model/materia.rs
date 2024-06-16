use crate::model::{Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{aluno::AlunoWithUser, base::DbBmc, ModelManager};

#[derive(FromRow, Serialize)]
pub struct Materia {
    pub created_at: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub nome: String,
    pub descricao: String,
    pub professor_nome: String,
    pub conteudos: Vec<String>,
    pub qtd_alunos: Option<i64>,
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

        let id = query.id;

        Ok(id)
    }

    pub async fn add_aluno(mm: &ModelManager, aluno_id: Uuid, materia_id: Uuid) -> Result<()> {
        let db = mm.db();

        sqlx::query(
            r#"
                INSERT INTO aluno_materia (aluno_id, materia_id)
                VALUES ($1, $2)
            "#,
        )
        .bind(aluno_id)
        .bind(materia_id)
        .execute(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(())
    }

    pub async fn remove_aluno(mm: &ModelManager, aluno_id: Uuid, materia_id: Uuid) -> Result<()> {
        let db = mm.db();

        sqlx::query(
            r#"
                DELETE FROM aluno_materia
                WHERE aluno_id = $1 AND materia_id = $2
            "#,
        )
        .bind(aluno_id)
        .bind(materia_id)
        .execute(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(())
    }

    pub async fn find_alunos_registered(
        mm: &ModelManager,
        materia_id: Uuid,
    ) -> Result<Vec<AlunoWithUser>> {
        let db = mm.db();

        let alunos = sqlx::query_as!(
            AlunoWithUser,
            r#"
            SELECT
                aluno.created_at as created_at,
                aluno.id as id,
                aluno.nome as nome,
                usuario.username as username
            FROM aluno
            INNER JOIN usuario ON aluno.usuario_id = usuario.id
            INNER JOIN aluno_materia ON aluno.id = aluno_materia.aluno_id AND aluno_materia.materia_id = $1
            "#,
            materia_id
        )
        .fetch_all(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(alunos)
    }

    pub async fn find_alunos_not_registered(
        mm: &ModelManager,
        materia_id: Uuid,
    ) -> Result<Vec<AlunoWithUser>> {
        let db = mm.db();

        let alunos = sqlx::query_as!(
            AlunoWithUser,
            r#"
            SELECT
                aluno.created_at as created_at,
                aluno.id as id,
                aluno.nome as nome,
                usuario.username as username
            FROM aluno
            INNER JOIN usuario ON aluno.usuario_id = usuario.id
            LEFT JOIN aluno_materia ON aluno.id = aluno_materia.aluno_id AND aluno_materia.materia_id = $1
            WHERE aluno_materia.materia_id IS NULL
            "#,
            materia_id
        )
        .fetch_all(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(alunos)
    }

    pub async fn find_by_professor_id(
        mm: &ModelManager,
        professor_id: Uuid,
    ) -> Result<Vec<Materia>> {
        let db = mm.db();

        let materias = sqlx::query_as!(
            Materia,
            r#"
                SELECT
                    materia.created_at as created_at,
                    materia.id as id,
                    materia.nome as nome,
                    materia.descricao as descricao,
                    materia.conteudos as conteudos,
                    professor.nome as professor_nome,
                    COUNT(aluno) as qtd_alunos
                FROM materia
                INNER JOIN professor ON materia.professor_id = professor.id
                LEFT JOIN aluno_materia ON materia.id = aluno_materia.materia_id
                LEFT JOIN aluno ON aluno_materia.aluno_id = aluno.id
                WHERE materia.professor_id = $1
                GROUP BY professor.nome, materia.id
            "#,
            professor_id
        )
        .fetch_all(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(materias)
    }
}
