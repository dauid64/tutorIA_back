use std::path;

use crate::{
    config,
    ctx::Ctx,
    manager::TutorIAManager,
    model::{
        materia::{MateriaBmc, MateriaForCreate},
        professor::ProfessorBmc,
    },
    utils::convert_file_for_string,
    web::error::{Error, Result},
};
use axum::{
    extract::{Multipart, Path, State},
    routing::{get, post},
    Json, Router,
};
use futures::TryFutureExt;
use lopdf::Document;
use redis::{AsyncCommands, Commands};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tutoria_agent::{
    ais::embeddings::{self, get_embeddings},
    tutoria::TutorIA,
};
use uuid::Uuid;

pub fn router(tutoria_manager: TutorIAManager) -> Router {
    Router::new()
        .route("/api/materia", post(api_create_materia_handler))
        .route("/api/materia", get(api_find_materia_handler))
        .route("/api/materia/aluno/add", post(api_add_aluno_of_materia))
        .route(
            "/api/materia/aluno/remove",
            post(api_remove_aluno_of_materia),
        )
        .route(
            "/api/materia/:materia_id/alunos/registered",
            get(api_find_alunos_registered_in_materia),
        )
        .route(
            "/api/materia/:materia_id/alunos/not-registered",
            get(api_find_alunos_not_registered_in_materia),
        )
        .with_state(tutoria_manager)
}

async fn api_create_materia_handler(
    ctx: Ctx,
    State(tutoria_manager): State<TutorIAManager>,
    mut multipart: Multipart,
) -> Result<Json<Value>> {
    let user_id = ctx.user_id();

    let oac = tutoria_manager.oac();
    let tutoria = TutorIA::new(vec![]);
    let mut rc = tutoria_manager.rc().await;

    let professor_opt = ProfessorBmc::find_by_user_id(&tutoria_manager, user_id).await?;
    if professor_opt.is_none() {
        return Err(Error::Unauthorized(
            "Nenhum professor encontrado com esse id",
        ));
    }
    let professor = professor_opt.unwrap();

    let mut materia_c = MateriaForCreate::default();
    materia_c.professor_id = professor.id;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == "conteudos" {
            let original_file_name = field.file_name().unwrap().to_string();
            let new_file_name = format!("{}_{}", Uuid::new_v4(), original_file_name);
            let file_dir = format!(
                "{}materia/conteudos/{}",
                &config().web_folder,
                new_file_name
            );

            let data = field.bytes().await.unwrap();

            let mut file = File::create(&file_dir).await.unwrap();
            file.write(&data).await.unwrap();
            
            let string_file = convert_file_for_string(&data)?;

            let embeddings = get_embeddings(oac, string_file, &tutoria)
                .await
                .map_err(|err| Error::TutorIAAgentError(err.to_string()))?;

            let embedding = &embeddings[0].embedding;

            let embeddings_json = serde_json::to_string(&embedding)
                .map_err(|err| Error::FailToConvertForJson(err.to_string()))?;

            let _: () = rc.set(file_dir.as_str(), embeddings_json)
                .await
                .map_err(|err| Error::RedisError(err.to_string()))?;

            materia_c.conteudos.push(file_dir);
        } else {
            let data = field.text().await.unwrap();

            match field_name.as_str() {
                "nome" => materia_c.nome = data,
                "descricao" => materia_c.descricao = data,
                "professor_id" => {
                    materia_c.professor_id = Uuid::parse_str(data.as_str()).map_err(|_| {
                        Error::Router("Não foi possível converter professor_id para Uuid")
                    })?
                }
                _ => (),
            }
        }
    }

    MateriaBmc::validate(&materia_c).await?;
    let id = MateriaBmc::create(&tutoria_manager, materia_c).await?;

    let body = Json(json!({
        "result": {
            "id": id
        }
    }));

    Ok(body)
}

async fn api_find_materia_handler(
    ctx: Ctx,
    State(tutoria_manager): State<TutorIAManager>,
) -> Result<Json<Value>> {
    let user_id = ctx.user_id();

    let professor_opt = ProfessorBmc::find_by_user_id(&tutoria_manager, user_id).await?;
    if professor_opt.is_none() {
        return Err(Error::Unauthorized(
            "Nenhum professor encontrado com esse id",
        ));
    }

    let professor = professor_opt.unwrap();

    let materias = MateriaBmc::find_by_professor_id(&tutoria_manager, professor.id).await?;

    let body = Json(json!({
        "result": {
            "materias": materias
        }
    }));

    Ok(body)
}

async fn api_add_aluno_of_materia(
    State(tutoria_manager): State<TutorIAManager>,
    Json(payload): Json<AlunoMateriaPayload>,
) -> Result<Json<Value>> {
    let materia_id =
        Uuid::parse_str(&payload.materia_id).map_err(|err| Error::InvalidUuid(err.to_string()))?;

    let aluno_id =
        Uuid::parse_str(&payload.aluno_id).map_err(|err| Error::InvalidUuid(err.to_string()))?;

    MateriaBmc::add_aluno(&tutoria_manager, aluno_id, materia_id).await?;

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

async fn api_remove_aluno_of_materia(
    State(tutoria_manager): State<TutorIAManager>,
    Json(payload): Json<AlunoMateriaPayload>,
) -> Result<Json<Value>> {
    let materia_id =
        Uuid::parse_str(&payload.materia_id).map_err(|err| Error::InvalidUuid(err.to_string()))?;

    let aluno_id =
        Uuid::parse_str(&payload.aluno_id).map_err(|err| Error::InvalidUuid(err.to_string()))?;

    MateriaBmc::remove_aluno(&tutoria_manager, aluno_id, materia_id).await?;

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Deserialize)]
struct AlunoMateriaPayload {
    materia_id: String,
    aluno_id: String,
}

async fn api_find_alunos_registered_in_materia(
    State(tutoria_manager): State<TutorIAManager>,
    Path(materia_id): Path<String>,
) -> Result<Json<Value>> {
    let materia_id =
        Uuid::parse_str(&materia_id).map_err(|err| Error::InvalidUuid(err.to_string()))?;

    let alunos_registered =
        MateriaBmc::find_alunos_registered(&tutoria_manager, materia_id).await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "alunos_registered": alunos_registered
        }
    }));

    Ok(body)
}

async fn api_find_alunos_not_registered_in_materia(
    State(tutoria_manager): State<TutorIAManager>,
    Path(materia_id): Path<String>,
) -> Result<Json<Value>> {
    let materia_id =
        Uuid::parse_str(&materia_id).map_err(|err| Error::InvalidUuid(err.to_string()))?;

    let alunos_not_registered =
        MateriaBmc::find_alunos_not_registered(&tutoria_manager, materia_id).await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "alunos_not_registered": alunos_not_registered
        }
    }));

    Ok(body)
}
