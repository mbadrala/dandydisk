use clap::Parser;

#[derive(Parser)]
#[command(name = "DandyDisk")]
#[command(version = "1.0")]
#[command(about = "Scan files", long_about = None)]
struct Cli {
    dir: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("Dir: {:?}", cli.dir);
}
