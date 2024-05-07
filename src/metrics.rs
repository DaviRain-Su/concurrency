// metrics data structure
// basic function: inc/dec/snapshot
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Default)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> anyhow::Result<HashMap<String, i64>> {
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .clone())
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{:?}", self.snapshot())
        let data = self.snapshot().map_err(|_e| fmt::Error)?;
        for (key, value) in data {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}
