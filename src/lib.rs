//! lib
mod cli;
mod processor;
mod serv_info;

use std::io::BufRead;

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
            let mut lines = buf.lines().collect::<std::io::Result<Vec<String>>>()?;
            let map = processor::regex_yaml_locate_key("proxies:", &lines)?;
            println!("{:#?}", map);
        }
        cli::Commands::Exp2(opt) => {
            // todo
            processor::backup_with_sequence(opt.path, opt.out)?;
        }
        cli::Commands::GetAllLinks(opt) => {
            cli::cmd_get_all_subscription_links(&opt)?;
        }
    }
    Ok(())
}
