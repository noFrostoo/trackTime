// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env::{self}, time::Instant
};
use crate::jira::{setup_jira, JiraConfiguration, get_issue_jira};
use crate::storage::Storage;
use sqlx::sqlite::SqlitePoolOptions;
use shared::{Issue, Worklog};
use tokio::sync::Mutex;

mod jira;
mod storage;
mod commands;

struct AppState {
    jira_config: JiraConfiguration,
    tracing_info: Option<TracingInfo>,
    storage: Storage,
}

#[derive(Clone)]
struct TracingInfo {
    time_tracing_issue: Issue,
    current_worklog: Worklog,
    start_time: Instant,
}

struct MangedState(Mutex<AppState>);



#[tokio::main]
async fn main() {
    let jira_cfg = setup_jira().expect("Error while setting up jira");
    
    let database_url: String = match env::var("DATABASE_URL") {
        Ok(v) => v,
        Err(_) => "storage".to_string(),
    };
    
    let conn = SqlitePoolOptions::new().max_connections(3).connect(&database_url).await.expect("failed to connect to storage");
    sqlx::migrate!().run(&conn).await.expect("Couldn't init db");
    
    tauri::Builder::default()
        .manage(MangedState {
            0: Mutex::new(AppState {
                jira_config: jira_cfg,
                tracing_info: None,
                storage: Storage{conn}
            }),
        })
        .invoke_handler(tauri::generate_handler![
            get_issue_jira,
            commands::get_issues,
            commands::add_issue,
            commands::start_tracking_cmd,
            commands::stop_tracking_cmd,
            commands::get_tracing_issue_name,
            commands::get_elapsed_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
