mod cli;
mod processor;
mod serv_info;

use clap::Parser;

pub fn run() -> anyhow::Result<()> {
    let cmd = cli::Cli::parse();
    match cmd.cmd {
        #![allow(unused_variables)]
        cli::Commands::Exp(opt) => {
            /*
            let content = processor::get_config_file_content(opt.path)?;
            let mut m = serv_info::ServInfo::new_hysteria2();
            serv_info::ServInfo::load_config_field(&mut m, &content)?;
            println!("{:?}", m);
            */
            let buf = processor::read_to_buf(opt.path)?;
            let map = processor::regex_yaml_locate_key("proxies:", buf)?;
            println!("{:#?}", map);
        }
        cli::Commands::GetAllLinks(opt) => {
            cli::cmd_get_all_subscription_links(&opt)?;
        }
    }
    Ok(())
}
