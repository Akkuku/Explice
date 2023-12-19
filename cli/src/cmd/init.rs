use crate::dialog::confirm_override;
use clap::Args;
use lib::{ExpliceConfig, Persist};

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct InitArgs {
    #[arg(long, short)]
    apikey: String,
    #[arg(long, short, default_value = "40")]
    token_limit: u16,
}

pub(crate) fn init_cmd(args: InitArgs) -> anyhow::Result<()> {
    if ExpliceConfig::exists()? {
        println!("Config already exists");

        if !confirm_override()? {
            return Ok(());
        }
    }

    let config = ExpliceConfig::new(&args.apikey, &args.token_limit);
    config.save()?;

    println!("Successfully initialized");
    Ok(())
}
