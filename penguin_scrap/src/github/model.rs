use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewIssue {
    pub html_url: String,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub labels: Vec<IssueLabel>,
    pub assignees: Vec<IssueAssigned>,
    pub created_at: String,
}

impl NewIssue {
    pub fn with_label(&self, label_to_find: &str) -> bool {
        let res: Vec<IssueLabel> = self
            .labels
            .clone()
            .into_iter()
            .filter(|label| label.name == label_to_find)
            .collect();
        !res.is_empty()
    }

    pub fn assigned_to_str(&self) -> String {
        self.assignees
            .iter()
            .map(|user| format!("*@{user}* "))
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueLabel {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueAssigned {
    login: String,
}

impl fmt::Display for IssueAssigned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.login)
    }
}
