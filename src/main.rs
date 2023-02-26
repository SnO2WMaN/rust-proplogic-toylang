use anyhow::Result;
use clap::Parser;

mod tokenizer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    eval: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let tokens = tokenizer::tokenize(&args.eval)?;
    println!("Tokens: {:?}", tokens);

    Ok(())
}
