# 🚀 部署和发布指南

本指南将帮助你设置完整的 CI/CD 流程，实现代码提交后自动构建并部署到 Cloudflare R2。

## 📋 前置要求

- GitHub 仓库
- Cloudflare 账户
- Node.js 和 Yarn/NPM
- Git

## 🔧 快速设置

### 1. 配置 Cloudflare R2

#### 创建 R2 存储桶
1. 登录 [Cloudflare Dashboard](https://dash.cloudflare.com/)
2. 进入 **R2 Object Storage**
3. 点击 **Create bucket**
4. 输入存储桶名称（如：`grape-planit-releases`）
5. 选择区域并创建

#### 获取 API 凭据
1. 在 Cloudflare Dashboard 中，进入 **My Profile** > **API Tokens**
2. 点击 **Create Token**
3. 选择 **Custom token**
4. 配置权限：
   - **Account**: `Cloudflare R2:Edit`
   - **Zone Resources**: `Include - All zones`
5. 创建后保存 **Access Key ID** 和 **Secret Access Key**

#### 获取账户信息
1. 在 Cloudflare Dashboard 右侧找到 **Account ID**
2. R2 端点格式：`https://[Account-ID].r2.cloudflarestorage.com`

### 2. 配置 GitHub Secrets

在 GitHub 仓库中设置以下 Secrets：

1. 进入仓库 **Settings** > **Secrets and variables** > **Actions**
2. 点击 **New repository secret** 添加以下密钥：

```
CLOUDFLARE_R2_ACCESS_KEY_ID=你的R2访问密钥ID
CLOUDFLARE_R2_SECRET_ACCESS_KEY=你的R2秘密访问密钥
CLOUDFLARE_R2_BUCKET=你的存储桶名称
CLOUDFLARE_R2_ENDPOINT=https://你的账户ID.r2.cloudflarestorage.com
```

### 3. 配置应用签名（可选）

如果需要应用自动更新功能：

```bash
# 生成签名密钥
npx @tauri-apps/cli signer generate -w ~/.tauri/myapp.key

# 获取公钥
npx @tauri-apps/cli signer sign -k ~/.tauri/myapp.key --password "" dummy_file
```

在 GitHub Secrets 中添加：
```
TAURI_SIGNING_PRIVATE_KEY=私钥内容
TAURI_SIGNING_PRIVATE_KEY_PASSWORD=私钥密码（如果有）
```

## 🎯 发布新版本

### 方法一：使用自动化脚本（推荐）

```bash
# 使用 Node.js 脚本
npm run release 1.0.1

# 或使用 PowerShell 脚本（Windows）
npm run release:ps 1.0.1
```

### 方法二：手动发布

1. **更新版本号**：
   ```bash
   # 手动编辑以下文件中的版本号：
   # - package.json
   # - src-tauri/tauri.conf.json
   # - src-tauri/Cargo.toml
   ```

2. **提交更改**：
   ```bash
   git add .
   git commit -m "chore: bump version to v1.0.1"
   git push
   ```

3. **创建标签**：
   ```bash
   git tag v1.0.1
   git push origin v1.0.1
   ```

## 📦 构建产物

发布后，构建产物将自动上传到 R2，目录结构如下：

```
your-bucket/
├── releases/
│   └── v1.0.1/
│       ├── windows/
│       │   ├── PlanIt_1.0.1_x64_en-US.msi
│       │   └── PlanIt_1.0.1_x64_en-US.msi.sig
│       ├── darwin-x64/
│       │   ├── PlanIt_1.0.1_x64.dmg
│       │   └── PlanIt_1.0.1_x64.dmg.sig
│       ├── darwin-aarch64/
│       │   ├── PlanIt_1.0.1_aarch64.dmg
│       │   └── PlanIt_1.0.1_aarch64.dmg.sig
│       ├── linux/
│       │   ├── plan-it_1.0.1_amd64.deb
│       │   ├── plan-it_1.0.1_amd64.AppImage
│       │   └── *.sig
│       └── manifest.json
└── grape/
    └── updates/
        └── latest.json
```

## 🔍 监控和调试

### 查看构建状态
- GitHub Actions: `https://github.com/你的用户名/grape-planit/actions`
- 构建日志中包含详细的错误信息

### 常见问题

#### 构建失败
1. 检查 GitHub Secrets 是否正确配置
2. 验证 Tauri 配置文件语法
3. 查看 Actions 日志中的具体错误

#### R2 上传失败
1. 验证 R2 API 凭据是否正确
2. 检查存储桶权限设置
3. 确认端点 URL 格式正确

#### 应用更新失败
1. 检查签名密钥配置
2. 验证 `latest.json` 文件格式
3. 确认更新器端点可访问

## 🛠️ 自定义配置

### 修改构建平台
编辑 `.github/workflows/release.yml` 中的 `matrix` 部分：

```yaml
matrix:
  include:
    - platform: 'windows-latest'
      args: ''
    # 添加或移除其他平台
```

### 修改存储路径
在工作流文件中修改上传路径：

```bash
aws s3 cp "$file" "s3://$CLOUDFLARE_R2_BUCKET/custom-path/$VERSION/$filename"
```

### 添加通知
可以在工作流中添加 Slack、Discord 或邮件通知：

```yaml
- name: Notify on success
  if: success()
  run: |
    # 添加通知逻辑
```

## 📚 相关文档

- [Tauri 官方文档](https://tauri.app/)
- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [Cloudflare R2 文档](https://developers.cloudflare.com/r2/)
- [项目 GitHub Actions 配置](./.github/README.md)

## 🆘 获取帮助

如果遇到问题：
1. 查看 [GitHub Issues](https://github.com/你的用户名/grape-planit/issues)
2. 检查 [GitHub Actions 日志](https://github.com/你的用户名/grape-planit/actions)
3. 参考 [故障排除指南](./.github/README.md#故障排除)