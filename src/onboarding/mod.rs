use directory_creation::create_all_directories;

use crate::onboarding::configs::configs_pipeline;

pub(super) mod configs;
pub(super) mod directory_creation;

pub(crate) fn pipeline() {
    create_all_directories().unwrap();
    configs_pipeline().unwrap();
}
