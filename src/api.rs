use crate::config::Config;
use anyhow::{Context, Result};
use colored::Colorize;
use futures_util::StreamExt;
use reqwest::{header, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, Write};

const DEEPSEEK_API_URL: &str = "https://api.deepseek.com/chat/completions";

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
    temperature: f64,
}

/// Generate a commit message using the DeepSeek API
pub async fn generate_commit_message(
    config: &Config,
    diff: &str,
    history_title: Option<Vec<String>>,
) -> Result<String> {
    let client = reqwest::Client::new();

    // Request body
    let request = build_request(config, diff, history_title);
    let response = send_api_request(&client, &request, &config.api_key).await?;

    println!("");
    println!(
        "{} {}",
        "🤖".green(),
        format!("Generating commit message, using {}:\n", config.model).cyan()
    );

    let commit_message = process_stream_response(response).await?;

    Ok(commit_message.trim().to_string())
}

/// Build the request body for the API
fn build_request(
    config: &Config,
    diff: &str,
    history_title: Option<Vec<String>>,
) -> ChatCompletionRequest {
    // Prompt
    let format_instruction = config.format.get_prompt();
    let mut prompt = format!(
        "Please write a commit message for the following changes:\n\n{}\n\n{}",
        format_instruction, diff
    );

    match history_title {
        Some(titles) if !titles.is_empty() => {
            prompt.push_str(
                "\n\nFor reference, here are some recent commit titles in this repository:\n",
            );
            for (i, title) in titles.iter().enumerate() {
                prompt.push_str(&format!("{}. {}\n", i + 1, title));
            }
        }
        _ => {
            prompt.push_str("\n\nNo recent commit titles available for reference!");
        }
    }

    // Request body
    ChatCompletionRequest {
        model: config.model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a Git commit message assistant. Your task is to generate concise and clear commit messages based on the provided Git diff.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        stream: true,
        temperature: 0.3,
    }
}

/// Send the API request
async fn send_api_request(
    client: &reqwest::Client,
    request: &ChatCompletionRequest,
    api_key: &str,
) -> Result<Response> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let response = client
        .post(DEEPSEEK_API_URL)
        .headers(headers)
        .json(request)
        .send()
        .await
        .context("Failed to send API request, please check your network connection")?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        anyhow::bail!("API returned error: {}", error_text);
    }

    Ok(response)
}

/// Process the stream response from the API
async fn process_stream_response(response: Response) -> Result<String> {
    log::info!("Generating commit message:");

    let mut commit_message = String::new();
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        let chunk_str = std::str::from_utf8(&chunk)?;

        for line in chunk_str.lines() {
            if line.starts_with("data: ") {
                process_data_line(line, &mut commit_message)?;
            }
        }
    }

    log::info!("\nCommit message generated successfully");
    Ok(commit_message.trim().to_string())
}

/// Process a line of data from the stream response
fn process_data_line(line: &str, commit_message: &mut String) -> Result<()> {
    let data = &line["data: ".len()..];

    // Skip the "[DONE]" line
    if data == "[DONE]" {
        return Ok(());
    }

    // Parse JSON
    if let Ok(json) = serde_json::from_str::<Value>(data) {
        extract_content_from_json(&json, commit_message)?;
    }

    Ok(())
}

/// Extract content from the JSON response
fn extract_content_from_json(json: &Value, commit_message: &mut String) -> Result<()> {
    json.get("choices")
        .and_then(|choices| choices.as_array())
        .and_then(|arr| arr.get(0))
        .and_then(|choice| choice.get("delta"))
        .and_then(|delta| delta.get("content"))
        .and_then(|content| content.as_str())
        .map(|content| {
            commit_message.push_str(content);
            print!("{}", content);
            io::stdout().flush()
        })
        .transpose()?;

    Ok(())
}
