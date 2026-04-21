use std::collections::HashMap;

use crate::pagination::models::Page;

pub(crate) fn pages_offset_to_pg_no(pages: &Vec<Page>) -> HashMap<usize, usize> {
    let mut offset_to_page: HashMap<usize, usize> = HashMap::new();
    for (idx, page) in pages.iter().enumerate() {
        offset_to_page.insert(page.get_start_offset(), idx);
    }
    offset_to_page
}
