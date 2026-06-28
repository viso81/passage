use crate::processor::{
    json_array_only_first_element_warning, json_get_field_to_string, json_get_field_to_u16,
};
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// # 服务器信息数据结构  
#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ServInfo {
    Hysteria2(Vec<InfoHysteria2>),
    Mieru(Vec<InfoMieru>),
}

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
pub struct InfoHysteria2 {
    server: String,
    port: u16,
    password: String,
}

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
pub struct InfoMieru {
    server: String,
    port: u16,
    username: String,
    password: String,
}

#[allow(unused)]
impl ServInfo {
    pub fn new_mieru() -> Self {
        ServInfo::Mieru(Vec::new())
    }
    pub fn new_hysteria2() -> Self {
        ServInfo::Hysteria2(Vec::new())
    }

    /// # 获取json关键字段并存储
    pub fn load_config_field(serv: &mut ServInfo, content: &String) -> Result<()> {
        let json: Value = serde_json::from_str(content).context("JSON解析失败")?;

        match serv {
            // Mieru服务器
            ServInfo::Mieru(info) => {
                // 检查json中数组
                for json_path in ["/profiles", "/profiles/0/servers"] {
                    json_array_only_first_element_warning(&json, json_path);
                }

                let server = json_get_field_to_string(&json, "/profiles/0/servers/0/ipAddress")?;
                let port =
                    json_get_field_to_u16(&json, "/profiles/0/servers/0/portBindings/0/port")?;
                let username = json_get_field_to_string(&json, "/profiles/0/user/name")?;
                let password = json_get_field_to_string(&json, "/profiles/0/user/password")?;

                info.push(InfoMieru {
                    server,
                    port,
                    username,
                    password,
                })
            }
            // Hysteria2服务器
            ServInfo::Hysteria2(info) => {
                let value = json_get_field_to_string(&json, "/server")?;
                let (addr_part, port_str) = match value.rsplit_once(":") {
                    Some(a) => a,
                    None => {
                        bail!("格式错误: 期望'server:port', 获得{}", value);
                    }
                };
                let server = addr_part.to_string();
                let port = port_str.parse::<u16>().context("端口解析失败!")?;
                let password = json_get_field_to_string(&json, "/auth")?;

                info.push(InfoHysteria2 {
                    server,
                    port,
                    password,
                });
            }
        }

        Ok(())
    }
}
