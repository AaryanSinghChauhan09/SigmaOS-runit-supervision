// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-runit-supervision: runit supervision component model.
//!
//! Real implementation: supervisions, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Supervision {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunitSupervisionError {
    EmptySupervisionName,
    DuplicateSupervision(String),
    EmptyEventSubject,
}

impl fmt::Display for RunitSupervisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySupervisionName => write!(f, "supervision name must not be empty"),
            Self::DuplicateSupervision(id) => write!(f, "duplicate supervision {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RunitSupervisionError {}

pub struct RunitSupervision {
    pub name: String,
    supervisions: BTreeMap<String, Supervision>,
    events: BTreeMap<String, Event>,
}

impl RunitSupervision {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            supervisions: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_supervision(&mut self, item: Supervision) -> Result<(), RunitSupervisionError> {
        if item.name.trim().is_empty() {
            return Err(RunitSupervisionError::EmptySupervisionName);
        }
        let id = item.id.clone();
        if self.supervisions.contains_key(&id) {
            return Err(RunitSupervisionError::DuplicateSupervision(id.clone()));
        }
        self.supervisions.insert(id, item);
        Ok(())
    }

    pub fn supervision_count(&self) -> usize {
        self.supervisions.len()
    }

    pub fn get_supervision(&self, id: &str) -> Option<&Supervision> {
        self.supervisions.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RunitSupervisionError> {
        if event.subject.trim().is_empty() {
            return Err(RunitSupervisionError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_supervision(id: &str, name: &str) -> Supervision {
        Supervision {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = RunitSupervision::new("RunitSupervision");
        assert_eq!(m.name, "RunitSupervision");
        assert_eq!(m.supervision_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_supervision() {
        let mut m = RunitSupervision::new("RunitSupervision");
        m.add_supervision(sample_supervision("a1", "Alpha")).unwrap();
        assert_eq!(m.supervision_count(), 1);
        assert!(m.get_supervision("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = RunitSupervision::new("RunitSupervision");
        assert!(m.add_supervision(sample_supervision("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = RunitSupervision::new("RunitSupervision");
        m.add_supervision(sample_supervision("x", "One")).unwrap();
        assert!(m.add_supervision(sample_supervision("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = RunitSupervision::new("RunitSupervision");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
