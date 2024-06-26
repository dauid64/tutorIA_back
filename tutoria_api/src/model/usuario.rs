use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::{postgres::PgRow, prelude::FromRow};
use uuid::Uuid;

use crate::manager::TutorIAManager;

use super::{base::{self, DbBmc}, Result, Error};

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Usuario {
    pub id: Uuid,
    pub username: String
}

#[derive(Deserialize, FromRow, Fields)]
pub struct UsuarioForCreate {
    pub username: String,
    pub pwd: String,
}

#[derive(Deserialize, FromRow, Fields)]
pub struct UsuarioForAuth {
    pub id: Uuid,
    pub username: String
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UsuarioForLogin {
    pub id: Uuid,
    pub username: String,
    pub pwd: Option<String>,
}

pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {
    
}

impl UserBy for Usuario{}
impl UserBy for UsuarioForCreate{}
impl UserBy for UsuarioForAuth{}
impl UserBy for UsuarioForLogin{}

impl UsuarioForCreate {
    fn validate(&self) -> Result<()> {
        let username = self.username.trim();
        let pwd = self.pwd.trim();

        if username.is_empty() {
            return Err(Error::ValidateFail("Nome de usuário em branco"))
        }

        if username.contains(char::is_whitespace) {
            return Err(Error::ValidateFail("Nome de usuário não pode conter espaços"))
        }

        if pwd.is_empty() {
            return Err(Error::ValidateFail("Senha em branco"))
        }

        Ok(())
    }
}

pub struct UsuarioBmc;

impl DbBmc for UsuarioBmc {
    const TABLE: &'static str = "usuario";
}

impl UsuarioBmc {
    pub async fn validate(usuario_c: &UsuarioForCreate) -> Result<()> {
        usuario_c.validate()
    }

    pub async fn create(tutoria_manager: &TutorIAManager, usuario_c: UsuarioForCreate) -> Result<Uuid> {
        base::create::<Self, _>(tutoria_manager, usuario_c).await
    }

    pub async fn first_by_username<E>(
        tutoria_manager: &TutorIAManager,
        username: &str,
    ) -> Result<Option<E>> 
    where
        E: UserBy
    {
        let db = tutoria_manager.db();

        let user = sqlb::select()
            .table(Self::TABLE)
            .and_where("username", "=", username)
            .fetch_optional::<_, E>(db)
            .await
            .map_err(|err| Error::Sqlx(err.to_string()))?;

        Ok(user)
    }
}