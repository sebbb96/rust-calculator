#[derive(Clone, Debug)]
pub struct HistoryEntry {
    pub operation: String,
    pub operands: Vec<f64>,
    pub result: f64,
}

pub struct History {
    entries: Vec<HistoryEntry>,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, operation: String, operands: Vec<f64>, result: f64) {
        self.entries.push(HistoryEntry {
            operation,
            operands,
            result,
        });
    }

    pub fn get_entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
