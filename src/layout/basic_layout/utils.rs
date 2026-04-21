use crate::common::models::line::Line;

pub(super) fn wrap_words_to_next_line(
    line: &str,
    max_width: usize,
    byte_offset: usize,
) -> Vec<Line> {
    let mut res: Vec<Line> = Vec::new();
    let mut curr_line: String = String::new();
    let mut current_chunk_offset = byte_offset;

    for word in line.split_whitespace() {
        let separator = if curr_line.is_empty() { "" } else { " " };
        if curr_line.len() + separator.len() + word.len() > max_width && !curr_line.is_empty() {
            let taken = std::mem::take(&mut curr_line);
            res.push(Line::new(&taken, current_chunk_offset));
            current_chunk_offset += &taken.len() + 1;
        } else {
            curr_line.push_str(separator);
        }
        curr_line.push_str(word);
    }
    res.push(Line::new(&curr_line, current_chunk_offset));
    res
}
