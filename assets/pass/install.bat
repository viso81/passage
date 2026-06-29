@echo off
setlocal

:: 检查管理员权限
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [错误] 请右键选择"以管理员身份运行"
    pause
    exit /b 1
)

set ROOT=%~dp0
set EXE=%ROOT%mihomo.exe
set CFG=%ROOT%config
set LOG=%ROOT%log
set NSSM=nssm.exe

:: 检查 mihomo.exe
if not exist "%EXE%" (
    echo [错误] 找不到 mihomo.exe，请先下载并放到 %ROOT%
    echo 下载地址: https://github.com/MetaCubeX/mihomo/releases/latest
    echo 文件名: mihomo-windows-amd64-vX.X.X.zip，解压后重命名为 mihomo.exe
    pause
    exit /b 1
)

:: 检查 nssm
where nssm >nul 2>&1
if %errorLevel% neq 0 (
    set NSSM=C:\Users\%USERNAME%\scoop\apps\nssm\current\nssm.exe
)
if not exist "%NSSM%" (
    echo [错误] 找不到 nssm，请确认已通过 scoop install nssm 安装
    pause
    exit /b 1
)

:: 创建日志目录
if not exist "%LOG%" mkdir "%LOG%"

:: 卸载旧服务（如果存在）
sc query mihomo >nul 2>&1
if %errorLevel% equ 0 (
    echo [信息] 发现旧服务，正在卸载...
    net stop mihomo >nul 2>&1
    "%NSSM%" remove mihomo confirm
)

:: 安装服务
echo [信息] 正在安装 mihomo 服务...
"%NSSM%" install mihomo "%EXE%" -d "%CFG%"
"%NSSM%" set mihomo AppDirectory "%CFG%"
"%NSSM%" set mihomo AppStdout "%LOG%\mihomo.log"
"%NSSM%" set mihomo AppStderr "%LOG%\mihomo.log"
"%NSSM%" set mihomo AppRotateFiles 1
"%NSSM%" set mihomo AppRotateBytes 10485760
"%NSSM%" set mihomo DisplayName "Mihomo Proxy"
"%NSSM%" set mihomo Description "Mihomo (Clash.Meta) proxy service"
"%NSSM%" set mihomo Start SERVICE_AUTO_START
"%NSSM%" set mihomo ObjectName LocalSystem

:: 启动服务
echo [信息] 正在启动服务...
net start mihomo

echo.
sc query mihomo | findstr "STATE"
echo.
echo [完成] mihomo 服务已安装并启动
echo 本地代理端口: 7890 (HTTP/SOCKS5 混合)
echo 控制台地址:   http://127.0.0.1:9090
echo 日志位置:     %LOG%\mihomo.log
echo.
pause
