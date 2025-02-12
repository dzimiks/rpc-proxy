use {
    serde::{Deserialize, Serialize},
    wc::metrics::TaskMetrics,
};

pub mod health;
pub mod identity;
pub mod metrics;
pub mod proxy;
pub mod ws_proxy;

static HANDLER_TASK_METRICS: TaskMetrics = TaskMetrics::new("handler_task");

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RpcQueryParams {
    pub chain_id: String,
    pub project_id: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    status: String,
}
