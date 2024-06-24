use crate::model::{Error, Result};
use serde::Serialize;
use sqlb::Fields;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use super::{
    base::{self, DbBmc},
    ModelManager,
};

#[derive(Fields)]
pub struct ChatForCreate {
    pub aluno_id: Uuid,
    pub tutor_id: Uuid,
}

#[derive(Serialize, FromRow, Fields)]
pub struct Chat {
    pub id: Uuid,
    pub aluno_id: Uuid,
    pub tutor_id: Uuid,
}

pub struct ChatBmc;

impl DbBmc for ChatBmc {
    const TABLE: &'static str = "chat";
}

impl ChatBmc {
    pub async fn create(mm: &ModelManager, chat_c: ChatForCreate) -> Result<Uuid> {
        base::create::<Self, _>(mm, chat_c).await
    }

    pub async fn find_by_aluno_and_tutor_id(
        mm: &ModelManager,
        aluno_id: Uuid,
        tutor_id: Uuid,
    ) -> Result<Option<Chat>> {
        let db = mm.db();

        let chat = sqlx::query_as!(
            Chat,
            r#"
                SELECT
                    chat.id as id,
                    chat.aluno_id as aluno_id,
                    chat.tutor_id as tutor_id
                FROM chat
                WHERE chat.aluno_id = $1 AND chat.tutor_id = $2
            "#,
            aluno_id,
            tutor_id
        )
        .fetch_optional(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(chat)
    }

    pub async fn find_by_id(mm: &ModelManager, id: Uuid) -> Result<Chat> {
        let chat = base::find_by_id::<Self, Chat>(mm, id).await?;

        if chat.is_none() {
            return Err(Error::EntityNotFound {
                entity: "chat",
                id: id,
            });
        }

        Ok(chat.unwrap())
    }
}
