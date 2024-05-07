use std::{borrow::Borrow, collections::VecDeque, num::ParseIntError};

use serde::{Deserialize, Serialize};
use shared::{Issue, Worklog};
use sqlx::{Pool, Sqlite};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WorklogDB {
    pub id: String,
    pub issue_id: String,
    pub start: String,
    pub end: String,
    pub total_time: String,
}

impl TryFrom<WorklogDB> for Worklog {
    type Error = ParseIntError;

    fn try_from(value: WorklogDB) -> Result<Self, Self::Error> {
        let start = match value.start.parse::<u64>() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let end = match value.end.parse::<u64>() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let total_time = match value.total_time.parse::<u64>() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok(Worklog {
            id: value.id,
            issue_id: value.issue_id,
            start: start,
            end: end,
            total_time: total_time,
        })
    }
}

impl Into<WorklogDB> for Worklog {
    fn into(self) -> WorklogDB {
        WorklogDB {
            id: self.id,
            issue_id: self.issue_id,
            start: self.start.to_string(),
            end: self.end.to_string(),
            total_time: self.total_time.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct IssueDb {
    pub id: String,
    pub name: String, // key in jira
    pub url: String,
    pub summary: String,
    pub assignee_email: String,
    pub time_tracked_all: String,
}

impl TryFrom<IssueDb> for Issue {
    type Error = ParseIntError;

    fn try_from(value: IssueDb) -> Result<Self, Self::Error> {
        let time = match value.time_tracked_all.parse::<u64>() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok(Issue {
            id: value.id,
            name: value.name,
            url: value.url,
            summary: value.summary,
            assignee_email: value.assignee_email,
            time_tracked_all: time,
        })
    }
}

impl Into<IssueDb> for Issue {
    fn into(self) -> IssueDb {
        IssueDb {
            id: self.id,
            name: self.name,
            url: self.url,
            summary: self.summary,
            assignee_email: self.assignee_email,
            time_tracked_all: self.time_tracked_all.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Storage {
    pub conn: Pool<Sqlite>,
}

impl Storage {
    pub async fn get_issue(&self, name: String) -> Result<Issue, String> {
        let val = sqlx::query_as!(
            IssueDb,
            r#"
            select id, name, url, summary, assignee_email, time_tracked_all
            from issue
            where name = $1
            "#,
            name
        )
        .fetch_one(&self.conn)
        .await
        .map_err(|e| e.to_string());

        convert_to_issue(val)
    }

    pub async fn get_issues(&self) -> Result<Vec<Issue>, String> {
        let values = sqlx::query_as!(
            IssueDb,
            r#"
            select id, name, url, summary, assignee_email, time_tracked_all
            from issue
            "#
        )
        .fetch_all(&self.conn)
        .await
        .map_err(|e| e.to_string());

        match values {
            Ok(values) => {
                let mut exit_vales = Vec::new();
                for val in values {
                    match Issue::try_from(val) {
                        Ok(val) => exit_vales.push(val),
                        Err(e) => return Err(e.to_string()),
                    }
                }
                Ok(exit_vales)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn add_issue(&self, given_issue: Issue) -> Result<Issue, String> {
        let issue: IssueDb = given_issue.into();
        let val = sqlx::query_as!(
            IssueDb,
            r#"
            insert into issue(id, name, url, summary, assignee_email, time_tracked_all)
            values($1,$2,$3,$4,$5,$6)
            returning id, name, url, summary, assignee_email, time_tracked_all
            "#,
            issue.id,
            issue.name,
            issue.url,
            issue.summary,
            issue.assignee_email,
            issue.time_tracked_all
        )
        .fetch_one(&self.conn)
        .await
        .map_err(|e| e.to_string());

        convert_to_issue(val)
    }

    pub async fn edit_issue(&self, given_issue: Issue) -> Result<Issue, String> {
        let issue: IssueDb = given_issue.into();
        let val = sqlx::query_as!(
            IssueDb,
            r#"
            update issue
            set name = $2, url = $3, summary = $4, assignee_email = $5, time_tracked_all = $6
            where id = $1
            returning id, name, url, summary, assignee_email, time_tracked_all
            "#,
            issue.id,
            issue.name,
            issue.url,
            issue.summary,
            issue.assignee_email,
            issue.time_tracked_all
        )
        .fetch_one(&self.conn)
        .await
        .map_err(|e| e.to_string());

        convert_to_issue(val)
    }

    pub async fn add_worklog(&self, given_worklog: Worklog) -> Result<Worklog, String> {
        let worklog: WorklogDB = given_worklog.into();
        let val = sqlx::query_as!(
            WorklogDB,
            r#"
            insert into worklog(id, issue_id, start, end, total_time)
            values($1,$2,$3,$4,$5)
            returning id, issue_id, start, end, total_time
            "#,
            worklog.id,
            worklog.issue_id,
            worklog.start,
            worklog.end,
            worklog.total_time,
        )
        .fetch_one(&self.conn)
        .await
        .map_err(|e| e.to_string());

        convert_to_worklog(val)
    }

    pub async fn edit_worklog(&self, given_worklog: Worklog) -> Result<Worklog, String> {
        let worklog: WorklogDB = given_worklog.into();
        let val = sqlx::query_as!(
            WorklogDB,
            r#"
            update worklog
            set start = $2, end = $3, total_time = $4
            where id = $1
            returning id, issue_id, start, end, total_time
            "#,
            worklog.id,
            worklog.start,
            worklog.end,
            worklog.total_time,
        )
        .fetch_one(&self.conn)
        .await
        .map_err(|e| e.to_string());

        convert_to_worklog(val)
    }

    pub async fn save_recent_issues(&self, recent_issues: VecDeque<String>) -> Result<(), String> {
        sqlx::query!(
            r#"
            delete from recent_issue
            "#
        )
        .execute(&self.conn)
        .await
        .map_err(|e| e.to_string())?;


        for issue in recent_issues.iter() {
            sqlx::query!(
                r#"
                insert into recent_issue(name) values ($1) 
                "#,
                issue
            )
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub async fn get_recent_issues(&self) -> Result<VecDeque<String>, String> {
        let values = sqlx::query!(
            r#"
            select name from recent_issue
            "#,
        )
        .fetch_all(&self.conn)
        .await
        .map_err(|e| e.to_string())?;
        
        let mut recent_issues = VecDeque::new();
        for record in values.iter() {
            recent_issues.push_back(record.name.clone())
        }

        Ok(recent_issues)
    }
}

fn convert_to_issue(val: Result<IssueDb, String>) -> Result<Issue, String> {
    match val {
        Ok(val) => match Issue::try_from(val) {
            Ok(issue) => Ok(issue),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e),
    }
}

fn convert_to_worklog(val: Result<WorklogDB, String>) -> Result<Worklog, String> {
    match val {
        Ok(val) => match Worklog::try_from(val) {
            Ok(issue) => Ok(issue),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e),
    }
}
