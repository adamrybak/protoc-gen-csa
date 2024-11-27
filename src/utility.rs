pub trait IndentLines {
    fn indent_lines(&self, indent: usize) -> String;
    fn indent_subsequent_lines(&self, indent: usize) -> String;
}

impl IndentLines for String {
    fn indent_lines(&self, indent: usize) -> String {
        let mut result = String::new();
        for line in self.split_inclusive("\n") {
            if line.trim().is_empty() {
                result.push_str(line);
            } else {
                result.push_str(&format!("{}{}", " ".repeat(indent * 4), line));
            }
        }
        result
    }

    fn indent_subsequent_lines(&self, indent: usize) -> String {
        let mut result = String::new();
        let mut lines = self.split_inclusive("\n");
        if let Some(first_line) = lines.next() {
            result.push_str(first_line);
            for line in lines {
                if line.trim().is_empty() {
                    result.push_str(line);
                } else {
                    result.push_str(&format!("{}{}", " ".repeat(indent * 4), line));
                }
            }
        }
        result
    }
}

pub trait JoinNonEmpty {
    fn join_non_empty(&mut self, separator: &str) -> String;
}

impl<T> JoinNonEmpty for T
where
    T: Iterator,
    T::Item: AsRef<str>,
{
    fn join_non_empty(&mut self, separator: &str) -> String {
        self.fold(String::new(), |acc, x| {
            let value = x.as_ref();
            if value.is_empty() {
                acc
            } else {
                if acc.is_empty() {
                    value.to_string()
                } else {
                    format!("{}{}{}", acc, separator, value)
                }
            }
        })
    }
}
