use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    action: String,
    sub_action: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    match args.action.as_str() {
        "new" => match args.sub_action.as_str() {
            "module" => {
                println!("Creating new module");
            }
            _ => {}
        },
        _ => {}
    }
}
