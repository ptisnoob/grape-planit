# SQLite 数据库使用指南

本项目已集成 SQLite 数据库来管理倒计时配置和记录。以下是详细的使用说明。

## 🚀 功能特性

- ✅ 倒计时配置的数据库存储
- ✅ 倒计时历史记录
- ✅ 自动数据库迁移
- ✅ 向后兼容文件配置
- ✅ TypeScript 类型支持

## 📁 项目结构

```
src/
├── types/
│   └── database.ts          # 数据库类型定义
├── composables/
│   └── useDatabase.ts       # 数据库操作组合函数
├── components/
│   └── DatabaseExample.vue  # 数据库使用示例组件
└── model/
    └── countdown.ts         # 更新的倒计时模型

src-tauri/
└── src/
    ├── lib.rs              # 包含数据库操作的 Rust 代码
    └── Cargo.toml          # 添加了 tauri-plugin-sql 依赖
```

## 🗄️ 数据库结构

### countdown_config 表
存储倒计时配置信息：

```sql
CREATE TABLE countdown_config (
    id INTEGER PRIMARY KEY,
    work_end_time TEXT NOT NULL,
    custom_countdown_name TEXT NOT NULL,
    custom_countdown_target TEXT NOT NULL,
    show_seconds BOOLEAN NOT NULL DEFAULT 1,
    time_display_mode TEXT NOT NULL DEFAULT 'remaining',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### countdown_records 表
存储倒计时历史记录：

```sql
CREATE TABLE countdown_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mode TEXT NOT NULL,
    target_time DATETIME,
    duration INTEGER,
    status TEXT NOT NULL DEFAULT 'running',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    finished_at DATETIME
);
```

## 🔧 Rust 后端 API

### 可用的 Tauri 命令

```rust
// 从数据库加载配置
load_config_from_db() -> Result<CountdownConfig, String>

// 保存配置到数据库
save_config_to_db(config: CountdownConfig) -> Result<(), String>

// 更新配置（同时更新内存和数据库）
update_countdown_config(config: CountdownConfig) -> Result<(), String>

// 保存倒计时记录
save_countdown_record(mode: String, target_time: Option<String>, duration: Option<i64>) -> Result<(), String>

// 启动倒计时定时器
start_countdown_timer() -> Result<(), String>
```

## 🎯 前端使用方法

### 1. 使用组合函数

```typescript
import { useDatabase } from '@/composables/useDatabase'

const {
  loadConfigFromDb,
  saveConfigToDb,
  updateCountdownConfig,
  saveCountdownRecord,
  startCountdownTimer
} = useDatabase()

// 加载配置
const config = await loadConfigFromDb()

// 保存配置
await updateCountdownConfig(newConfig)

// 启动倒计时
await startCountdownTimer()
```

### 2. 在 Vue 组件中使用

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useDatabase } from '@/composables/useDatabase'
import type { CountdownConfig } from '@/model/countdown'

const { loadConfigFromDb, updateCountdownConfig } = useDatabase()
const config = ref<CountdownConfig | null>(null)

// 组件挂载时加载配置
onMounted(async () => {
  try {
    config.value = await loadConfigFromDb()
  } catch (error) {
    console.error('Failed to load config:', error)
  }
})

// 保存配置
const saveConfig = async () => {
  if (config.value) {
    await updateCountdownConfig(config.value)
  }
}
</script>
```

### 3. 类型定义

```typescript
// 倒计时配置
interface CountdownConfig {
  workEndTime: string
  customCountdown: {
    name: string
    target: string
  }
  showSeconds: boolean
  timeDisplayMode: string
}

// 倒计时记录
interface CountdownRecord {
  id: number
  mode: string
  targetTime?: string
  duration?: number
  status: string
  createdAt: string
  finishedAt?: string
}
```

## 🔄 数据迁移

数据库迁移在应用启动时自动执行：

1. **版本 1**: 创建 `countdown_config` 表
2. **版本 2**: 创建 `countdown_records` 表

新的迁移可以通过在 `lib.rs` 中添加新的 `Migration` 结构体来实现。

## 🔒 向后兼容性

- 现有的文件配置系统仍然保留
- 配置更新时会同时保存到数据库和文件
- 如果数据库操作失败，会回退到文件系统

## 🚨 错误处理

所有数据库操作都包含错误处理：

```typescript
try {
  await saveConfigToDb(config)
  console.log('配置保存成功')
} catch (error) {
  console.error('保存失败:', error)
  // 处理错误，可能回退到文件保存
}
```

## 🔍 调试和监控

- 数据库文件位置：应用目录下的 `grape_planit.db`
- 可以使用 SQLite 浏览器工具查看数据库内容
- 所有数据库操作都有详细的错误日志

## 📝 示例组件

查看 `src/components/DatabaseExample.vue` 了解完整的使用示例，包括：

- 配置的加载和保存
- 倒计时记录的创建
- 错误处理和用户反馈
- 加载状态管理

## 🎉 开始使用

1. 确保已安装依赖：`npm install` 或 `yarn install`
2. 构建 Tauri 应用：`npm run tauri build` 或 `yarn tauri build`
3. 运行应用，数据库将自动创建和初始化
4. 使用提供的 API 和组合函数开始开发

现在你可以享受强大的 SQLite 数据库功能了！🎊