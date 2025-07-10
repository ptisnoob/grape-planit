# GitHub Actions CI/CD 配置说明

本项目使用 GitHub Actions 实现自动化构建和部署到 Cloudflare R2 对象存储。

## 工作流说明

### 1. Release 工作流 (`release.yml`)

**触发条件**: 推送带有 `v*.*.*` 格式的 tag（如 `v1.0.0`）

**功能**:
- 多平台构建（Windows、macOS、Linux）
- 自动创建 GitHub Release
- 上传构建产物到 Cloudflare R2
- 生成更新器清单文件
- 创建版本发布清单

### 2. Build 工作流 (`build.yml`)

**触发条件**: 
- 推送到 `main` 或 `develop` 分支
- 向 `main` 分支提交 Pull Request

**功能**:
- 代码质量检查和测试构建
- 开发分支预览版本构建
- 预览版本上传到 R2（可选）

## 必需的 GitHub Secrets 配置

在 GitHub 仓库的 Settings > Secrets and variables > Actions 中添加以下密钥：

### Cloudflare R2 配置
```
CLOUDFLARE_R2_ACCESS_KEY_ID=your_r2_access_key_id
CLOUDFLARE_R2_SECRET_ACCESS_KEY=your_r2_secret_access_key
CLOUDFLARE_R2_BUCKET=your_bucket_name
CLOUDFLARE_R2_ENDPOINT=https://your_account_id.r2.cloudflarestorage.com
```

### Tauri 签名配置（可选，用于应用更新）
```
TAURI_SIGNING_PRIVATE_KEY=your_private_key_content
TAURI_SIGNING_PRIVATE_KEY_PASSWORD=your_private_key_password
```

## Cloudflare R2 配置步骤

### 1. 创建 R2 存储桶
1. 登录 Cloudflare Dashboard
2. 进入 R2 Object Storage
3. 创建新的存储桶
4. 记录存储桶名称

### 2. 创建 API 令牌
1. 进入 Cloudflare Dashboard > My Profile > API Tokens
2. 创建自定义令牌
3. 权限设置：
   - Account: `Cloudflare R2:Edit`
   - Zone Resources: `Include - All zones`
4. 记录 Access Key ID 和 Secret Access Key

### 3. 获取账户 ID
1. 在 Cloudflare Dashboard 右侧找到账户 ID
2. R2 端点格式：`https://[账户ID].r2.cloudflarestorage.com`

## 发布新版本

1. 更新版本号：
   ```bash
   # 更新 package.json 中的版本号
   # 更新 src-tauri/tauri.conf.json 中的版本号
   # 更新 src-tauri/Cargo.toml 中的版本号
   ```

2. 提交更改：
   ```bash
   git add .
   git commit -m "chore: bump version to v1.0.1"
   git push
   ```

3. 创建并推送 tag：
   ```bash
   git tag v1.0.1
   git push origin v1.0.1
   ```

4. GitHub Actions 将自动：
   - 构建所有平台的应用
   - 创建 GitHub Release
   - 上传文件到 Cloudflare R2
   - 更新应用更新器清单

## R2 存储结构

```
your-bucket/
├── releases/
│   ├── v1.0.0/
│   │   ├── windows/
│   │   │   ├── PlanIt_1.0.0_x64_en-US.msi
│   │   │   └── PlanIt_1.0.0_x64_en-US.msi.sig
│   │   ├── darwin-x64/
│   │   │   ├── PlanIt_1.0.0_x64.dmg
│   │   │   └── PlanIt_1.0.0_x64.dmg.sig
│   │   ├── darwin-aarch64/
│   │   │   ├── PlanIt_1.0.0_aarch64.dmg
│   │   │   └── PlanIt_1.0.0_aarch64.dmg.sig
│   │   ├── linux/
│   │   │   ├── plan-it_1.0.0_amd64.deb
│   │   │   ├── plan-it_1.0.0_amd64.AppImage
│   │   │   └── *.sig
│   │   └── manifest.json
│   └── v1.0.1/
│       └── ...
├── grape/
│   └── updates/
│       └── latest.json
└── previews/
    └── [commit-sha]/
        └── ...
```

## 故障排除

### 构建失败
1. 检查 GitHub Actions 日志
2. 确认所有必需的 secrets 已正确配置
3. 验证 Tauri 配置文件语法

### R2 上传失败
1. 验证 R2 API 凭据
2. 检查存储桶权限
3. 确认端点 URL 格式正确

### 应用更新器问题
1. 确认签名密钥配置正确
2. 检查 `latest.json` 文件格式
3. 验证更新器端点 URL

## 本地测试

在推送 tag 之前，可以本地测试构建：

```bash
# 安装依赖
yarn install

# 构建前端
yarn build

# 构建 Tauri 应用
cd src-tauri
cargo tauri build
```

## 自定义配置

可以根据需要修改工作流文件：
- 调整构建平台
- 修改 R2 存储路径
- 添加额外的构建步骤
- 配置通知机制