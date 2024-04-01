use std::fmt::Display;

use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, ImplicitClone, Clone)]
pub struct Issue {
    pub id: Box<String>,
    pub name: Box<String>, // key in jira
    pub url: Box<String>,
    pub summary: Box<String>,
    pub assignee_email: Box<String>,
    pub time_tracked_all: i64,
}

impl Issue {
    pub fn new(id: Box<String>, name: Box<String>, url: Box<String>, summary: Box<String>, assignee_email: Box<String>, time_tracked_all: i64) -> Self { Self { id, name, url, summary, assignee_email, time_tracked_all } }
    pub fn empty() -> Self { 
        Self { 
            id: Box::new(String::new()), 
            name: Box::new(String::new()), 
            url: Box::new(String::new()), 
            summary: Box::new(String::new()), 
            assignee_email: Box::new(String::new()), 
            time_tracked_all: 0 } 
    }
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.name, self.summary)
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
