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

#[cfg(test)]
mod wraps_text_into_multiple_lines_tests {
    // AI generated tests

    use super::*;

    #[test]
    fn returns_single_line_when_text_fits_within_max_width() {
        let result = wrap_words_to_next_line("hello world", 20, 0);

        assert_eq!(result.len(), 1);

        assert_eq!(result[0].get_line_content(), "hello world");
        assert_eq!(result[0].get_offset_of_first_char(), 0);
    }

    #[test]
    fn wraps_text_into_multiple_lines() {
        let result = wrap_words_to_next_line("hello world from rust", 11, 0);

        assert_eq!(result.len(), 2);

        assert_eq!(result[0].get_line_content(), "hello world");
        assert_eq!(result[0].get_offset_of_first_char(), 0);

        assert_eq!(result[1].get_line_content(), "from rust");
        assert_eq!(result[1].get_offset_of_first_char(), 12);
    }

    #[test]
    fn preserves_correct_offsets_after_wrapping() {
        let result = wrap_words_to_next_line("abc def ghi", 7, 10);

        assert_eq!(result.len(), 2);

        assert_eq!(result[0].get_line_content(), "abc def");
        assert_eq!(result[0].get_offset_of_first_char(), 10);

        // "abc def" length = 7, +1 for skipped space
        assert_eq!(result[1].get_line_content(), "ghi");
        assert_eq!(result[1].get_offset_of_first_char(), 18);
    }

    #[test]
    fn handles_single_word_longer_than_max_width() {
        let result = wrap_words_to_next_line("superlongword", 5, 0);

        assert_eq!(result.len(), 1);

        assert_eq!(result[0].get_line_content(), "superlongword");
        assert_eq!(result[0].get_offset_of_first_char(), 0);
    }

    #[test]
    fn handles_empty_input() {
        let result = wrap_words_to_next_line("", 10, 0);

        assert_eq!(result.len(), 1);

        assert_eq!(result[0].get_line_content(), "");
        assert_eq!(result[0].get_offset_of_first_char(), 0);
    }

    #[test]
    fn ignores_extra_whitespace_between_words() {
        let result = wrap_words_to_next_line("hello    world", 20, 0);

        assert_eq!(result.len(), 1);

        // split_whitespace collapses repeated spaces
        assert_eq!(result[0].get_line_content(), "hello world");
    }

    #[test]
    fn wraps_every_word_when_max_width_is_small() {
        let result = wrap_words_to_next_line("a bb ccc", 1, 0);

        assert_eq!(result.len(), 3);

        assert_eq!(result[0].get_line_content(), "a");
        assert_eq!(result[1].get_line_content(), "bb");
        assert_eq!(result[2].get_line_content(), "ccc");
    }
}
