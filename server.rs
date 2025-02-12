use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use reqwest::Client;
use std::{env, net::IpAddr};
use dotenv::dotenv;
use tracing::{info, error, warn};
use tracing_subscriber;
use actix_rate_limit::{RateLimiter, MemoryStore, MemoryStoreActor};

// ---- Data Structures ----
#[derive(Deserialize)]
struct MessageRequest {
    content: String,
    user_ip: Option<String>,
}

#[derive(Serialize)]
struct AnalysisResponse {
    threat_level: u8,
    category: String,
    analysis_report: String,
}

// ---- Threat Analysis Function ----
async fn analyze_message(
    db: web::Data<PgPool>,
    message: web::Json<MessageRequest>,
    client_ip: Option<web::ReqData<IpAddr>>,
) -> impl Responder {
    let content = &message.content;
    let user_ip = message.user_ip.clone().unwrap_or_else(|| client_ip.map_or("Unknown".to_string(), |ip| ip.to_string()));

    let prompt = format!(
        "Analyze this message and classify it into one of the following categories: 
        1) Harmless
        2) Grooming 
        3) Solicitation 
        4) Exploitation 
        5) High-risk threat 
        Provide a threat level from 1-5, and a brief analysis. Message: {}",
        content
    );

    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            error!("Missing OPENAI_API_KEY");
            return HttpResponse::InternalServerError().body("Missing API key");
        }
    };

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "text-davinci-003",
            "prompt": prompt,
            "max_tokens": 150
        }))
        .send()
        .await;

    match response {
        Ok(res) => {
            let gpt_response: serde_json::Value = res.json().await.unwrap_or_else(|_| serde_json::json!({}));
            let completion = gpt_response
                .get("choices")
                .and_then(|choices| choices.get(0))
                .and_then(|choice| choice.get("text"))
                .and_then(|text| text.as_str())
                .unwrap_or("No analysis available");

            let (threat_level, category) = classify_threat(completion);

            info!("Message analyzed (Threat Level {}): {}", threat_level, category);

            // Store in database for law enforcement review
            if let Err(err) = sqlx::query!(
                "INSERT INTO flagged_messages (content, threat_level, category, user_ip) VALUES ($1, $2, $3, $4)",
                content,
                threat_level as i32,
                category,
                user_ip
            )
            .execute(db.get_ref())
            .await
            {
                error!("Database insert failed: {:?}", err);
            }

            HttpResponse::Ok().json(AnalysisResponse {
                threat_level,
                category,
                analysis_report: completion.to_string(),
            })
        }
        Err(err) => {
            error!("Error contacting OpenAI API: {:?}", err);
            HttpResponse::InternalServerError().body("AI request failed")
        }
    }
}

// ---- Threat Level Classification ----
fn classify_threat(response: &str) -> (u8, String) {
    if response.contains("High-risk threat") {
        (5, "High-risk threat".to_string())
    } else if response.contains("Exploitation") {
        (4, "Exploitation".to_string())
    } else if response.contains("Solicitation") {
        (3, "Solicitation".to_string())
    } else if response.contains("Grooming") {
        (2, "Grooming".to_string())
    } else {
        (1, "Harmless".to_string())
    }
}

// ---- Main Application Setup ----
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url).await.expect("Failed to connect to database");

    let rate_limit_store = MemoryStore::new();
    let rate_limiter = RateLimiter::new(MemoryStoreActor::from(rate_limit_store.clone()))
        .with_interval(std::time::Duration::from_secs(60)) // Limit requests per minute
        .with_max_requests(5);

    let server_address = "127.0.0.1:8080";
    info!("Server starting on {}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(Logger::default())
            .wrap(rate_limiter.clone())
            .route("/analyze", web::post().to(analyze_message))
    })
    .bind(server_address)?
    .run()
    .await
}