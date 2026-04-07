pub(super) mod directory_creation;

use directory_creation::create_all_directories;

pub(crate) fn pipeline() {
    create_all_directories();
}
