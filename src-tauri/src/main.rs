// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::exit_save_process;
use crate::jira::{get_issue_jira, setup_jira, JiraConfiguration};
use crate::storage::Storage;
use shared::{Issue, Worklog};
use sqlx::sqlite::SqlitePoolOptions;
use std::collections::VecDeque;
use std::sync::Arc;
use std::{
    env::{self},
    time::Instant,
};
use tauri::{AppHandle, CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tokio::sync::Mutex;

mod commands;
mod jira;
mod storage;

struct AppState {
    jira_config: JiraConfiguration,
    tracing_info: Option<TracingInfo>,
    storage: Storage,
    recent_issues: VecDeque<String>,
}

#[derive(Clone)]
struct TracingInfo {
    time_tracing_issue: Issue,
    current_worklog: Worklog,
    start_time: Instant,
}

struct MangedState(Mutex<AppState>);


fn set_up_tray(recent_issues: &VecDeque<String>) -> SystemTrayMenu {
  let mut tray_menu = SystemTrayMenu::new()
  .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
  .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(CustomMenuItem::new("hide".to_string(), "Hide"))
  .add_item(CustomMenuItem::new("show".to_string(), "Show"))
  .add_native_item(SystemTrayMenuItem::Separator);

  let fill_in_count = 5 - recent_issues.len();

  for (i, issue) in recent_issues.iter().enumerate() {
    tray_menu = tray_menu.add_item(CustomMenuItem::new(i.to_string(), issue.clone()));
  }

  for i in 0..fill_in_count {
    tray_menu = tray_menu.add_item(CustomMenuItem::new(i.to_string(), format!("Recent Issue: {} ", i)));
    
  }

  tray_menu
}



fn handle_tray_event<'a>(app: &'a AppHandle, event: SystemTrayEvent) {
  match event {
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "quit" => {
            let s:State<Arc<MangedState>> = app.state();
            exit_save_process(s.inner().clone()).expect("Could not save the progress");
            print!("closing app from tray click");
            app.exit(0);
        }
        "hide" => {
            let window = app.get_window("main").unwrap();

            match window.hide() {
                Ok(_) => print!("window hidden"),
                Err(e) => print!("window cannot be hidden:  {}", e),
            }
        }
        "show" => {
          let window = app.get_window("main").unwrap();

          match window.show() {
              Ok(_) => print!("window show"),
              Err(e) => print!("window cannot be shown:  {}", e),
          }
        }
        "0" => {
          
        }
        "1" => {
          
        }
        "2" => {
          
        }
        "3" => {
          
        }
        "4" => {
          
        }
        _ => {}
    },
    _ => {}
  }
}


#[tokio::main]
async fn main() {
    let setup_jira = setup_jira().expect("Error while setting up jira");
    let jira_cfg = setup_jira;

    let database_url: String = match env::var("DATABASE_URL") {
        Ok(v) => v,
        Err(_) => "storage".to_string(),
    };

    let conn = SqlitePoolOptions::new()
        .max_connections(3)
        .connect(&database_url)
        .await
        .expect("failed to connect to storage");
    sqlx::migrate!().run(&conn).await.expect("Couldn't init db");

    let storage = Storage { conn };
    let recent_issues = storage.get_recent_issues().await.expect("Error getting recent issues");
      
    let tray_menu = set_up_tray(&recent_issues);
    let system_tray = SystemTray::new().with_menu(tray_menu);



    tauri::Builder::default()
        .manage(Arc::new(MangedState {
            0: Mutex::new(AppState {
                jira_config: jira_cfg,
                tracing_info: None,
                storage: storage,
                recent_issues: recent_issues,
            }),
        }))
        .system_tray(system_tray)
        .on_system_tray_event(handle_tray_event)
        .on_window_event(|event| match event.event() {
          tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
          }
          _ => {}
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
