// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use shared::JiraIssue;
use tauri::{State, api::{http::{ClientBuilder, Client, HttpRequestBuilder}, self}, Error, http::header::AUTHORIZATION};
use serde_json::Value;
use http_auth_basic::Credentials;
use serde::{Deserialize, Serialize};

struct Configuration {
    base_path: String,
    user: String,
    api_key: String,
    client: Client,
    credentials: Credentials
}

struct JiraState {
    config: Configuration,
}

#[tauri::command]
fn get_mine_issues(jira_state: State<JiraState>) -> String {
    "xd".to_string()
}

#[tauri::command(async)]
async fn get_issue(key: String, jira_state: State<'_, JiraState>) -> Result<JiraIssue, String> {
    let link = format!("{}/rest/api/3/issue/{}", jira_state.config.base_path, key);
    let request = HttpRequestBuilder::new(
        "GET", 
        link,
    ).map_err(|err| {err.to_string()})?;

    let request = request.header( AUTHORIZATION ,jira_state.config.credentials.as_http_header()).map_err(|err| {err.to_string()})?;
    let response = jira_state.config.client.send(request).await.map_err(|err| {err.to_string()})?.read().await.map_err(|err| {err.to_string()})?;

    Ok(JiraIssue{ 
        id: Box::new(response.data["id"].to_string()), 
        key: Box::new(key), 
        url: Box::new(response.data["self"].to_string()), 
        summary: Box::new(response.data["fields"]["summary"].to_string()), 
        assignee_email: Box::new(response.data["fields"]["assignee"]["emailAddress"].to_string()), 
        time_tracked_all: response.data["fields"]["timespent"].as_i64().map_or(0, |v| v)
    })
}



fn setup_jira() -> Result<Configuration, Error>{
    let client_builder = ClientBuilder::new();
    let client = client_builder.build()?;
    let base_path = std::env::var("JIRA_URL").map_err(|err| { Error::AssetNotFound(err.to_string())})?;
    let user = std::env::var("JIRA_USER").map_err(|err| { Error::AssetNotFound(err.to_string())})?;
    let api_key = std::env::var("JIRA_KEY").map_err(|err| { Error::AssetNotFound(err.to_string())})?;
    let credentials = Credentials::new(user.as_str(), api_key.as_str());

    let config = Configuration{
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
        .manage(JiraState{config: jira_cfg})
        .invoke_handler(tauri::generate_handler![get_issue])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
