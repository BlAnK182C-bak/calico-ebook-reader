use indexmap::IndexMap;

// structs
pub(crate) struct RawEpub {
    pub(crate) file_path: String,
    pub(crate) extracted_directory_path: Option<String>, // The folder in device where the epub is extracted to
    pub(crate) is_validated: bool,
    pub(crate) entry_file_path: Option<String>, // META-INF/container.xml
    pub(crate) rootfile_path: Option<String>,   //content.obf
    pub(crate) spine_to_mainfest_map: IndexMap<String, String>, // using an IndexMap because insertion order
}
