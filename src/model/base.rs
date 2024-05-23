use sqlb::HasFields;
use crate::model::Result;
use super::ModelManager;

pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn create<MC, E>(mm: &ModelManager, data: E) -> Result<i64> 
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
        .fetch_one::<_, (i64,)>(db)
        .await?;

    Ok(id)
}