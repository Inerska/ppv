use std::io;
use std::path::Path;
use std::fs;
use crate::scaffolding::project_template::{ProjectTemplate, Clonable};

pub struct ProjectBuilder;

impl ProjectBuilder {
    pub fn create_project<P: AsRef<Path>>(&self, dest_folder: P, name: &str, template_project: &ProjectTemplate) -> io::Result<()> {
        println!("Creating project {} in {:?}", name, dest_folder.as_ref());

        // TODO: more errors
        fs::create_dir_all(&dest_folder)?;

        template_project.clone_project()?;

        Ok(())
    }
}
