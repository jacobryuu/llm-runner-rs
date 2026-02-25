use crate::config::Config;
use crate::error::{LlmError, Result};
use std::io::Write;
use std::num::NonZeroU32;
use tracing::{info, warn, debug};

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::{AddBos, LlamaModel, Special, LlamaChatMessage};
use llama_cpp_2::sampling::LlamaSampler;

pub struct InferenceEngine<'a> {
    model: &'a LlamaModel,
    ctx: LlamaContext<'a>,
    chat_history: Vec<LlamaChatMessage>,
}

impl<'a> InferenceEngine<'a> {
    /// Initializes the inference engine with a model and backend.
    pub fn new(model: &'a LlamaModel, backend: &LlamaBackend) -> Result<Self> {
        info!("Initializing inference engine");
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(2048));
        let ctx = model.new_context(backend, ctx_params)
            .map_err(|e| LlmError::ModelLoadError(e.to_string()))?;

        info!("Inference engine initialized successfully");
        Ok(Self {
            model,
            ctx,
            chat_history: Vec::new(),
        })
    }

    /// Generate a response without maintaining chat history
    pub fn generate(&mut self, prompt: &str, max_tokens: usize, temperature: f32, top_p: f32) -> Result<String> {
        debug!("Generating response for prompt length: {}", prompt.len());
        
        let tokens = self.model.str_to_token(prompt, AddBos::Always)
            .map_err(|e| LlmError::InferenceError(e.to_string()))?;

        self.ctx.clear_kv_cache();

        let mut batch = LlamaBatch::new(2048, 1);
        let last_index: i32 = (tokens.len() - 1) as i32;

        for (i, token) in tokens.iter().enumerate() {
            batch.add(*token, i as i32, &[0], i as i32 == last_index)
                .map_err(|e| LlmError::InferenceError(e.to_string()))?;
        }

        self.ctx.decode(&mut batch)
            .map_err(|e| LlmError::InferenceError(e.to_string()))?;

        let mut generated_tokens = 0;
        let eos_token = self.model.token_eos();
        let mut response_text = String::new();

        let mut sampler_chain = LlamaSampler::chain_simple([
            LlamaSampler::penalties(64, 1.1, 0.0, 0.0),
            LlamaSampler::temp(temperature),
            LlamaSampler::top_p(top_p, 1),
            LlamaSampler::dist(1234),
        ]);

        let mut n_curr = batch.n_tokens();

        while generated_tokens < max_tokens {
            let new_token = sampler_chain.sample(&self.ctx, batch.n_tokens() - 1);
            sampler_chain.accept(new_token);

            if new_token == eos_token {
                break;
            }

            let str_slice = self.model.token_to_str(new_token, Special::Tokenize)
                .map_err(|e| LlmError::InferenceError(e.to_string()))?;
            response_text.push_str(&str_slice);

            batch.clear();
            batch.add(new_token, n_curr, &[0], true)
                .map_err(|e| LlmError::InferenceError(e.to_string()))?;
            
            n_curr += 1;
            self.ctx.decode(&mut batch)
                .map_err(|e| LlmError::InferenceError(e.to_string()))?;
            generated_tokens += 1;
        }

        debug!("Generated {} tokens", generated_tokens);
        Ok(response_text)
    }

    /// Generates a response to a user input, maintaining conversation history.
    pub fn chat(&mut self, user_input: &str, config: &Config) -> Result<()> {
        info!("Processing chat message");
        self.chat_history.push(LlamaChatMessage::new("user".to_string(), user_input.to_string())
            .map_err(|e| LlmError::InferenceError(e.to_string()))?);

        let prompt = match self.model.chat_template(None) {
            Ok(template) => {
                match self.model.apply_chat_template(&template, &self.chat_history, true) {
                    Ok(p) => p,
                    Err(e) => {
                        warn!("Failed to apply chat template: {}", e);
                        user_input.to_string()
                    }
                }
            },
            Err(_) => {
                user_input.to_string()
            }
        };

        let tokens = self.model.str_to_token(&prompt, AddBos::Always)
            .map_err(|e| LlmError::InferenceError(e.to_string()))?;

        self.ctx.clear_kv_cache();

        let mut batch = LlamaBatch::new(2048, 1);
        let last_index: i32 = (tokens.len() - 1) as i32;

        for (i, token) in tokens.iter().enumerate() {
            batch.add(*token, i as i32, &[0], i as i32 == last_index)
                .map_err(|e| LlmError::InferenceError(e.to_string()))?;
        }

        self.ctx.decode(&mut batch)
            .map_err(|e| LlmError::InferenceError(e.to_string()))?;

        let mut generated_tokens = 0;
        let eos_token = self.model.token_eos();
        let mut response_text = String::new();

        let mut sampler_chain = LlamaSampler::chain_simple([
            LlamaSampler::penalties(64, 1.1, 0.0, 0.0),
            LlamaSampler::temp(0.7),
            LlamaSampler::top_p(0.9, 1),
            LlamaSampler::dist(1234),
        ]);

        let mut n_curr = batch.n_tokens();

        while generated_tokens < config.max_tokens {
            let new_token = sampler_chain.sample(&self.ctx, batch.n_tokens() - 1);
            sampler_chain.accept(new_token);

            if new_token == eos_token {
                break;
            }

            let str_slice = self.model.token_to_str(new_token, Special::Tokenize)
                .map_err(|e| LlmError::InferenceError(e.to_string()))?;
            print!("{}", str_slice);
            std::io::stdout().flush()?;
            response_text.push_str(&str_slice);

            batch.clear();
            batch.add(new_token, n_curr, &[0], true)
                .map_err(|e| LlmError::InferenceError(e.to_string()))?;
            
            n_curr += 1;
            self.ctx.decode(&mut batch)
                .map_err(|e| LlmError::InferenceError(e.to_string()))?;
            generated_tokens += 1;
        }

        println!();

        self.chat_history.push(LlamaChatMessage::new("assistant".to_string(), response_text)
            .map_err(|e| LlmError::InferenceError(e.to_string()))?);

        info!("Chat completed, generated {} tokens", generated_tokens);
        Ok(())
    }
}
