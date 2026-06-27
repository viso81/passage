use anyhow::{Context, Result, anyhow};
use chardetng::EncodingDetector;
use chardetng::Iso2022JpDetection;
use chardetng::Utf8Detection;
use reqwest::blocking::Client;
use serde_json::Value;
use std::io::Read;
use std::io::{BufReader, ErrorKind};
use std::path::Path;

/// # 读取文件内容返回BufReader
pub fn read_to_buf<T: AsRef<Path>>(path: T) -> Result<BufReader<std::fs::File>> {
    let p = path.as_ref();
    let content = std::fs::File::open(p).map_err(|e| match e.kind() {
        ErrorKind::NotFound => {
            anyhow::anyhow!("文件不存在: {}", p.display())
        }
        ErrorKind::PermissionDenied => {
            anyhow::anyhow!("权限不足: {}", p.display())
        }
        _ => anyhow::anyhow!(e).context(format!("无法打开文件: {}", p.display())),
    })?;

    Ok(BufReader::new(content))
}

/// # 内容转换为utf-8
pub fn convert_to_utf8(bytes: &[u8]) -> Result<String> {
    // 尝试直接以utf-8打开文件
    if let Ok(text) = String::from_utf8(bytes.to_vec()) {
        return Ok(text);
    }
    // 探测编码
    let mut detector = EncodingDetector::new(Iso2022JpDetection::Deny);
    detector.feed(bytes, true);
    // 探测编码
    let encoding = detector.guess(None, Utf8Detection::Allow);
    // 解码
    let (text, _, had_errors) = encoding.decode(bytes);
    // 处理错误
    if had_errors {
        return Err(anyhow!("文件包含无效的{}编码字符", encoding.name()));
    }

    Ok(text.into_owned())
}

/// # 获取文件内的wget后的url
pub fn extract_wget_urls(lines: &str) -> Result<Vec<String>> {
    let re = regex::Regex::new(r"wget.*?(https?://\S+)")
        .map_err(|e| anyhow!("正则表达式编译失败: {}", e))?;
    let urls: Vec<String> = re
        .captures_iter(lines)
        .map(|cap| cap[1].to_string())
        .collect();

    if urls.is_empty() {
        return Err(anyhow!("文件内未提取到url链接!"));
    }

    Ok(urls)
}

/// # 通过url下载配置文件返回字符串
pub fn download_to_string(url: &str) -> Result<String> {
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .with_context(|| format!("下载失败: {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP 错误: {}", response.status());
    }

    let content = response.text().context("读取响应内容失败")?;

    Ok(content)
}

/// # 数组有多个元素时提醒只使用了第一个
pub fn json_array_only_first_element_warning(json: &Value, field: &str) {
    let arr = json.pointer(field).and_then(|v| v.as_array());
    if let Some(a) = arr {
        if a.len() > 1 {
            eprintln!("警告: {} 有多个选项, 当前只使用了第一个.", field);
        }
    }
}

/// # 获取json字段返回u16
pub fn get_json_field_to_u16(json: &Value, field: &str) -> Result<u16> {
    json.pointer(field)
        .and_then(|v| v.as_u64())
        .map(|n| n as u16)
        .ok_or_else(|| anyhow::anyhow!("解析{}发生异常", field))
}

/// # 获取json数组返回字符串
pub fn get_json_field_to_string(json: &Value, field: &str) -> Result<String> {
    let r = json
        .pointer(field)
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("解析{}发生异常", field))?
        .to_string();
    Ok(r)
}

/// # 从ip_*.bat中的url获取配置文件并返回内容
pub fn get_config_file_content<P: AsRef<std::path::Path>>(path: P) -> Result<String> {
    let p = path.as_ref();
    // 获取可能含有url的文件内容
    let mut reader = read_to_buf(p)?;
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    // 转换内容为utf-8
    let content_utf8 = convert_to_utf8(&bytes)?;
    // 获取url列表
    let urls = match extract_wget_urls(&content_utf8) {
        Ok(u) => u,
        Err(e) => {
            return Err(anyhow!("{}({})", e, p.display()));
        }
    };
    // 下载配置文件
    let mut downloaded_content = None;
    'sesstion: for url in urls {
        match download_to_string(&url) {
            Ok(content) => {
                downloaded_content = Some(content);
                break 'sesstion;
            }
            Err(e) => {
                eprintln!("下载失败: {} - {}", url, e);
                continue;
            }
        };
    }
    // 处理下载内容
    let downloaded_string = match downloaded_content {
        Some(content) => content,
        None => {
            anyhow::bail!("所有url提取文件均失败: {}", p.display());
        }
    };

    Ok(downloaded_string)
}
