//! Manange a logical environment to sandbox environment variable access

use std::collections::BTreeMap;
use regex::{Error, Regex};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LogicalEnv {
    env: BTreeMap<String, String>,
    pristine: bool,
}

impl LogicalEnv {
    /// Create a new empty environment
    pub fn new() -> Self {
        LogicalEnv {
            env: Default::default(),
            pristine: false, // not a copy of the process environment
        }
    }

    pub fn is_pristine(&self) -> bool {
        self.pristine
    }

    /// Initialize from the process environment
    pub fn from_process_environment() -> Self {
        LogicalEnv {
            env: std::env::vars().collect(),
            pristine: true, // copy of process env
        }
    }

    pub fn get(&self, var: &str) -> Option<&str> {
        self.env.get(var).map(String::as_str)
    }

    pub fn set(&mut self, var: &str, val: &str) {
        let _ = self.env.insert(var.to_string(), val.to_string());
        self.pristine = false;
    }

    pub fn deny(&mut self, regex: &str) -> Result<(), Error> {
        self.pristine = false; // XXX only if we make a change?
        let re = Regex::new(regex)?;

        self.env = std::mem::replace(&mut self.env, Default::default())
            .into_iter()
            .filter(|(var, _)| !re.is_match(var))
            .collect();

        Ok(())
    }

    pub fn allow(&mut self, regex: &str) -> Result<(), Error> {
        self.pristine = false;
        let re = Regex::new(regex)?;

        for (var, val) in std::env::vars().filter(|(var, _)| re.is_match(var)) {
            self.env.insert(var, val);
        }

        Ok(())
    }
}
