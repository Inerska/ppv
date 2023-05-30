use crate::scaffolding::ProjectTemplate;

pub struct ProjectBuilder {
    dest_folder: Option<String>,
    name: Option<String>,
    template_project: Option<ProjectTemplate>,
}

impl ProjectBuilder {
    pub fn new() -> Self {
        Self {
            dest_folder: None,
            name: None,
            template_project: None,
        }
    }

    pub fn dest_folder(mut self, folder: String) -> Self {
        self.dest_folder = Some(folder);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn template_project(mut self, template: ProjectTemplate) -> Self {
        self.template_project = Some(template);
        self
    }

    pub fn build(self) -> Result<(), &'static str> {
        match (self.dest_folder, self.name, self.template_project) {
            (Some(folder), Some(name), Some(template)) => {
                println!("Creating project {} in {}", name, folder);

                //TODO: logic

                Ok(())
            }
            _ => Err("Missing required fields"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scaffolding::{ProjectLanguage, ProjectType};
    use super::*;

    #[test]
    fn test_builder_dest_folder() {
        let builder = ProjectBuilder::new().dest_folder("/path/to/dest".to_string());
        assert_eq!(builder.dest_folder, Some("/path/to/dest".to_string()));
    }

    #[test]
    fn test_builder_name() {
        let builder = ProjectBuilder::new().name("MyProject".to_string());
        assert_eq!(builder.name, Some("MyProject".to_string()));
    }

    #[test]
    fn test_builder_template_project() {
        let template = ProjectTemplate::new(
            "MyProject".to_string(),
            ProjectLanguage::Cpp,
            ProjectType::Executable,
        );
        let builder = ProjectBuilder::new().template_project(template.clone());
        assert_eq!(builder.template_project, Some(template));
    }

    #[test]
    fn test_builder_build() {
        let template = ProjectTemplate::new(
            "MyProject".to_string(),
            ProjectLanguage::Cpp,
            ProjectType::Executable,
        );
        let builder = ProjectBuilder::new()
            .dest_folder("/path/to/dest".to_string())
            .name("MyProject".to_string())
            .template_project(template.clone());

        assert_eq!(builder.build().is_ok(), true);
    }

    #[test]
    fn test_builder_build_missing_fields() {
        let builder = ProjectBuilder::new()
            .dest_folder("/path/to/dest".to_string())
            .name("MyProject".to_string());

        assert_eq!(builder.build().is_err(), true);
    }
}
