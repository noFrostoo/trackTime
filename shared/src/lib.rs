use std::fmt::Display;

use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, ImplicitClone, Clone)]
pub struct JiraIssue {
    pub id: Box<String>,
    pub key: Box<String>,
    pub url: Box<String>,
    pub summary: Box<String>,
    pub assignee_email: Box<String>,
    pub time_tracked_all: i64,
}

impl JiraIssue {
    pub fn new(id: Box<String>, key: Box<String>, url: Box<String>, summary: Box<String>, assignee_email: Box<String>, time_tracked_all: i64) -> Self { Self { id, key, url, summary, assignee_email, time_tracked_all } }
    pub fn empty() -> Self { 
        Self { 
            id: Box::new(String::new()), 
            key: Box::new(String::new()), 
            url: Box::new(String::new()), 
            summary: Box::new(String::new()), 
            assignee_email: Box::new(String::new()), 
            time_tracked_all: 0 } 
    }
}

impl Display for JiraIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.key, self.summary)
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
