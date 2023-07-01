use std::sync::{Arc, Mutex};

use axum::{
    routing::get,
    http::StatusCode,
    extract::State,
    Router,
};
use tracing::{info};

pub struct HealthHandle {
    status: bool
}

impl HealthHandle {
    async fn health_handle(State(state): State<Arc<Mutex<HealthHandle>>>) -> (StatusCode, &'static str) {
        let s = state.as_ref().lock().unwrap();
        if s.status {
            info!("health state passing!");
            (StatusCode::OK, "passing!")
        }
        else {
            info!("health state busy!");
            (StatusCode::INTERNAL_SERVER_ERROR, "busy!")
        }
    }

    pub fn set_health_status(&mut self, _status: bool) {
        self.status = _status
    }

    pub async fn new(addr: String) -> Arc<Mutex<HealthHandle>> {
        let state = Arc::new(Mutex::new(HealthHandle { status: true }));

        let state_clone = state.clone();
        let app = Router::new()
            .route("/health", get(HealthHandle::health_handle))
            .with_state(state_clone);

        axum::Server::bind(&addr.parse().unwrap())
            .serve(app.into_make_service())
            .await.unwrap();

        state
    }
}
