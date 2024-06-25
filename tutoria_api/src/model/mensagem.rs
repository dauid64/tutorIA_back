use crate::model::{Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;
use uuid::Uuid;

use super::{
    base::{self, DbBmc},
    ModelManager,
};

#[derive(Deserialize, Fields)]
pub struct MensagemForCreate {
    pub conteudo: String,
    pub tipo: String,
    pub chat_id: Uuid,
}

#[derive(FromRow, Serialize, Debug)]
pub struct Mensagem {
    pub created_at: Option<DateTime<Utc>>,
    pub conteudo: String,
    pub tipo: String,
}

pub struct MensagemBmc;

impl DbBmc for MensagemBmc {
    const TABLE: &'static str = "mensagem";
}

impl MensagemBmc {
    pub async fn create(mm: &ModelManager, mensagem_c: MensagemForCreate) -> Result<Uuid> {
        base::create::<Self, _>(mm, mensagem_c).await
    }

    pub async fn find_by_chat_id(mm: &ModelManager, chat_id: Uuid) -> Result<Vec<Mensagem>> {
        let db = mm.db();

        let mensagens = sqlx::query_as!(
            Mensagem,
            r#"
                SELECT 
                    mensagem.created_at as created_at,
                    mensagem.conteudo as conteudo,
                    mensagem.tipo as tipo
                FROM mensagem
                WHERE mensagem.chat_id = $1
                ORDER BY mensagem.created_at
            "#,
            chat_id
        )
        .fetch_all(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(mensagens)
    }
}
