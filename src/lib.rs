//! SigmaOS-android-gradle: Android Gradle build model.
//!
//! Real implementation: tasks, dependencies, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub id: String,
    pub task_id: String,
    pub library: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AndroidGradleError {
    EmptyTaskName,
    DuplicateTask(String),
}

impl fmt::Display for AndroidGradleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTaskName => write!(f, "task name must not be empty"),
            Self::DuplicateTask(id) => write!(f, "duplicate task {:?}", id),
        }
    }
}

impl std::error::Error for AndroidGradleError {}

pub struct AndroidGradle {
    pub name: String,
    tasks: BTreeMap<String, Task>,
    dependencies: BTreeMap<String, Dependency>,
}

impl AndroidGradle {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), tasks: BTreeMap::new(), dependencies: BTreeMap::new() }
    }

    pub fn add_task(&mut self, t: Task) -> Result<(), AndroidGradleError> {
        if t.name.trim().is_empty() { return Err(AndroidGradleError::EmptyTaskName); }
        let id = t.id.clone();
        if self.tasks.contains_key(&id) {
            return Err(AndroidGradleError::DuplicateTask(id.clone()));
        }
        self.tasks.insert(id, t);
        Ok(())
    }

    pub fn task_count(&self) -> usize { self.tasks.len() }

    pub fn get_task(&self, id: &str) -> Option<&Task> { self.tasks.get(id) }

    pub fn add_dependency(&mut self, d: Dependency) {
        self.dependencies.insert(d.id.clone(), d);
    }

    pub fn get_dependency(&self, id: &str) -> Option<&Dependency> { self.dependencies.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = AndroidGradle::new("AndroidGradle");
        assert_eq!(a.name, "AndroidGradle");
        assert_eq!(a.task_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = AndroidGradle::new("AndroidGradle");
        let task = Task { id: "t1".into(), name: "build".into(), type_: "build".into() };
        a.add_task(task).unwrap();
        assert_eq!(a.task_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = AndroidGradle::new("AndroidGradle");
        let task = Task { id: "x".into(), name: "".into(), type_: "x".into() };
        assert!(a.add_task(task).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = AndroidGradle::new("AndroidGradle");
        let task = Task { id: "x".into(), name: "X".into(), type_: "x".into() };
        a.add_task(task).unwrap();
        let task2 = Task { id: "x".into(), name: "X".into(), type_: "x".into() };
        assert!(a.add_task(task2).is_err());
    }
}
