use scaffolding::{ProjectBuilder, ProjectLanguage, ProjectTemplate, ProjectType};
use clap::Parser;

mod scaffolding;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t=1)]
    count: u8,
}

fn main() {

    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

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