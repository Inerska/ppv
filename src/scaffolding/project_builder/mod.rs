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
