use crate::common::models::book::Book;
use crate::layout::structs::LayoutEngine;

pub(crate) mod basic_layout;
pub(crate) mod impls;
pub(crate) mod structs;

pub(crate) fn layoutize<E: LayoutEngine>(book: &Book, max_width: usize) -> E::OutputLayout {
    E::create_layout(max_width, book)
}
