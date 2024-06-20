use sqlb::HasFields;
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;
use crate::model::Result;
use super::ModelManager;
use super::Error;

pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn create<MC, E>(mm: &ModelManager, data: E) -> Result<Uuid> 
where
    MC: DbBmc,
    E: HasFields
{
    let db = mm.db();

    let fields = data.not_none_fields();
    let (id,) = sqlb::insert()
        .table(MC::TABLE)
        .data(fields)
        .returning(&["id"])
        .fetch_one::<_, (Uuid,)>(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

    Ok(id)
}

pub async fn find_by_id<MC, E>(mm: &ModelManager, id: Uuid) -> Result<Option<E>>
where
    MC: DbBmc ,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    E: HasFields,
{
    let db = mm.db();

    let entity: Option<E> = sqlb::select()
        .table(MC::TABLE)
        .columns(E::field_names())
        .and_where("id", "=", id)
        .fetch_optional(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

    
    Ok(entity)   
}