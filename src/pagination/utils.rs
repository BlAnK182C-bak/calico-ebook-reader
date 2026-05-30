use std::collections::HashMap;

use crate::pagination::models::Page;

pub(crate) fn pages_offset_to_pg_no(pages: &Vec<Page>) -> HashMap<usize, usize> {
    let mut offset_to_page: HashMap<usize, usize> = HashMap::new();
    for (idx, page) in pages.iter().enumerate() {
        offset_to_page.insert(page.get_start_offset(), idx);
    }
    offset_to_page
}

#[cfg(test)]
mod pages_offset_to_pg_no_tests {
    // AI generated tests

    use super::*;
    use crate::common::utils::tests::create_page;

    #[test]
    fn returns_empty_map_for_empty_pages() {
        let pages = vec![];
        let result = pages_offset_to_pg_no(&pages);

        assert!(result.is_empty());
    }

    #[test]
    fn maps_single_page_offset_to_index() {
        let pages = vec![create_page(10)];
        let result = pages_offset_to_pg_no(&pages);

        assert_eq!(result.len(), 1);
        assert_eq!(result.get(&10), Some(&0));
    }

    #[test]
    fn maps_multiple_page_offsets_to_correct_indices() {
        let pages = vec![
            create_page(0),
            create_page(120),
            create_page(240),
            create_page(360),
        ];
        let result = pages_offset_to_pg_no(&pages);

        assert_eq!(result.len(), 4);
        assert_eq!(result.get(&0), Some(&0));
        assert_eq!(result.get(&120), Some(&1));
        assert_eq!(result.get(&240), Some(&2));
        assert_eq!(result.get(&360), Some(&3));
    }

    #[test]
    fn duplicate_offsets_overwrite_previous_index() {
        let pages = vec![create_page(100), create_page(200), create_page(100)];
        let result = pages_offset_to_pg_no(&pages);

        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&100), Some(&2));
        assert_eq!(result.get(&200), Some(&1));
    }
}
