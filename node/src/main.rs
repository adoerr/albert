mod command;
mod cli;

fn main() -> sc_cli::Result<()> {
    cli::run()
}