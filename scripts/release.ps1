<#
.SYNOPSIS
    自动化发布脚本 - PowerShell 版本
.DESCRIPTION
    自动更新版本号并创建 Git tag 来触发 GitHub Actions 构建和部署
.PARAMETER Version
    要发布的版本号，格式为 x.y.z
.EXAMPLE
    .\scripts\release.ps1 -Version "1.0.1"
.EXAMPLE
    .\scripts\release.ps1 "1.0.1"
#>

param(
    [Parameter(Mandatory=$true, Position=0)]
    [string]$Version
)

# 设置错误处理
$ErrorActionPreference = "Stop"

# 验证版本号格式
if ($Version -notmatch '^\d+\.\d+\.\d+$') {
    Write-Host "❌ 版本号格式不正确，应为 x.y.z 格式" -ForegroundColor Red
    exit 1
}

Write-Host "🚀 开始发布版本 $Version" -ForegroundColor Green

try {
    # 1. 检查工作目录是否干净
    Write-Host "📋 检查 Git 状态..." -ForegroundColor Yellow
    
    $gitStatus = git status --porcelain
    if ($gitStatus) {
        Write-Host "❌ 工作目录不干净，请先提交所有更改" -ForegroundColor Red
        Write-Host "未提交的文件:" -ForegroundColor Red
        Write-Host $gitStatus -ForegroundColor Red
        exit 1
    }

    # 2. 更新 package.json
    Write-Host "📝 更新 package.json..." -ForegroundColor Yellow
    $packageJsonPath = "./package.json"
    $packageJson = Get-Content $packageJsonPath -Raw | ConvertFrom-Json
    $packageJson.version = $Version
    $packageJson | ConvertTo-Json -Depth 10 | Set-Content $packageJsonPath -Encoding UTF8

    # 3. 更新 tauri.conf.json
    Write-Host "📝 更新 tauri.conf.json..." -ForegroundColor Yellow
    $tauriConfPath = "./src-tauri/tauri.conf.json"
    $tauriConf = Get-Content $tauriConfPath -Raw | ConvertFrom-Json
    $tauriConf.version = $Version
    $tauriConf | ConvertTo-Json -Depth 10 | Set-Content $tauriConfPath -Encoding UTF8

    # 4. 更新 Cargo.toml
    Write-Host "📝 更新 Cargo.toml..." -ForegroundColor Yellow
    $cargoTomlPath = "./src-tauri/Cargo.toml"
    $cargoContent = Get-Content $cargoTomlPath -Raw
    $cargoContent = $cargoContent -replace 'version = ".*"', "version = `"$Version`""
    Set-Content $cargoTomlPath -Value $cargoContent -Encoding UTF8

    # 5. 提交更改
    Write-Host "💾 提交版本更新..." -ForegroundColor Yellow
    git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml
    git commit -m "chore: bump version to v$Version"

    # 6. 创建并推送 tag
    Write-Host "🏷️  创建 Git tag..." -ForegroundColor Yellow
    git tag "v$Version"
    
    Write-Host "📤 推送到远程仓库..." -ForegroundColor Yellow
    git push
    git push origin "v$Version"

    Write-Host "✅ 发布成功！" -ForegroundColor Green
    Write-Host "🎉 版本 v$Version 已发布，GitHub Actions 将自动构建和部署" -ForegroundColor Green
    Write-Host "📊 查看构建状态: https://github.com/your-username/grape-planit/actions" -ForegroundColor Cyan
    
} catch {
    Write-Host "❌ 发布失败: $($_.Exception.Message)" -ForegroundColor Red
    
    # 清理可能创建的 tag
    try {
        git tag -d "v$Version" 2>$null
        Write-Host "🧹 已清理本地 tag" -ForegroundColor Yellow
    } catch {
        # 忽略删除 tag 的错误
    }
    
    exit 1
}

# 显示下一步操作提示
Write-Host ""
Write-Host "📋 接下来的步骤:" -ForegroundColor Cyan
Write-Host "1. 访问 GitHub Actions 页面查看构建进度" -ForegroundColor White
Write-Host "2. 构建完成后，检查 GitHub Releases 页面" -ForegroundColor White
Write-Host "3. 验证文件是否已上传到 Cloudflare R2" -ForegroundColor White
Write-Host "4. 测试应用更新功能（如果已配置）" -ForegroundColor White