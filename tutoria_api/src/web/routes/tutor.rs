use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tutoria_agent::ais::message::Message;
use tutoria_agent::ais::OaClient;
use tutoria_agent::tutoria::TutorIA;
use tutoria_agent::{send_tutoria_message, TutorIAContext};
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::model::aluno::AlunoBmc;
use crate::model::chat::{ChatBmc, ChatForCreate};
use crate::model::materia::MateriaBmc;
use crate::model::mensagem::{MensagemBmc, MensagemForCreate};
use crate::model::tutor::{TutorBmc, TutorForCreate};
use crate::model::ModelManager;
use crate::web::error::{Error, Result};

#[derive(Clone)]
struct TutorIAState {
    mm: ModelManager,
    oac: OaClient,
}

pub fn routes(mm: ModelManager, oac: OaClient) -> Router {
    let tutoria_state = TutorIAState { mm, oac };

    Router::new()
        .route("/api/tutor", post(api_create_tutor_handler))
        .route("/api/tutor/:tutor_id", get(api_get_chat_handler))
        .route(
            "/api/tutor/mensagem/:chat_id",
            post(api_create_mensagem_handler),
        )
        .with_state(tutoria_state)
}

async fn api_create_tutor_handler(
    State(tutoria_state): State<TutorIAState>,
    Json(payload): Json<TutorForCreate>,
) -> Result<Json<Value>> {
    let mm = tutoria_state.mm;

    let materia = MateriaBmc::find_by_id(&mm, payload.materia_id).await?;

    let tutor_c = TutorForCreate {
        nome: payload.nome,
        materia_id: materia.id,
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
    State(tutoria_state): State<TutorIAState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<Value>> {
    let mm = tutoria_state.mm;

    let tutor_id_str = params.get("tutor_id").ok_or(Error::ParamsNotFound)?;
    let tutor_id =
        Uuid::parse_str(&tutor_id_str).map_err(|err| Error::InvalidUuid(err.to_string()))?;
    let tutor = TutorBmc::find_by_id(&mm, tutor_id).await?;

    let user_id = ctx.user_id();

    let aluno_opt = AlunoBmc::find_by_user_Id(&mm, user_id).await?;
    if aluno_opt.is_none() {
        return Err(Error::Unauthorized("Nenhum aluno encontrado com esse id"));
    }
    let aluno = aluno_opt.unwrap();

    let chat_opt = ChatBmc::find_by_aluno_and_tutor_id(&mm, aluno.id, tutor_id).await?;

    if chat_opt.is_some() {
        let chat = chat_opt.unwrap();
        let mut mensagens = MensagemBmc::find_by_chat_id(&mm, chat.id).await?;

        let body = Json(json!({
            "result": {
                "chat": chat,
                "mensagens": mensagens.split_off(1)
            }
        }));

        return Ok(body);
    }

    let ctx = TutorIAContext {
        materia: "calculo 2".to_string(),
    };

    let chat_c = ChatForCreate {
        aluno_id: aluno.id,
        tutor_id: tutor.id,
    };

    let chat_id = ChatBmc::create(&mm, chat_c).await?;
    let chat = ChatBmc::find_by_id(&mm, chat_id).await?;

    let initial_message = TutorIA::get_initial_system_msg(ctx)
        .map_err(|err| Error::TutorIAAgentError(err.to_string()))?;

    let message_c = MensagemForCreate {
        tipo: initial_message.role,
        conteudo: initial_message.content,
        chat_id,
    };

    MensagemBmc::create(&mm, message_c).await?;

    let body = Json(json!({
        "result": {
            "chat": chat,
            "mensagens": []
        }
    }));

    Ok(body)
}

async fn api_create_mensagem_handler(
    State(tutoria_state): State<TutorIAState>,
    Path(params): Path<HashMap<String, String>>,
    Json(payload): Json<MsgCreatePayload>,
) -> Result<Json<Value>> {
    let mm = tutoria_state.mm;
    let oac = tutoria_state.oac;

    let chat_id = Uuid::parse_str(params.get("chat_id").ok_or(Error::ParamsNotFound)?)
        .map_err(|err| Error::InvalidUuid(err.to_string()))?;
    let chat = ChatBmc::find_by_id(&mm, chat_id).await?;

    let mensagem_c = MensagemForCreate {
        conteudo: payload.conteudo,
        tipo: "user".to_string(),
        chat_id: chat.id,
    };
    MensagemBmc::create(&mm, mensagem_c).await?;

    let mensagens = MensagemBmc::find_by_chat_id(&mm, chat.id).await?;

    let messages_formatted = mensagens
        .into_iter()
        .map(|msg| Message {
            role: msg.tipo,
            content: msg.conteudo,
        })
        .collect();

    let tutoria = TutorIA::new(messages_formatted);

    let tutoria = send_tutoria_message(oac, tutoria)
        .await
        .map_err(|err| Error::TutorIAAgentError(err.to_string()))?;

    let response_message = tutoria.messages.last().unwrap();

    let mensagemtutoria_c = MensagemForCreate {
        conteudo: response_message.content.clone(),
        tipo: "assistant".to_string(),
        chat_id: chat_id,
    };
    MensagemBmc::create(&mm, mensagemtutoria_c).await?;

    let mut mensagens = MensagemBmc::find_by_chat_id(&mm, chat.id).await?;

    let body = Json(json!({
        "result": {
            "resposta": mensagens.split_off(1)
        }
    }));

    Ok(body)
}

#[derive(Clone, Deserialize)]
struct MsgCreatePayload {
    conteudo: String,
}
