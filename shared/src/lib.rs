use std::fmt::Display;

use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, ImplicitClone, Clone)]
pub struct Issue {
    pub id: String,
    pub name: String, // key in jira
    pub url: String,
    pub summary: String,
    pub assignee_email: String,
    pub time_tracked_all: u64,
}

impl Issue {
    pub fn new(
        id: String,
        name: String,
        url: String,
        summary: String,
        assignee_email: String,
        time_tracked_all: u64,
    ) -> Self {
        Self {
            id,
            name,
            url,
            summary,
            assignee_email,
            time_tracked_all,
        }
    }
    pub fn empty() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            url: String::new(),
            summary: String::new(),
            assignee_email: String::new(),
            time_tracked_all: 0,
        }
    }
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.name, self.summary)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ImplicitClone, Clone)]
pub struct Worklog {
    pub id: String,
    pub issue_id: String, // key in jira
    pub start: u64,
    pub end: u64,
    pub total_time: u64,
}

impl Worklog {
    pub fn new(id: String, issue_id: String, start: u64, end: u64, total_time: u64) -> Self {
        Self {
            id,
            issue_id,
            start,
            end,
            total_time,
        }
    }

    pub fn empty() -> Self {
        Self {
            id: "".to_string(),
            issue_id: "".to_string(),
            start: 0,
            end: 0,
            total_time: 0,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
