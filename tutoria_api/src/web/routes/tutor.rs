use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tutoria_agent::{
    create_tutoria_assistant, create_tutoria_thread, get_tutoria, send_tutoria_message,
    TutorIAContext,
};
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::model::aluno::AlunoBmc;
use crate::model::chat::{ ChatBmc, ChatForCreate};
use crate::model::materia::MateriaBmc;
use crate::model::mensagem::{MensagemBmc, MensagemForCreate};
use crate::model::tutor::{TutorBmc, TutorForCreate};
use crate::model::ModelManager;
use crate::web::error::{Error, Result};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/tutor", post(api_create_tutor_handler))
        .route("/api/tutor/:tutor_id", get(api_get_chat_handler))
        .route("/api/tutor/mensagem/:chat_id", post(api_post_mensagem_handler))
        .with_state(mm)
}

async fn api_create_tutor_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<TutorForCreate>,
) -> Result<Json<Value>> {
    let materia = MateriaBmc::find_by_id(&mm, payload.materia_id).await?;
    let assistant_name = format!("{} {}", payload.nome, Uuid::new_v4());

    let assitant_ctx = TutorIAContext {
        materia: materia.nome,
    };

    let tutoria = create_tutoria_assistant(assistant_name, assitant_ctx)
        .await
        .map_err(|err| Error::TutorIAAgentError(err.to_string()))?;

    let tutor_c = TutorForCreate {
        nome: payload.nome,
        assistant_id: tutoria.assistant_id.as_str().to_string(),
        materia_id: payload.materia_id,
    };

    TutorBmc::validate(&tutor_c).await?;
    let tutor_id = TutorBmc::create(&mm, tutor_c).await?;

    let body = Json(json!({
        "result": {
            "id": tutor_id
        }
    }));

    Ok(body)
}

async fn api_get_chat_handler(
    ctx: Ctx,
    State(mm): State<ModelManager>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<Value>> {
    let tutor_id_str = params.get("tutor_id").ok_or(Error::ParamsNotFound)?;
    let tutor_id =
        Uuid::parse_str(&tutor_id_str).map_err(|err| Error::InvalidUuid(err.to_string()))?;

    let user_id = ctx.user_id();

    let aluno_opt = AlunoBmc::find_by_user_Id(&mm, user_id).await?;
    if aluno_opt.is_none() {
        return Err(Error::Unauthorized("Nenhum aluno encontrado com esse id"));
    }
    let aluno = aluno_opt.unwrap();

    let chat_opt = ChatBmc::find_by_aluno_and_tutor_id(&mm, aluno.id, tutor_id).await?;

    if chat_opt.is_some() {
        let chat = chat_opt.unwrap();

        let mensagens = MensagemBmc::find_by_chat_id(&mm, chat.id).await?;
        let body = Json(json!({
            "result": {
                "chat": chat,
                "mensagens": mensagens
            }
        }));

        return Ok(body);
    }

    let tutor = TutorBmc::find_by_id(&mm, tutor_id).await?;

    let tutoria = get_tutoria(&tutor.assistant_id)
        .await
        .map_err(|err| Error::TutorIAAgentError(err.to_string()))?;

    let thread_id = create_tutoria_thread(&tutoria)
        .await
        .map_err(|err| Error::TutorIAAgentError(err.to_string()))?;

    let chat_c = ChatForCreate {
        aluno_id: aluno.id,
        tutor_id: tutor_id,
        thread_id,
    };

    let chat_id = ChatBmc::create(&mm, chat_c).await?;
    let chat = ChatBmc::find_by_id(&mm, chat_id).await?;
    let mensagens = MensagemBmc::find_by_chat_id(&mm, chat_id).await?;

    let body = Json(json!({
        "result": {
            "chat": chat,
            "mensagens": mensagens
        }
    }));

    Ok(body)
}

async fn api_post_mensagem_handler(
    State(mm): State<ModelManager>,
    Path(params): Path<HashMap<String, String>>,
    Json(payload): Json<MsgCreatePayload>,
) -> Result<Json<Value>> {
    let chat_id = Uuid::parse_str(params.get("chat_id").ok_or(Error::ParamsNotFound)?)
        .map_err(|err| Error::InvalidUuid(err.to_string()))?;

    let chat = ChatBmc::find_by_id(&mm, chat_id).await?;
    let tutor = TutorBmc::find_by_id(&mm, chat.tutor_id).await?;
    let tutoria = get_tutoria(&tutor.assistant_id)
        .await
        .map_err(|err| Error::TutorIAAgentError(err.to_string()))?;

    let mensagem_c = MensagemForCreate {
        conteudo: payload.conteudo.clone(),
        tipo: "user".to_string(),
        chat_id: chat.id,
    };

    MensagemBmc::create(&mm, mensagem_c).await?;

    let responsetutoria = send_tutoria_message(&tutoria, &chat.thread_id, payload.conteudo)
        .await
        .map_err(|err| Error::TutorIAAgentError(err.to_string()))?;

    let mensagemtutoria_c = MensagemForCreate {
        conteudo: responsetutoria.clone(),
        tipo: "assistant".to_string(),
        chat_id: chat_id
    };

    MensagemBmc::create(&mm, mensagemtutoria_c).await?;

    let body = Json(json!({
        "result": {
            "resposta": responsetutoria
        }
    }));

    Ok(body)
}

#[derive(Clone, Deserialize)]
struct MsgCreatePayload {
    conteudo: String,
}
