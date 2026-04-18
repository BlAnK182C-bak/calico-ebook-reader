use crate::common::models::line::Line;

pub(super) fn wrap_words_to_next_line(line: &str, max_width: usize) -> Vec<Line> {
    let mut res: Vec<Line> = Vec::new();
    let mut curr_line: String = String::new();

    for word in line.split_whitespace() {
        let separator = if curr_line.is_empty() { "" } else { " " };
        if curr_line.len() + separator.len() + word.len() > max_width && !curr_line.is_empty() {
            res.push(Line::new(std::mem::take(&mut curr_line)));
        } else {
            curr_line.push_str(separator);
        }
        curr_line.push_str(word);
    }
    res.push(Line::new(curr_line));
    res
}
