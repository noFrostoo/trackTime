use http_auth_basic::Credentials;
use shared::Issue;
use tauri::{
    api::http::{Client, ClientBuilder, HttpRequestBuilder},
    http::header::AUTHORIZATION,
    Error, State,
};

use crate::AppState;

pub struct JiraConfiguration {
    pub base_path: String,
    pub user: String,
    pub api_key: String,
    pub client: Client,
    pub credentials: Credentials,
}

pub fn setup_jira() -> Result<JiraConfiguration, Error> {
    let client_builder = ClientBuilder::new();
    let client = client_builder.build()?;
    let base_path =
        std::env::var("JIRA_URL").map_err(|err| Error::AssetNotFound(err.to_string()))?;
    let user = std::env::var("JIRA_USER").map_err(|err| Error::AssetNotFound(err.to_string()))?;
    let api_key = std::env::var("JIRA_KEY").map_err(|err| Error::AssetNotFound(err.to_string()))?;
    let credentials = Credentials::new(user.as_str(), api_key.as_str());

    let config = JiraConfiguration {
        base_path,
        client,
        api_key,
        user,
        credentials,
    };

    Ok(config)
}

#[tauri::command(async)]
pub async fn get_issue_jira(name: String, app_state: State<'_, AppState>) -> Result<Issue, String> {
    let link = format!(
        "{}/rest/api/3/issue/{}",
        app_state.jira_config.base_path, name
    );
    let request = HttpRequestBuilder::new("GET", link).map_err(|err| err.to_string())?;

    let request = request
        .header(
            AUTHORIZATION,
            app_state.jira_config.credentials.as_http_header(),
        )
        .map_err(|err| err.to_string())?;
    let response = app_state
        .jira_config
        .client
        .send(request)
        .await
        .map_err(|err| err.to_string())?
        .read()
        .await
        .map_err(|err| err.to_string())?;

    Ok(Issue {
        id: response.data["id"].to_string(),
        name: name,
        url: response.data["self"].to_string(),
        summary: response.data["fields"]["summary"].to_string(),
        assignee_email: response.data["fields"]["assignee"]["emailAddress"].to_string(),
        time_tracked_all: response.data["fields"]["timespent"]
            .as_u64()
            .map_or(0, |v| v),
    })
}
