//! lib
mod cli;
mod processor;
mod serv_info;

use std::io::BufRead;

use anyhow::Context;
use clap::Parser;
use regex::Regex;

pub fn run() -> anyhow::Result<()> {
    let cmd = cli::Cli::parse();
    match cmd.cmd {
        #![allow(unused_variables)]
        cli::Commands::Exp(opt) => {
            let buf = processor::read_to_buf(opt.path)?;
            let mut lines = buf.lines().collect::<std::io::Result<Vec<String>>>()?;
            let map: crate::processor::YamlMap =
                processor::regex_yaml_locate_key("proxies:", &lines)?;
            let m: serv_info::ServInfo =
                crate::processor::load_data_from_file("./assets/temp/Mieru.tmp")?;
            let h2: serv_info::ServInfo =
                crate::processor::load_data_from_file("./assets/temp/Hysteria2.tmp")?;

            let re_kv = Regex::new(crate::processor::REGEX_YAML_KEY_VAL)?;

            write_to_data(&mut lines, &map, m, &"mieru".to_string(), 0, &re_kv)?;
            write_to_data(&mut lines, &map, h2, &"hysteria2".to_string(), 0, &re_kv)?;

            println!("-------------");
            for l in lines {
                println!("{}", l);
            }
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

fn write_to_data(
    lines: &mut Vec<String>,
    map: &crate::processor::YamlMap,
    targets: crate::serv_info::ServInfo,
    name: &String,
    no: usize,
    re: &Regex,
) -> anyhow::Result<()> {
    match targets {
        crate::serv_info::ServInfo::Mieru(info) => {
            let mieru = &map.get(name).context("")?;
            // let (_, serv_type, _) = mieru.get("type").context("")?;
            // if !serv_type.eq(&"mieru".to_string()) {}
            let (serv_num, _, _) = mieru.get("server").context("缺少字段server")?;
            let (port_num, _, _) = mieru.get("port").context("")?;
            let (pass_num, _, _) = mieru.get("password").context("")?;
            let (user_num, _, _) = mieru.get("username").context("")?;

            let m = info.get(no).context("")?;

            crate::processor::regex_modify_key_val_node(
                lines,
                (
                    serv_num.clone(),
                    crate::processor::YamlValue::Str(m.server.clone()),
                ),
                &re,
            )?;
            crate::processor::regex_modify_key_val_node(
                lines,
                (
                    port_num.clone(),
                    crate::processor::YamlValue::Num(m.port.clone() as usize),
                ),
                &re,
            )?;
            crate::processor::regex_modify_key_val_node(
                lines,
                (
                    pass_num.clone(),
                    crate::processor::YamlValue::Str(m.password.clone()),
                ),
                &re,
            )?;
            crate::processor::regex_modify_key_val_node(
                lines,
                (
                    user_num.clone(),
                    crate::processor::YamlValue::Str(m.username.clone()),
                ),
                &re,
            )?;
        }
        crate::serv_info::ServInfo::Hysteria2(info) => {
            let h2 = &map.get(name).context("")?;
            let (serv_num, _, _) = h2.get("server").context("")?;
            let (port_num, _, _) = h2.get("port").context("")?;
            let (pass_num, _, _) = h2.get("password").context("")?;

            let m = info.get(no).context("")?;

            crate::processor::regex_modify_key_val_node(
                lines,
                (
                    serv_num.clone(),
                    crate::processor::YamlValue::Str(m.server.clone()),
                ),
                &re,
            )?;
            crate::processor::regex_modify_key_val_node(
                lines,
                (
                    port_num.clone(),
                    crate::processor::YamlValue::Num(m.port.clone() as usize),
                ),
                &re,
            )?;
            crate::processor::regex_modify_key_val_node(
                lines,
                (
                    pass_num.clone(),
                    crate::processor::YamlValue::Str(m.password.clone()),
                ),
                &re,
            )?;
        }
    }

    Ok(())
}
