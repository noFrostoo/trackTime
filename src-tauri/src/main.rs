// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use shared::Issue;
use tauri::{State, api::{http::{ClientBuilder, Client, HttpRequestBuilder}, self}, Error, http::header::AUTHORIZATION};
use serde_json::Value;
use http_auth_basic::Credentials;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

struct JiraConfiguration {
    base_path: String,
    user: String,
    api_key: String,
    client: Client,
    credentials: Credentials
}

struct AppState {
    jira_config: JiraConfiguration,
    issues: Vec<Issue>
}

struct MangedState(Mutex<AppState>);

#[tauri::command]
fn get_mine_issues(jira_state: State<AppState>) -> String {
    "xd".to_string()
}

#[tauri::command(async)]
async fn get_issue(name: String, app_state: State<'_, AppState>) -> Result<Issue, String> {
    let link = format!("{}/rest/api/3/issue/{}", app_state.jira_config.base_path, name);
    let request = HttpRequestBuilder::new(
        "GET", 
        link,
    ).map_err(|err| {err.to_string()})?;

    let request = request.header( AUTHORIZATION ,app_state.jira_config.credentials.as_http_header()).map_err(|err| {err.to_string()})?;
    let response = app_state.jira_config.client.send(request).await.map_err(|err| {err.to_string()})?.read().await.map_err(|err| {err.to_string()})?;

    Ok(Issue{ 
        id: Box::new(response.data["id"].to_string()), 
        name: Box::new(name), 
        url: Box::new(response.data["self"].to_string()), 
        summary: Box::new(response.data["fields"]["summary"].to_string()), 
        assignee_email: Box::new(response.data["fields"]["assignee"]["emailAddress"].to_string()), 
        time_tracked_all: response.data["fields"]["timespent"].as_i64().map_or(0, |v| v)
    })
}

#[tauri::command(async)]
async fn get_issues(app_state: State<'_, MangedState>) -> Result<Vec<Issue>, String> {
    match app_state.0.lock() {
        Ok(m) => {
            Ok(m.issues.clone())
        },
        Err(e) => {Err(e.to_string())},
    }
}

#[tauri::command(async)]
async fn add_issue(name: String, summary: String, app_state: State<'_, MangedState>) -> Result<(), String> {
    match app_state.0.lock() {
        Ok(mut m) => {
            let issue = Issue{
                id: Box::new(Uuid::new_v4().to_string()),
                name: Box::new(name),
                url: Box::new("".to_string()),
                summary: Box::new(summary),
                assignee_email: Box::new(m.jira_config.user.clone()),
                time_tracked_all: 0,
            };
        
            m.issues.push(issue);
            Ok(())
        },
        Err(e) => {Err(e.to_string())},
    }
}




fn setup_jira() -> Result<JiraConfiguration, Error>{
    let client_builder = ClientBuilder::new();
    let client = client_builder.build()?;
    let base_path = std::env::var("JIRA_URL").map_err(|err| { Error::AssetNotFound(err.to_string())})?;
    let user = std::env::var("JIRA_USER").map_err(|err| { Error::AssetNotFound(err.to_string())})?;
    let api_key = std::env::var("JIRA_KEY").map_err(|err| { Error::AssetNotFound(err.to_string())})?;
    let credentials = Credentials::new(user.as_str(), api_key.as_str());

    let config = JiraConfiguration{
        base_path,
        client,
        api_key,
        user,
        credentials
    };

    Ok(config)
}

fn main() {
    let jira_cfg = setup_jira().expect("Error while setting up jira");
    tauri::Builder::default()
        .manage(MangedState{0: Mutex::new(AppState{jira_config: jira_cfg, issues: Vec::new() })})
        .invoke_handler(tauri::generate_handler![get_issue, get_issues, add_issue])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
