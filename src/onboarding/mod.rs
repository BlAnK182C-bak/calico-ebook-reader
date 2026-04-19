use directory_creation::create_all_directories;

use crate::onboarding::settings_file_creation::settings_pipeline;

pub(super) mod directory_creation;
pub(super) mod settings_file_creation;

pub(crate) fn pipeline() {
    create_all_directories();
    settings_pipeline().unwrap();
}
