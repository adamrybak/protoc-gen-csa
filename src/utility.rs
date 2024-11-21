pub trait IndentLines {
    fn indent_lines(&self, indent: usize) -> String;
    fn indent_subsequent_lines(&self, indent: usize) -> String;
}

impl IndentLines for String {
    fn indent_lines(&self, indent: usize) -> String {
        let mut result = String::new();
        for line in self.split_inclusive("\n") {
            result.push_str(&format!("{}{}", "    ".repeat(indent), line));
        }
        result
    }

    fn indent_subsequent_lines(&self, indent: usize) -> String {
        let mut result = String::new();
        let mut lines = self.split_inclusive("\n");
        if let Some(first_line) = lines.next() {
            result.push_str(first_line);
            for line in lines {
                result.push_str(&format!("{}{}", "    ".repeat(indent), line));
            }
        }
        result
    }
}
