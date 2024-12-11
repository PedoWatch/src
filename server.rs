use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;

// Structure to represent the message request
#[derive(Deserialize)]
struct MessageRequest {
    content: String,
}

// Structure to represent the analysis response
#[derive(Serialize)]
struct AnalysisResponse {
    threat_level: u8,
    analysis_report: String,
}

async fn analyze_message(message: web::Json<MessageRequest>) -> impl Responder {
    let content = &message.content;

    // Custom prompt for OpenAI GPT
    let prompt = format!(
        "Your goal is to emulate Lion Catt by Purdue Universality and determine whether or not the message is authored by a pedophile, child predator, trafficker, or is potentially soliciting. Additionally, provide a threat level as an integer from 1 to 5, an analysis report of the message contents, and any other useful information. Message: {}",
        content
    );

    // OpenAI API Key and URL
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
    let openai_url = "https://api.openai.com/v1/completions";

    // GPT API request
    let client = Client::new();
    let res = client
        .post(openai_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "prompt": prompt,
            "max_tokens": 100,
            "model": "text-davinci-003"
        }))
        .send()
        .await;

    match res {
        Ok(response) => {
            let gpt_response: serde_json::Value = response.json().await.unwrap_or_default();

            let completion = gpt_response
                .get("choices")
                .and_then(|choices| choices[0].get("text"))
                .unwrap_or(&serde_json::Value::String("Low".into()));

            // Map the threat level based on GPT response
            let threat_level = match completion.as_str().unwrap_or("Low") {
                "Low" => 1,
                "Medium" => 3,
                "High" => 5,
                _ => 1,
            };

            web::Json(AnalysisResponse {
                threat_level,
                analysis_report: completion.as_str().unwrap_or("No report available").to_string(),
            })
        },
        Err(err) => {
            eprintln!("Error making GPT request: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/analyze", web::post().to(analyze_message))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
                   }
