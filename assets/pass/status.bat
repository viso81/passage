@echo off
echo ===== mihomo 服务状态 =====
sc query mihomo | findstr /i "STATE"
echo.
echo ===== 最新日志（末尾 30 行）=====
set LOG=%~dp0log\mihomo.log
if exist "%LOG%" (
    powershell -command "Get-Content '%LOG%' -Tail 30"
) else (
    echo 日志文件不存在: %LOG%
)
echo.
pause
