// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-service-resource-limits: service resource limits component model.
//!
//! Real implementation: limits, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limit {
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
pub enum ServiceResourceLimitsError {
    EmptyLimitName,
    DuplicateLimit(String),
    EmptyEventSubject,
}

impl fmt::Display for ServiceResourceLimitsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLimitName => write!(f, "limit name must not be empty"),
            Self::DuplicateLimit(id) => write!(f, "duplicate limit {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ServiceResourceLimitsError {}

pub struct ServiceResourceLimits {
    pub name: String,
    limits: BTreeMap<String, Limit>,
    events: BTreeMap<String, Event>,
}

impl ServiceResourceLimits {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            limits: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_limit(&mut self, item: Limit) -> Result<(), ServiceResourceLimitsError> {
        if item.name.trim().is_empty() {
            return Err(ServiceResourceLimitsError::EmptyLimitName);
        }
        let id = item.id.clone();
        if self.limits.contains_key(&id) {
            return Err(ServiceResourceLimitsError::DuplicateLimit(id.clone()));
        }
        self.limits.insert(id, item);
        Ok(())
    }

    pub fn limit_count(&self) -> usize {
        self.limits.len()
    }

    pub fn get_limit(&self, id: &str) -> Option<&Limit> {
        self.limits.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ServiceResourceLimitsError> {
        if event.subject.trim().is_empty() {
            return Err(ServiceResourceLimitsError::EmptyEventSubject);
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

    fn sample_limit(id: &str, name: &str) -> Limit {
        Limit {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ServiceResourceLimits::new("ServiceResourceLimits");
        assert_eq!(m.name, "ServiceResourceLimits");
        assert_eq!(m.limit_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_limit() {
        let mut m = ServiceResourceLimits::new("ServiceResourceLimits");
        m.add_limit(sample_limit("a1", "Alpha")).unwrap();
        assert_eq!(m.limit_count(), 1);
        assert!(m.get_limit("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ServiceResourceLimits::new("ServiceResourceLimits");
        assert!(m.add_limit(sample_limit("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ServiceResourceLimits::new("ServiceResourceLimits");
        m.add_limit(sample_limit("x", "One")).unwrap();
        assert!(m.add_limit(sample_limit("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ServiceResourceLimits::new("ServiceResourceLimits");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
