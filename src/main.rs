use scaffolding::{ProjectBuilder, ProjectLanguage, ProjectTemplate, ProjectType};

mod scaffolding;

fn main() {

    //TODO: cli

    let template = ProjectTemplate::new(
        "MyProject".to_string(),
        ProjectLanguage::Cpp,
        ProjectType::Executable,
    );

    let builder = ProjectBuilder::new()
        .dest_folder("/path/to/destination".to_string())
        .name("MyProject".to_string())
        .template_project(template);

    if let Err(e) = builder.build() {
        eprintln!("Error: {}", e);
    }
}