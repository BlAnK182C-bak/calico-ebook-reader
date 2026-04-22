use directory_creation::create_all_directories;

use crate::onboarding::{
    bookmarks_file_creation::bookmarks_pipeline, settings_file_creation::settings_pipeline,
};

pub(super) mod bookmarks_file_creation;
pub(super) mod directory_creation;
pub(super) mod settings_file_creation;

pub(crate) fn pipeline() {
    create_all_directories();
    settings_pipeline().unwrap();
    bookmarks_pipeline().unwrap();
}
