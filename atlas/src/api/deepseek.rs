use async_openai::config::OpenAIConfig;

use crate::{AskModel, Model};

use super::openai::OpenAIClient;

/// refs: https://api-docs.deepseek.com/zh-cn/quick_start/pricing
#[derive(Debug, Clone)]
pub enum DeepSeekModels {
    DeepSeekChat,
    DeepSeekReasoner,
}

impl Model for DeepSeekModels {
    fn as_str(&self) -> &str {
        match self {
            DeepSeekModels::DeepSeekChat => "deepseek-chat",
            DeepSeekModels::DeepSeekReasoner => "deepseek-reasoner",
        }
    }
}

const DEEPSEEK_API_BASE: &str = "https://api.deepseek.com/v1";

pub struct DeepSeekClient {
    openai_client: OpenAIClient,
}

/// refs: https://api-docs.deepseek.com/zh-cn/
/// The DeepSeek API is fully compatible with OpenAI's API,
/// so the same client as OpenAI is used to make requests here.
impl DeepSeekClient {
    pub fn new(api_key: String, model: DeepSeekModels, api_base: Option<String>) -> Self {
        let api_base = match api_base {
            Some(api) => api,
            None => DEEPSEEK_API_BASE.to_owned(),
        };

        let config = OpenAIConfig::new()
            .with_api_base(api_base)
            .with_api_key(&api_key);

        let client = async_openai::Client::with_config(config);
        Self {
            openai_client: OpenAIClient::from_client_and_model(client, Box::new(model)),
        }
    }
}

impl AskModel for DeepSeekClient {
    async fn ask_model_with_context(
        &self,
        context: crate::ChatMessage,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.openai_client.ask_model_with_context(context).await
    }
}

#[cfg(test)]
mod test {
    use crate::api::{
        deepseek::{DeepSeekClient, DeepSeekModels},
        test::{get_deepseek_key, test_client_with_context},
    };

    #[tokio::test]
    async fn test_deepseek() {
        let api_key = get_deepseek_key().unwrap();
        let models = [
            DeepSeekModels::DeepSeekChat,
            DeepSeekModels::DeepSeekReasoner,
        ];

        for model in models {
            let api_base = std::env::var("DEEPSEEK_API_BASE");
            let client = DeepSeekClient::new(
                api_key.clone(),
                model,
                match api_base {
                    Ok(api_base) => Some(api_base),
                    Err(_) => None,
                },
            );
            test_client_with_context(client).await;
        }
    }
}
