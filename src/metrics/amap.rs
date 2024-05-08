use std::collections::HashMap;
use std::fmt::Display;
use std::sync::atomic::AtomicI64;
use std::sync::Arc;

#[derive(Debug)]
pub struct AtomicMap {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl Clone for AtomicMap {
    fn clone(&self) -> Self {
        AtomicMap {
            data: Arc::clone(&self.data),
        }
    }
}

impl AtomicMap {
    pub fn new(metrics_name: &[&'static str]) -> Self {
        let mut data = HashMap::new();
        for name in metrics_name {
            data.insert(*name, AtomicI64::new(0));
        }
        AtomicMap {
            data: Arc::new(data),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> anyhow::Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or(anyhow::anyhow!("key: ({}) not found", key))?;
        counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}

impl Display for AtomicMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(
                f,
                "{}: {}",
                key,
                value.load(std::sync::atomic::Ordering::Relaxed)
            )?;
        }
        Ok(())
    }
}
