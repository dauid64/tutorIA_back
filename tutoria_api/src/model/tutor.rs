use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlb::Fields;
use uuid::Uuid;
use crate::model::Result;

use super::{base::{self, DbBmc}, Error, ModelManager};

#[derive(Fields, Deserialize)]
pub struct TutorForCreate {
    pub nome: String,
    #[serde(skip)]
    pub assistant_id: String,
    pub materia_id: Uuid,
}

pub struct TutorBmc;

impl DbBmc for TutorBmc {
    const TABLE: &'static str = "tutor";
}

impl TutorForCreate {
    fn validate(&self) -> Result<()> {
        let nome = self.nome.trim();

        if nome.is_empty() {
            return Err(Error::ValidateFail("Nome não pode estar branco"));
        }

        if self.assistant_id.is_empty() {
            return Err(Error::ValidateFail("assistant_id não pode estar branco"));
        }

        if self.materia_id.is_nil() {
            return Err(Error::ValidateFail("materia_id não pode estar branco"));
        }

        Ok(())
    }
}

impl TutorBmc {
    pub async fn validate(tutor_c: &TutorForCreate) -> Result<()> {
        tutor_c.validate()
    }

    pub async fn create(mm: &ModelManager, tutor_c: TutorForCreate) -> Result<Uuid> {
        base::create::<Self, _>(mm, tutor_c).await
    }
}