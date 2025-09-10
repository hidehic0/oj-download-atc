use clap::Parser;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"),version = env!("CARGO_PKG_VERSION"),arg_required_else_help = true,)]
struct Args {
    problemid: String,
}

fn main() {
    let args = Args::parse();

    println!("The selected problem is {}", args.problemid)
}
