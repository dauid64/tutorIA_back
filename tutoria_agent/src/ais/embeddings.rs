use async_openai::types::{CreateEmbeddingRequestArgs, Embedding};

use crate::tutoria::TutorIA;

use super::{error::Result, OaClient};

pub async fn get_embeddings(oac: &OaClient, text: Vec<String>, tutoria: &TutorIA) -> Result<Vec<Embedding>> {
    let request = CreateEmbeddingRequestArgs::default()
        .model(tutoria.model_embeddings.clone())
        .input(text)
        .build()?;

    let response = oac.embeddings().create(request).await?;
    
    Ok(response.data)
}

#[cfg(test)]
mod tests {
    use crate::ais::new_oa_client;
    use super::*;

    #[tokio::test]
    async fn test_create_embedding_ok() -> Result<()>{
        let oac = new_oa_client()?;

        let tutoria = TutorIA::new(vec![]);

        let embedding = get_embeddings(&oac, vec!["Olá, tudo certo?".to_string()], &tutoria).await?;

        println!("{:?}", embedding[0].embedding);

        Ok(())
    }
}