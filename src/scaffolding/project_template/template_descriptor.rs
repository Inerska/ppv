pub enum ProjectLanguage {
    Cpp,
    C,
}

pub enum ProjectType {
    Library,
    Executable,
    Console,
    Web,
}

pub struct ProjectTemplate {
    name: String,
    project_language: ProjectLanguage,
    project_type: ProjectType,
}

impl ProjectTemplate {
    pub fn new(name: String, project_language: ProjectLanguage, project_type: ProjectType) -> Self {
        Self {
            name,
            project_language,
            project_type,
        }
    }
}
