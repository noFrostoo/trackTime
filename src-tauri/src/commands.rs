use crate::{storage::Storage, AppState, MangedState, TracingInfo};
use shared::{Issue, Worklog};
use std::{borrow::BorrowMut, sync::{mpsc::channel, Arc}, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use tauri::State;
use tokio::sync::MutexGuard;
use uuid::Uuid;
use tokio::runtime::Handle;
const MAX_RECENT: i32 = 5;


#[tauri::command(async)]
pub async fn get_issues(app_state: State<'_, Arc<MangedState>>) -> Result<Vec<Issue>, String> {
    let m = app_state.0.lock().await;
    m.storage.get_issues().await
}

#[tauri::command(async)]
pub async fn add_issue(
    name: String,
    summary: String,
    app_state: State<'_, Arc<MangedState>>,
) -> Result<Issue, String> {
    let m = app_state.0.lock().await;
    let issue = Issue {
        id: Uuid::new_v4().to_string(),
        name: name,
        url: "".to_string(),
        summary: summary,
        assignee_email: m.jira_config.user.clone(),
        time_tracked_all: 0,
    };

    m.storage.add_issue(issue).await
}

#[tauri::command(async)]
pub async fn get_tracing_issue_name(
    app_state: State<'_, Arc<MangedState>>,
) -> Result<Option<String>, String> {
    let m = app_state.0.lock().await;
    match &m.tracing_info {
        Some(info) => Ok(Some(info.time_tracing_issue.name.clone())),
        None => Ok(None),
    }
}

#[tauri::command(async)]
pub async fn get_elapsed_time(app_state: State<'_, Arc<MangedState>>) -> Result<Duration, String> {
    let m = app_state.0.lock().await;
    match &m.tracing_info {
        Some(info) => Ok(info.start_time.elapsed()),
        None => Ok(Duration::from_secs(0)),
    }
}

#[tauri::command(async)]
pub async fn start_tracking_cmd(
    name: String,
    app_state: State<'_, Arc<MangedState>>,
    app_handle: tauri::AppHandle
) -> Result<(), String> {
    println!("Start tracing {}", name);
    stop_tracking(&app_state).await?;
    start_tracking(name, &app_state, app_handle).await
}

#[tauri::command(async)]
pub async fn stop_tracking_cmd(app_state: State<'_, Arc<MangedState>>) -> Result<(), String> {
    stop_tracking(&app_state).await
}

pub async fn start_tracking(
    name: String,
    app_state: &State<'_, Arc<MangedState>>,
    app_handle: tauri::AppHandle
) -> Result<(), String> {
    let mut m = app_state.0.lock().await;

    if let Some(_) = m.tracing_info {
        return Err("Issue already tracked".to_string());
    }

    let issue = m.storage.get_issue(name).await?;

    let start_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => return Err("time went backwards".to_string()),
    };

    let worklog = Worklog {
        id: Uuid::new_v4().to_string(),
        issue_id: issue.id.clone(),
        start: start_time,
        end: 0,
        total_time: 0,
    };

    m.storage.add_worklog(worklog.clone()).await?;

    add_to_recent_issues(&issue.name, m.borrow_mut(), app_handle);

    let tracing_info = TracingInfo {
        time_tracing_issue: issue,
        current_worklog: worklog,
        start_time: Instant::now(),
    };

    m.tracing_info = Some(tracing_info);
    Ok(())
}

pub async fn stop_tracking(app_state: &State<'_, Arc<MangedState>>) -> Result<(), String> {
    let mut m = app_state.0.lock().await;

    match m.tracing_info.clone() {
        Some(mut tracing_info) => {
            let duration = tracing_info.start_time.elapsed();

            let end_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(d) => d.as_secs(),
                Err(_) => return Err("time went backwards".to_string()),
            };

            tracing_info.current_worklog.end = end_time;
            tracing_info.current_worklog.total_time = duration.as_secs();

            let mut issue = m
                .storage
                .get_issue(tracing_info.time_tracing_issue.name)
                .await?;
            issue.time_tracked_all += duration.as_secs();

            m.storage.edit_issue(issue).await?;
            m.storage.edit_worklog(tracing_info.current_worklog).await?;
        }
        None => return Ok(()),
    }

    m.tracing_info = None;

    Ok(())
}

fn add_to_recent_issues(issue_name: &String, state_mutex: &mut MutexGuard<AppState>, app_handle: tauri::AppHandle) {
    state_mutex.recent_issues.push_back(issue_name.clone());
    if state_mutex.recent_issues.len() == MAX_RECENT as usize {
        state_mutex.recent_issues.pop_front().unwrap();
    }

    for (i ,issue) in state_mutex.recent_issues.iter().enumerate() {
        app_handle.tray_handle().get_item(i.to_string().as_str()).set_title(issue).unwrap();
    }
    
}


pub async fn save_progress(app_state: Arc<MangedState>) -> Result<(), String> {
    let mut m = app_state.0.lock().await;
    save_tracing(&m).await?;
    m.tracing_info = None;
    Ok(())
}

async fn save_tracing<'a>(m: &MutexGuard<'a, AppState>) -> Result<(), String> {
    Ok(match m.tracing_info.clone() {
        Some(mut tracing_info) => {
            let duration = tracing_info.start_time.elapsed();
        
            let end_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(d) => d.as_secs(),
                Err(_) => return Err("time went backwards".to_string()),
            };
        
            tracing_info.current_worklog.end = end_time;
            tracing_info.current_worklog.total_time = duration.as_secs();
        
            let mut issue = m
                .storage
                .get_issue(tracing_info.time_tracing_issue.name)
                .await?;
            issue.time_tracked_all += duration.as_secs();
        
            m.storage.edit_issue(issue).await?;
            m.storage.edit_worklog(tracing_info.current_worklog).await?;
        }
        None => return Ok(()),
    })
}


pub fn exit_save_process(app_state: Arc<MangedState>) -> Result<(), String> {
    let handle = Handle::current();
    let (tx, rx) = channel();
    std::thread::spawn( move || {
        handle.block_on(async {
            let mut m = app_state.0.lock().await;

            let mut res = save_tracing(&m).await;
            if res.is_err() {
                return;
            }

            res = m.storage.save_recent_issues(m.recent_issues.clone()).await;
            m.tracing_info = None;
            tx.send(res).unwrap();
          })
    });

    let res = rx.recv();
    match res {
        Ok(res) => res,
        Err(e) => Err(e.to_string()),
    }
}