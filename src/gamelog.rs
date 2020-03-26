pub struct GameLog {
    pub entries: Vec<String>,
}

impl GameLog {
    pub fn push(&mut self, line: String) {
        self.entries.push(line);
    }
}
