pub struct History {
    lines: Vec<String>,
}

impl History {
    pub fn new() -> Self {
        History {
            lines: vec![]
        }
    }

    // get lines that is inputted before
    pub fn get(&self, index: usize) -> Option<String> {
        // index = 1 -> last line
        let len = self.lines.len();

        if index <= len {
            Some(self.lines[len - index].clone())
        } else {
            None
        }
    }
}