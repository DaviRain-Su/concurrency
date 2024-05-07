// metrics data structure
// basic function: inc/dec/snapshot
use dashmap::DashMap;
use std::fmt;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
