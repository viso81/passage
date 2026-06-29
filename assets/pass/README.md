# passage-core

mihomo 代理方案，结构精简版。

## 目录结构

```
D:\passage-core\
    mihomo.exe          ← 手动下载放这里
    install.bat         ← 安装服务（管理员运行）
    uninstall.bat       ← 卸载服务（管理员运行）
    status.bat          ← 查看状态和日志
    log\
        mihomo.log      ← 运行日志（自动轮转，上限 10MB）
    config\
        config.yaml     ← 主配置
        ruleset\        ← 规则集（首次启动自动下载）
```

## 首次安装步骤

1. 下载 mihomo：
   https://github.com/MetaCubeX/mihomo/releases/latest
   找 `mihomo-windows-amd64-vX.X.X.zip`，解压，重命名为 `mihomo.exe`，放到本目录

2. 右键 `install.bat` → 以管理员身份运行

3. 完成，开机自动启动

## 常用命令

```cmd
net start mihomo      # 启动
net stop mihomo       # 停止
sc query mihomo       # 查状态
```

或直接双击 `status.bat` 查看状态和日志。

## 代理信息

| 项目 | 值 |
|------|-----|
| 本地端口 | 7890 (HTTP/SOCKS5 混合) |
| TUN 模式 | 开启（自动接管全局流量） |
| 控制台 | http://127.0.0.1:9090 |

## 浏览器插件（SwitchyOmega）

- 协议：SOCKS5 或 HTTP
- 地址：127.0.0.1
- 端口：7890

## 服务器信息

| 字段 | 值 |
|------|-----|
| 协议 | mieru (TCP) |
| 服务器 | 156.225.88.93 |
| 端口 | 19022 |
| 用户名 | tcp_yuyys0xg |
| 密码 | 8n2IZjc7vUJPPEqm |

> IP 被封时更新服务器地址，改 config/config.yaml 里的 server 字段，然后 `net stop mihomo && net start mihomo`
