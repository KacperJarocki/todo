use chrono::{DateTime, Local};
use chrono_humanize::HumanTime;
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    id: u64,
    description: String,
    done: bool,
    created_at: DateTime<Local>,
    last_updated: Option<DateTime<Local>>,
}
impl Task {
    pub fn new(id: u64, description: &str) -> Task {
        Task {
            id: 0,
            description: description.to_string(),
            done: false,
            created_at: Local::now(),
            last_updated: None,
        }
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn is_done(&self) -> bool {
        self.done
    }
    pub fn mark_done(&mut self) {
        self.done = true;
        self.last_updated = Some(Local::now());
    }
    pub fn mark_undone(&mut self) {
        self.done = false;
        self.last_updated = Some(Local::now());
    }
    pub fn created_at(&self) -> HumanTime {
        HumanTime::from(self.created_at)
    }
    pub fn last_updated(&self) -> Option<HumanTime> {
        match self.last_updated {
            Some(time) => Some(HumanTime::from(time)),
            None => None,
        }
    }
}
#[test]
fn creating_a_task() {
    let task = Task::new(0, "Do the dishes");
    assert_eq!(task.description(), "Do the dishes");
    assert!(!task.is_done());
}
#[test]
fn date_is_readable_for_human() {
    let task = Task::new(0, "Do the dishes");
    assert_eq!("now", task.created_at().to_string());
}
#[test]
fn when_changing_task_status_it_updates_last_updated() {
    let mut task = Task::new(0, "Do the dishes");
    task.mark_done();
    assert!(task.is_done());
    assert_eq!("now", task.created_at().to_string());
    assert_eq!("now", task.last_updated().unwrap().to_string());
}
