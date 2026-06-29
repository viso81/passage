@echo off
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [错误] 请右键选择"以管理员身份运行"
    pause
    exit /b 1
)

set NSSM=nssm.exe
where nssm >nul 2>&1
if %errorLevel% neq 0 (
    set NSSM=C:\Users\%USERNAME%\scoop\apps\nssm\current\nssm.exe
)

echo [信息] 正在停止并卸载 mihomo 服务...
net stop mihomo >nul 2>&1
"%NSSM%" remove mihomo confirm

echo [完成] 服务已卸载
pause
