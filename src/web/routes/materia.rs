use axum::{extract::{Multipart, State}, routing::post, Json, Router};
use serde_json::{json, Value};
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;
use crate::{config, model::{materia::{MateriaBmc, MateriaForCreate}, ModelManager}, web::error::{Error, Result}};

pub fn router(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/materia", post(api_create_materia_handler))
        .with_state(mm)
}

async fn api_create_materia_handler(
    State(mm): State<ModelManager>,
    mut multipart: Multipart
) -> Result<Json<Value>> {
    let mut materia_c = MateriaForCreate::default();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == "conteudos" {
            let original_file_name = field.file_name().unwrap().to_string();
            let new_file_name = format!("{}_{}", Uuid::new_v4(), original_file_name);
            let file_dir = format!("{}materia/conteudos/{}.pdf", &config().web_folder, new_file_name);

            let data = field.bytes().await.unwrap();

            let mut file = File::create(&file_dir)
                .await.unwrap();
            file.write(&data).await.unwrap();

            materia_c.conteudos.push(file_dir);
        } else {
            let data = field.text().await.unwrap();

            match field_name.as_str() {
                "nome" => materia_c.nome = data,
                "descricao" => materia_c.descricao = data,
                "professor_id" => {
                    materia_c.professor_id = Uuid::parse_str(data.as_str())
                        .map_err(|_| Error::Router("Não foi possível converter professor_id para Uuid"))?
                }
                _ => (),
            }
        }
    }

    MateriaBmc::validate(&materia_c).await?;
    let id = MateriaBmc::create(&mm, materia_c).await?;

    let body = Json(json!({
        "result": {
            "id": id
        }
    }));

    Ok(body) 
}