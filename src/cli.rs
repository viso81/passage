///! # 命令行
use crate::processor::get_config_file_content;
use crate::serv_info::ServInfo;
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// 命令行结构
#[derive(Parser)]
#[command(name = "passage-cmd")]
pub struct Cli {
    #[command[subcommand]]
    pub cmd: Commands,
}
// 子命令
#[derive(Subcommand, Debug)]
pub enum Commands {
    Exp(ArgExp),
    GetAllLinks(ArgGetAllSubscriptionLinks),
}
// 参数
#[derive(Args, Debug)]
pub struct ArgExp {
    pub path: PathBuf,
}
#[derive(Args, Debug)]
pub struct ArgGetAllSubscriptionLinks {
    pub dir: PathBuf,
    #[arg(short, long, value_delimiter = ',', default_value = "hysteria2,mieru")]
    pub allowd: Vec<String>,
}

/// 获取所有服务器订阅
///
/// # 参数
/// - `opt`: 子命令和参数获取
///
/// # 返回值
/// 返回SerInfo类型,其中包括所有获取的代理参数
///
pub fn cmd_get_all_subscription_links(opt: &ArgGetAllSubscriptionLinks) -> Result<()> {
    let mut m = ServInfo::new_mieru();
    let mut h = ServInfo::new_hysteria2();

    for entry in walkdir::WalkDir::new(&opt.dir)
        .into_iter()
        .filter_entry(|entry| {
            let rel = match entry.path().strip_prefix(&opt.dir) {
                Ok(r) => r,
                Err(_) => return true,
            };
            let first = rel.components().next();
            let result = match first {
                None => true,
                Some(comp) => {
                    let name = comp.as_os_str().to_string_lossy();
                    opt.allowd.iter().any(|a| {
                        // println!("allow:{}, name:{}", a, &name);
                        a.eq_ignore_ascii_case(&name)
                    })
                }
            };
            result
        })
    {
        let entry = entry?;
        let ext = entry.path().extension();
        let parent = entry.path().parent();
        let pparent = entry
            .path()
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase());
        let is_bat = ext.map(|e| e.eq_ignore_ascii_case("bat")).unwrap_or(false);
        let is_parent_eq = parent
            .and_then(|p| p.file_name())
            .map(|e| e.eq_ignore_ascii_case("ip_update"))
            .unwrap_or(false);
        if entry.path().is_file() && is_bat && is_parent_eq {
            // println!("{}", entry.path().display());
            let content = get_config_file_content(entry.path())?;
            match &pparent {
                Some(name) => match name.as_str() {
                    "mieru" => {
                        // println!("{}", entry.path().display());
                        if let Err(e) = ServInfo::load_config_field(&mut m, &content) {
                            eprintln!("({})获取的配置{}", &entry.path().display(), e);
                            continue;
                        }
                    }
                    "hysteria2" => {
                        // println!("{}", entry.path().display());
                        if let Err(e) = ServInfo::load_config_field(&mut h, &content) {
                            eprintln!("({})获取的配置{}", &entry.path().display(), e);
                            continue;
                        }
                    }
                    _ => {}
                },
                None => {
                    eprintln!("没有匹配父目录");
                }
            }
        }
    }

    println!("{:?}", m);
    println!("{:?}", h);
    Ok(())
}
