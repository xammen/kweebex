use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::rt::TokioExecutor;

mod fetch;

pub use fetch::*;

pub async fn init_client() -> Option<clickhouse::Client> {
    let database = dotenvy::var("CLICKHOUSE_DATABASE").ok()?;
    init_client_with_database(&database).await.ok()
}

pub async fn init_client_with_database(
    database: &str,
) -> clickhouse::error::Result<clickhouse::Client> {
    let url = dotenvy::var("CLICKHOUSE_URL")
        .map_err(|_| clickhouse::error::Error::Custom("CLICKHOUSE_URL not set".into()))?;
    let user = dotenvy::var("CLICKHOUSE_USER")
        .map_err(|_| clickhouse::error::Error::Custom("CLICKHOUSE_USER not set".into()))?;
    let password = dotenvy::var("CLICKHOUSE_PASSWORD")
        .map_err(|_| clickhouse::error::Error::Custom("CLICKHOUSE_PASSWORD not set".into()))?;

    let client = {
        let https_connector = HttpsConnectorBuilder::new()
            .with_native_roots()?
            .https_or_http()
            .enable_all_versions()
            .build();
        let hyper_client =
            hyper_util::client::legacy::Client::builder(TokioExecutor::new())
                .build(https_connector);

        clickhouse::Client::with_http_client(hyper_client)
            .with_url(url)
            .with_user(user)
            .with_password(password)
            .with_validation(false)
    };

    client
        .query(&format!("CREATE DATABASE IF NOT EXISTS {database}"))
        .execute()
        .await?;

    let clickhouse_replicated =
        dotenvy::var("CLICKHOUSE_REPLICATED").unwrap_or_default() == "true";
    let cluster_line = if clickhouse_replicated {
        "ON cluster '{cluster}'"
    } else {
        ""
    };

    let engine = if clickhouse_replicated {
        "ReplicatedMergeTree('/clickhouse/{installation}/{cluster}/tables/{shard}/{database}/{table}', '{replica}')"
    } else {
        "MergeTree()"
    };

    // For the Clickhouse database on the staging environment, set a TTL to avoid accumulating too much data
    let ttl = if database == "staging_analytics" {
        "TTL toDateTime(recorded) + INTERVAL 1 DAY"
    } else {
        ""
    };

    client
        .query(&format!(
            "
            CREATE TABLE IF NOT EXISTS {database}.views {cluster_line}
            (
                recorded DateTime64(4),
                domain String,
                site_path String,

                user_id UInt64,
                project_id UInt64,
                monetized Bool DEFAULT True,

                ip IPv6,
                country String,
                user_agent String,
                headers Array(Tuple(String, String))
            )
            ENGINE = {engine}
            {ttl}
            PRIMARY KEY (project_id, recorded, ip)
            SETTINGS index_granularity = 8192
            "
        ))
        .execute()
        .await?;

    client
        .query(&format!(
            "
            CREATE TABLE IF NOT EXISTS {database}.downloads {cluster_line}
            (
                recorded DateTime64(4),
                domain String,
                site_path String,

                user_id UInt64,
                project_id UInt64,
                version_id UInt64,

                ip IPv6,
                country String,
                user_agent String,
                headers Array(Tuple(String, String))
            )
            ENGINE = {engine}
            {ttl}
            PRIMARY KEY (project_id, recorded, ip)
            SETTINGS index_granularity = 8192
            "
        ))
        .execute()
        .await?;

    client
        .query(&format!(
            "
            CREATE TABLE IF NOT EXISTS {database}.playtime {cluster_line}
            (
                recorded DateTime64(4),
                seconds UInt64,

                user_id UInt64,
                project_id UInt64,
                version_id UInt64,

                loader String,
                game_version String,
                parent UInt64
            )
            ENGINE = {engine}
            {ttl}
            PRIMARY KEY (project_id, recorded, user_id)
            SETTINGS index_granularity = 8192
            "
        ))
        .execute()
        .await?;

    client
        .query(&format!(
            "
            CREATE TABLE IF NOT EXISTS {database}.affiliate_code_clicks {cluster_line}
            (
                recorded DateTime64(4),
                domain String,

                user_id UInt64,
                affiliate_code_id UInt64,

                ip IPv6,
                country String,
                user_agent String,
                headers Array(Tuple(String, String))
            )
            ENGINE = {engine}
            {ttl}
            PRIMARY KEY (affiliate_code_id, recorded)
            SETTINGS index_granularity = 8192
            "
        ))
        .execute()
        .await?;

    Ok(client.with_database(database))
}
