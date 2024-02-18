use clap::Args;
use lib::ExpliceConfig;

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct ConfigArgs {
    #[arg(long, short)]
    apikey: Option<String>,
    #[arg(long, short)]
    token_limit: Option<u16>,
}

pub(crate) fn config_cmd(args: ConfigArgs) -> anyhow::Result<()> {
    ExpliceConfig::read()?
        .update(args.apikey.as_deref(), args.token_limit)
        .save()?;

    println!("Successfully updated config");
    Ok(())
}
