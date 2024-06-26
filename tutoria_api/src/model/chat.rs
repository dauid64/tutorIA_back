use crate::{manager::TutorIAManager, model::{Error, Result}};
use serde::Serialize;
use sqlb::Fields;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use super::base::{self, DbBmc};

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
    pub async fn create(tutoria_manager: &TutorIAManager, chat_c: ChatForCreate) -> Result<Uuid> {
        base::create::<Self, _>(tutoria_manager, chat_c).await
    }

    pub async fn find_by_aluno_and_tutor_id(
        tutoria_manager: &TutorIAManager,
        aluno_id: Uuid,
        tutor_id: Uuid,
    ) -> Result<Option<Chat>> {
        let db = tutoria_manager.db();

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

    pub async fn find_by_id(tutoria_manager: &TutorIAManager, id: Uuid) -> Result<Chat> {
        let chat = base::find_by_id::<Self, Chat>(tutoria_manager, id).await?;

        if chat.is_none() {
            return Err(Error::EntityNotFound {
                entity: "chat",
                id: id,
            });
        }

        Ok(chat.unwrap())
    }
}
