use axum::Json;
use crud_contracts::models::HealthResponse;

pub async fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "UP".to_string(),
    })
}
