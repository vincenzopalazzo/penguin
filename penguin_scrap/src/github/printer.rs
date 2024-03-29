use crate::github::model::NewIssue;
use crate::Printer;
use std::vec::Vec;

pub struct MDPrinter {
    created: String,
    team: String,
    labels: Vec<String>,
    // FIXME: add zulip stream
}

impl Printer for MDPrinter {
    type Input = NewIssue;

    fn new(created: &str, team: &str, labels: &[String]) -> Self {
        MDPrinter {
            created: created.to_owned(),
            team: team.to_owned(),
            labels: labels.to_owned(),
        }
    }

    fn printify<'a>(&'a self, issues: impl Iterator<Item = &'a Self::Input>) -> String {
        let mut content = String::new();
        content += format!("# Triage Meeting {}\n\n", self.created).as_str();
        content += format!("- Owner: {}\n", self.team).as_str();
        content += format!("- Date: {}\n", self.created).as_str();
        content += "\n\n";
        content += "## Announcements and Introductions\n\n";

        content += "\n\n";
        content += "## New Issues\n";
        for issue in issues {
            let mut labels = String::new();
            for label in &self.labels {
                if issue.with_label(label) {
                    labels += format!("**{label}**,").as_str();
                }
            }
            debug_assert!(
                !labels.is_empty(),
                "the labels that we are looking for are not in this issue!"
            );

            content += format!(
                "  - [{}]({}) in date {}\n",
                issue.title, issue.html_url, issue.created_at
            )
            .as_str();

            content += "    - with labels: ";
            content += labels.as_str();
            content = content.strip_suffix(',').unwrap().to_string();
            content += "\n";
            content += format!("    - assigned to {}\n", issue.assigned_to_str()).as_str();
        }
        content
    }
}
