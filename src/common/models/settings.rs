use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Settings {
    sources: SourceSettings,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct SourceSettings {
    source_paths: Vec<String>,
}

impl Settings {
    pub(crate) fn new(source_settings: SourceSettings) -> Self {
        Self {
            sources: source_settings,
        }
    }
    pub(crate) fn get_get_source_paths(&self) -> &Vec<String> {
        &self.sources.source_paths
    }
}

impl SourceSettings {
    pub(crate) fn new(source_paths: Vec<String>) -> Self {
        Self { source_paths }
    }
}
