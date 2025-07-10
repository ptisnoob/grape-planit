#!/usr/bin/env node

/**
 * 自动化发布脚本
 * 用法: node scripts/release.js [版本号]
 * 例如: node scripts/release.js 1.0.1
 */

import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// 获取命令行参数
const args = process.argv.slice(2);
if (args.length === 0) {
  console.error('❌ 请提供版本号，例如: node scripts/release.js 1.0.1');
  process.exit(1);
}

const newVersion = args[0];

// 验证版本号格式
if (!/^\d+\.\d+\.\d+$/.test(newVersion)) {
  console.error('❌ 版本号格式不正确，应为 x.y.z 格式');
  process.exit(1);
}

console.log(`🚀 开始发布版本 ${newVersion}`);

try {
  // 1. 检查工作目录是否干净
  console.log('📋 检查 Git 状态...');
  try {
    execSync('git diff --exit-code', { stdio: 'pipe' });
    execSync('git diff --cached --exit-code', { stdio: 'pipe' });
  } catch (error) {
    console.error('❌ 工作目录不干净，请先提交所有更改');
    process.exit(1);
  }

  // 2. 更新 package.json
  console.log('📝 更新 package.json...');
  const packageJsonPath = path.join(__dirname, '../package.json');
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
  packageJson.version = newVersion;
  fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2) + '\n');

  // 3. 更新 tauri.conf.json
  console.log('📝 更新 tauri.conf.json...');
  const tauriConfPath = path.join(__dirname, '../src-tauri/tauri.conf.json');
  const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
  tauriConf.version = newVersion;
  fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');

  // 4. 更新 Cargo.toml
  console.log('📝 更新 Cargo.toml...');
  const cargoTomlPath = path.join(__dirname, '../src-tauri/Cargo.toml');
  let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
  cargoToml = cargoToml.replace(/^version = ".*"$/m, `version = "${newVersion}"`);
  fs.writeFileSync(cargoTomlPath, cargoToml);

  // 5. 提交更改
  console.log('💾 提交版本更新...');
  execSync('git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml');
  execSync(`git commit -m "chore: bump version to v${newVersion}"`);

  // 6. 创建并推送 tag
  console.log('🏷️  创建 Git tag...');
  execSync(`git tag v${newVersion}`);
  
  console.log('📤 推送到远程仓库...');
  execSync('git push');
  execSync(`git push origin v${newVersion}`);

  console.log('✅ 发布成功！');
  console.log(`🎉 版本 v${newVersion} 已发布，GitHub Actions 将自动构建和部署`);
  console.log('📊 查看构建状态: https://github.com/your-username/grape-planit/actions');
  
} catch (error) {
  console.error('❌ 发布失败:', error.message);
  
  // 清理可能创建的 tag
  try {
    execSync(`git tag -d v${newVersion}`, { stdio: 'pipe' });
  } catch (e) {
    // 忽略删除 tag 的错误
  }
  
  process.exit(1);
}

/**
 * 辅助函数：执行命令并返回输出
 */
function execCommand(command) {
  try {
    return execSync(command, { encoding: 'utf8' }).trim();
  } catch (error) {
    throw new Error(`命令执行失败: ${command}\n${error.message}`);
  }
}