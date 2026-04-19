use directories::ProjectDirs;

pub(crate) fn extract_project_dir(
    qualifier: &str,
    organization: &str,
    application: &str,
) -> Option<ProjectDirs> {
    ProjectDirs::from(qualifier, organization, application)
}
