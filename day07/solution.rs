pub struct LogQuery<'a> {
    logs: &'a[String],
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        Self{ logs: logs.as_slice() } 
    }

    pub fn search(&self, query: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log|log.contains(query))
            .collect()
    }
}

