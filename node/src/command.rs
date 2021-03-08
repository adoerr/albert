use {sc_cli::RunCmd, structopt::StructOpt};

#[derive(Debug, StructOpt)]
pub struct Command {
    #[structopt(flatten)]
    pub run: RunCmd,
}
