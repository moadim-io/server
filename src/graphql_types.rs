// Shared between src/routes/graphql.rs (runtime) and src/build/graphql.rs (SDL generation).
// Must not import anything from the server crate — only async-graphql + serde primitives.

use async_graphql::{InputObject, SimpleObject};

/// Arbitrary JSON value (object, array, or primitive).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JSON(pub serde_json::Value);
async_graphql::scalar!(JSON, "JSON", "Arbitrary JSON value (object, array, or primitive).");

/// A managed or read-only system cron job.
#[derive(SimpleObject)]
#[graphql(name = "CronJob")]
pub struct GqlCronJob {
    pub id: String,
    pub schedule: String,
    pub handler: String,
    pub metadata: JSON,
    pub enabled: bool,
    /// `"managed"` for server-owned jobs; `"system:*"` for read-only system entries.
    pub source: String,
    /// Unix timestamp (seconds).
    pub created_at: u64,
    /// Unix timestamp (seconds).
    pub updated_at: u64,
}

/// A cron job with its handler registration status.
#[derive(SimpleObject)]
#[graphql(name = "CronJobResponse")]
pub struct GqlCronJobResponse {
    pub id: String,
    pub schedule: String,
    pub handler: String,
    pub metadata: JSON,
    pub enabled: bool,
    /// `"managed"` for server-owned jobs; `"system:*"` for read-only system entries.
    pub source: String,
    /// Unix timestamp (seconds).
    pub created_at: u64,
    /// Unix timestamp (seconds).
    pub updated_at: u64,
    /// True if the handler name matches a registered handler on this server.
    pub handler_registered: bool,
}

/// Server health and uptime.
#[derive(SimpleObject)]
#[graphql(name = "Health")]
pub struct GqlHealth {
    pub status: String,
    /// Seconds since server start.
    pub uptime_secs: u64,
    pub running: bool,
}

/// Input for creating a new cron job.
#[derive(InputObject)]
pub struct CreateCronJobInput {
    pub schedule: String,
    pub handler: String,
    pub metadata: Option<JSON>,
    /// Whether the job is active. Defaults to `true`.
    #[graphql(default = true)]
    pub enabled: bool,
}

/// Input for partially updating an existing cron job.
#[derive(InputObject)]
pub struct UpdateCronJobInput {
    pub schedule: Option<String>,
    pub handler: Option<String>,
    pub metadata: Option<JSON>,
    pub enabled: Option<bool>,
}
