use std::error::Error;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

use clap::Parser;
use dialoguer::console::Style;
use dialoguer::Select;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    action: String,
    sub_action: String,
}

#[derive(Debug)]
enum ModuleType {
    GLOBAL,
    VENDOR,
}

#[derive(Debug)]
struct ModuleConfig {
    name: String,
    module_type: ModuleType,
    template_path: std::path::PathBuf,
}

fn prompt_module_config() -> Result<Option<ModuleConfig>, Box<dyn Error>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("Welcome to the setup wizard");

    if !Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
        .interact()?
    {
        exit(1);
    }

    let module_name: String = Input::new().with_prompt("Name").interact_text().unwrap(); // todo: replace unwrap with ? and place it inside a function that
                                                                                         // returns a result

    println!("Module name: {}", module_name);

    let module_type_index = Select::new()
        .with_prompt("Type")
        .default(0)
        .item("global")
        .item("vendor")
        .interact()?;

    let module_type: ModuleType = match module_type_index {
        0 => ModuleType::GLOBAL,
        1 => ModuleType::VENDOR,
        _ => ModuleType::GLOBAL,
    };

    println!("Module type: {:?}", module_type);

    Ok(Some(ModuleConfig {
        name: module_name,
        module_type,
        template_path: PathBuf::from_str("./main.rs")?,
    }))
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    match args.action.as_str() {
        "new" => match args.sub_action.as_str() {
            "module" => {
                println!("New Module.");
                // let input: String = Input::new().with_prompt("Your name").interact_text()?;

                match prompt_module_config() {
                    Ok(config) => println!("Module user config: {:?}", config),
                    Err(err) => println!("Error getting module user config: {}", err),
                }
            }
            _ => {}
        },
        _ => {}
    }
}
