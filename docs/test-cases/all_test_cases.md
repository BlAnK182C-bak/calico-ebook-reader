## Parsers Module

### File: `src/parsers/utils.rs`

#### Function: `get_file_name_from_path`
Extracts a filename from a given file path and handles edge cases for malformed or empty paths.

| Test Case | Description |
|---|---|
| `filename_has_no_slash` | Verifies filenames without path separators are returned correctly. |
| `multiple_slashes_in_file_path` | Ensures nested paths correctly return only the filename portion. |
| `filename_get_when_empty` | Confirms empty input paths are handled safely. |
| `empty_file_name` | Validates behavior when the path ends without a filename. |

#### Function: `a_new_book`
Creates metadata or storage state for a newly discovered book.

| Test Case | Description |
|---|---|
| `a_new_book` | Verifies new book creation logic works correctly. |

#### Function: `an_old_book`
Handles logic for books already known to the system.

| Test Case | Description |
|---|---|
| `an_old_book` | Ensures existing books are detected and processed correctly. |

#### Function: `parallelism_testing_with_fifty_books`
Validates parallel processing behavior for large batches of books (multithreading).

| Test Case | Description |
|---|---|
| `parallelism_testing_with_fifty_books` | Ensures parallel execution remains stable with fifty books processed simultaneously. |

#### Function: `other_book_type_than_known`
Handles unsupported or unknown book file types.

| Test Case | Description |
|---|---|
| `other_book_type_than_known` | Verifies unsupported file types are rejected or handled gracefully. |

---

### File: `src/parsers/epub/utils.rs`

#### Function: `extract_attr_value_from_attrs`
Extracts attribute values from EPUB XML attribute collections.

| Test Case | Description |
|---|---|
| `extracts_attribute_value_successfully` | Confirms attribute values are correctly extracted when present. |
| `returns_error_when_attribute_missing` | Verifies an error is returned when the target attribute does not exist. |
| `returns_first_matching_attribute` | Ensures the first matching attribute value is returned when duplicates exist. |
| `works_with_empty_attribute_list` | Confirms empty attribute collections are handled safely. |
| `attribute_name_matching_is_case_sensitive` | Verifies attribute lookup respects case sensitivity. |

#### Function: `extract_rootfile_path`
Parses EPUB container metadata to determine the rootfile path.

| Test Case | Description |
|---|---|
| `extracts_full_path_successfully` | Verifies valid rootfile paths are extracted correctly. |
| `returns_none_when_rootfile_missing` | Confirms missing rootfile entries return `None`. |
| `returns_none_when_full_path_missing` | Ensures malformed entries without full paths are rejected. |
| `returns_first_rootfile_when_multiple_exist` | Verifies the first rootfile entry is selected when multiple exist. |

#### Function: `validate_content_obf`
Validates EPUB mimetype and archive metadata structure.

| Test Case | Description |
|---|---|
| `not_epub_mimetype` | Confirms invalid EPUB mimetypes are rejected. |

---

### File: `src/parsers/epub/impls.rs`

#### Function: `validate_epub`
Validates EPUB archive structure and metadata before extraction.

| Test Case | Description |
|---|---|
| `validates_valid_epub_structure` | Confirms a valid EPUB archive passes validation checks. |
| `invalidates_when_mimetype_is_wrong` | Ensures EPUB validation fails for invalid mimetype declarations. |

#### Function: `init`
Initializes EPUB parser state and resolves required internal paths.

| Test Case | Description |
|---|---|
| `init_sets_entry_and_rootfile_paths` | Verifies entry and rootfile paths are initialized correctly. |
| `init_fails_when_epub_not_validated` | Ensures initialization fails if validation has not been completed first. |
| `init_fails_when_extracted_directory_is_missing` | Confirms initialization fails when extracted content is unavailable. |
| `init_sets_correct_entry_file_path` | Verifies the correct EPUB entry file path is resolved. |

#### Function: `extract_epub_file`
Extracts EPUB archives into working directories for parsing.

| Test Case | Description |
|---|---|
| `extracts_valid_epub_successfully` | Confirms valid EPUB archives are extracted successfully. |
| `sets_extracted_directory_path_correctly` | Verifies extraction output paths are set correctly. |
| `fails_when_epub_file_does_not_exist` | Ensures extraction fails cleanly for missing EPUB files. |
| `fails_for_invalid_zip_file` | Confirms invalid ZIP archives are rejected during extraction. |

---

## Pagination Module

### File: `src/pagination/utils.rs`

#### Function: `pages_offset_to_pg_no`
Maps page offsets to their corresponding page numbers.

| Test Case | Description |
|---|---|
| `returns_empty_map_for_empty_pages` | Confirms empty page collections produce empty mappings. |
| `maps_single_page_offset_to_index` | Verifies a single page offset maps correctly to its index. |
| `maps_multiple_page_offsets_to_correct_indices` | Ensures multiple offsets are mapped to the correct page numbers. |
| `duplicate_offsets_overwrite_previous_index` | Confirms duplicate offsets overwrite earlier mappings consistently. |

---

### File: `src/pagination/basic_pagination/impls.rs`

#### Function: `create_layout`
Builds paginated page structures from layout sections.

| Test Case | Description |
|---|---|
| `returns_empty_pages_when_layout_has_no_sections` | Confirms empty layouts generate no pages. |
| `creates_single_page_when_lines_fit_in_page_size` | Verifies content fitting within limits stays on a single page. |
| `splits_lines_into_multiple_pages` | Ensures oversized content is divided across multiple pages correctly. |
| `creates_pages_across_multiple_sections` | Confirms pagination works correctly across several layout sections. |

---

## Layout Module

### File: `src/layout/basic_layout/utils.rs`

#### Function: `wrap_words_to_next_line`
Wraps text into lines constrained by a maximum width while preserving offsets.

| Test Case | Description |
|---|---|
| `returns_single_line_when_text_fits_within_max_width` | Verifies short text remains on a single line. |
| `wraps_text_into_multiple_lines` | Confirms long text is wrapped into multiple lines correctly. |
| `preserves_correct_offsets_after_wrapping` | Ensures text offsets remain accurate after wrapping. |
| `handles_single_word_longer_than_max_width` | Verifies oversized words are handled safely during wrapping. |
| `handles_empty_input` | Confirms empty strings are processed without errors. |
| `ignores_extra_whitespace_between_words` | Ensures repeated whitespace does not affect wrapping behavior. |
| `wraps_every_word_when_max_width_is_small` | Confirms very small widths force each word onto separate lines. |

---

### File: `src/layout/basic_layout/impls.rs`

#### Function: `new`
Creates layout structures from parsed book sections and line constraints.

| Test Case | Description |
|---|---|
| `creates_empty_layout_when_book_has_no_sections` | Confirms books without sections produce empty layouts. |
| `creates_single_section_layout` | Verifies layouts are generated correctly for single-section books. |
| `wraps_long_lines_based_on_max_width` | Ensures long lines are wrapped according to configured widths. |
| `preserves_offsets_across_newlines` | Confirms offsets remain accurate when processing newline characters. |
| `preserves_offsets_across_multiple_sections` | Verifies offsets remain consistent across multiple sections. |
| `handles_multiple_wrapped_lines_and_newlines` | Ensures wrapping logic behaves correctly with mixed wrapping and newline scenarios. |

---

## Common Settings Module

### File: `src/common/utils/settings.rs`

#### Function: `scan_sources_for_books`
Scans configured source directories and synchronizes discovered books with stored metadata.

| Test Case | Description |
|---|---|
| `existing_sources_no_exsting_bookmap` | Verifies sources are scanned correctly when no existing bookmap is present. |
| `bookmap_file_doesnt_exist` | Confirms missing bookmap files are handled gracefully. |
| `settings_file_doesnt_exist` | Ensures missing settings files do not crash source scanning. |
| `empty_source_file_and_bookmap` | Verifies behavior when both source and bookmap files are empty. |
| `multiple_sources_and_books` | Confirms multiple source directories and books are processed correctly. |
| `non_empty_bookmap_with_empty_sources` | Ensures stale bookmap entries are handled when no sources are configured. |
| `non_empty_bookmap_with_multiple_sources_and_books` | Verifies synchronization logic works with populated sources and existing bookmaps. |
