pub trait IndentSubsequentLines {
    fn indent_subsequent_lines(&self, indent: usize) -> String;
}

impl IndentSubsequentLines for String {
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
