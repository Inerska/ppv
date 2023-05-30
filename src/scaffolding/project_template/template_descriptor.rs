#[derive(Debug, PartialEq)]
pub enum ProjectLanguage {
    Cpp,
    C,
}

impl Clone for ProjectLanguage {
    fn clone(&self) -> Self {
        match self {
            Self::Cpp => Self::Cpp,
            Self::C => Self::C,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ProjectType {
    Library,
    Executable,
    Console,
    Web,
}

impl Clone for ProjectType {
    fn clone(&self) -> Self {
        match self {
            Self::Library => Self::Library,
            Self::Executable => Self::Executable,
            Self::Console => Self::Console,
            Self::Web => Self::Web,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ProjectTemplate {
    name: String,
    project_language: ProjectLanguage,
    project_type: ProjectType,
}

impl Clone for ProjectTemplate {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            project_language: self.project_language.clone(),
            project_type: self.project_type.clone(),
        }
    }
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
