mod builder;
mod manifest;
mod processors;

use clap::Parser;
use manifest::Manifest;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value = "src")]
    input: String,

    #[arg(short, long, default_value = "dist")]
    output: String,
}

fn main() {
    let args: Args = Args::parse();

    let mut manifest: Manifest = Manifest::new();

    builder::build_project(&args.input, &args.output, &mut manifest);

    manifest.save_to(&args.output);

    println!("✅ Build complete. Manifest written.");
}
