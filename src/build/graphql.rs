use async_graphql::{Context, EmptySubscription, Object, Schema};
use std::fs;
use std::path::Path;

#[path = "../graphql_types.rs"]
mod graphql_types;
use graphql_types::{
    CreateCronJobInput, GqlCronJob, GqlCronJobResponse, GqlHealth, UpdateCronJobInput,
};

// Stub resolvers — never called, exist only to satisfy async-graphql's type registration so
// schema.sdl() produces the correct SDL. Signatures must mirror src/routes/graphql.rs exactly.

struct Query;

#[Object]
impl Query {
    /// List all managed cron jobs with handler registration status.
    async fn cron_jobs(&self, _ctx: &Context<'_>) -> Vec<GqlCronJobResponse> {
        vec![]
    }

    /// Get a managed cron job by ID; returns null if not found.
    async fn cron_job(&self, _ctx: &Context<'_>, _id: String) -> Option<GqlCronJobResponse> {
        None
    }

    /// List read-only system cron jobs from crontab and /etc/cron.d.
    async fn system_cron_jobs(&self) -> Vec<GqlCronJob> {
        vec![]
    }

    /// Server health and uptime.
    async fn health(&self, _ctx: &Context<'_>) -> GqlHealth {
        GqlHealth { status: String::new(), uptime_secs: 0, running: false }
    }
}

struct Mutation;

#[Object]
impl Mutation {
    /// Create a new managed cron job.
    async fn create_cron_job(
        &self,
        _ctx: &Context<'_>,
        _input: CreateCronJobInput,
    ) -> async_graphql::Result<GqlCronJob> {
        Err(async_graphql::Error::new("stub"))
    }

    /// Update one or more fields of an existing managed cron job.
    async fn update_cron_job(
        &self,
        _ctx: &Context<'_>,
        _id: String,
        _input: UpdateCronJobInput,
    ) -> async_graphql::Result<GqlCronJob> {
        Err(async_graphql::Error::new("stub"))
    }

    /// Delete a managed cron job by ID; returns the deleted job ID.
    async fn delete_cron_job(
        &self,
        _ctx: &Context<'_>,
        _id: String,
    ) -> async_graphql::Result<String> {
        Err(async_graphql::Error::new("stub"))
    }
}

pub fn generate(manifest_dir: &str) {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
    let out_path = Path::new(manifest_dir).join("apis/graphql.graphql");
    fs::write(&out_path, schema.sdl()).expect("failed to write apis/graphql.graphql");
}
