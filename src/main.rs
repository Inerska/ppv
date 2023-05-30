use scaffolding::{ProjectBuilder, ProjectLanguage, ProjectTemplate, ProjectType};

mod scaffolding;

fn main() {

    //TODO: cli

    let template = ProjectTemplate::new(
        "MyProject".to_string(),
        ProjectLanguage::Cpp,
        ProjectType::Executable,
    );

    let builder = ProjectBuilder;

    if let Err(e) = builder.create_project("/path/to/destination", "MyProject", &template) {
        eprintln!("Error: {}", e);
    }
}