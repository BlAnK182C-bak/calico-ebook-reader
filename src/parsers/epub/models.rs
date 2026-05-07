use indexmap::IndexMap;

// structs
pub(crate) struct RawEpub {
    pub(super) file_path: String,
    pub(super) extracted_directory_path: Option<String>, // The folder in device where the epub is extracted to
    pub(super) is_validated: bool,
    pub(super) entry_file_path: Option<String>, // META-INF/container.xml
    pub(super) rootfile_path: Option<String>,   //content.obf
    pub(super) spine_to_mainfest_map: IndexMap<String, String>, // using an IndexMap because insertion order
}
