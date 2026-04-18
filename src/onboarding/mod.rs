use directory_creation::create_all_directories;

pub(super) mod directory_creation;

pub(crate) fn pipeline() {
    create_all_directories();
}
