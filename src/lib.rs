mod cli;
mod processor;
mod serv_info;

use clap::Parser;

pub fn run() -> anyhow::Result<()> {
    let cmd = cli::Cli::parse();
    match cmd.cmd {
        #![allow(unused_variables)]
        cli::Commands::Exp(opt) => {
            let content = processor::get_config_file_content(opt.path)?;
            let mut m = serv_info::ServInfo::new_hysteria2();
            serv_info::ServInfo::load_config_field(&mut m, &content)?;
            println!("{:?}", m);
        }
        cli::Commands::GetAllLinks(opt) => {
            // todo
            cli::cmd_get_all_subscription_links(&opt)?;
        }
    }
    Ok(())
}
