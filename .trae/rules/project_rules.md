# Grape-PlanIt 项目开发规范

## 技术栈
- **前端框架**: Vue 3.5+ + TypeScript 5.6+
- **桌面框架**: Tauri 2.x
- **状态管理**: Pinia 3.x
- **路由管理**: Vue Router 4.x
- **构建工具**: Vite 6.x
- **样式预处理**: Sass/SCSS
- **动画库**: Animate.css 4.x
- **后端语言**: Rust (Edition 2021)
- **数据库**: SQLite + SQLx

## 项目结构规范

### 前端目录结构
```
src/
├── assets/          # 静态资源
│   ├── css/         # 全局样式文件
│   ├── fonts/       # 字体文件
│   ├── icons/       # 图标资源
│   └── img/         # 图片资源
├── common/          # 通用工具函数
├── components/      # 可复用组件
├── composables/     # Vue 组合式函数
├── layout/          # 布局组件
├── model/           # TypeScript 类型定义
├── router/          # 路由配置
├── store/           # Pinia 状态管理
├── views/           # 页面组件
└── types/           # 全局类型定义
```

### 后端目录结构
```
src-tauri/src/
├── config.rs        # 配置管理
├── countdown.rs     # 倒计时功能
├── database.rs      # 数据库操作
├── todo.rs          # 待办事项功能
├── window_commands.rs # 窗口控制命令
├── lib.rs           # 库入口
└── main.rs          # 主程序入口
```

## 代码规范

### Vue 组件规范
1. **组件命名**: 使用 PascalCase，如 `AppHeader.vue`
2. **组合式 API**: 优先使用 `<script setup lang="ts">`
3. **Props 定义**: 使用 TypeScript 接口定义 props 类型
4. **事件定义**: 使用 `defineEmits` 明确定义组件事件
5. **样式作用域**: 使用 `<style lang="scss" scoped>`

### TypeScript 规范
1. **类型定义**: 统一放在 `src/model/` 目录下
2. **接口命名**: 使用 PascalCase，如 `Todo`、`TodoVo`
3. **严格模式**: 启用 TypeScript 严格模式
4. **路径别名**: 使用 `@/` 指向 `src/` 目录

### Rust 代码规范
1. **命名约定**: 使用 snake_case
2. **错误处理**: 统一返回 `Result<T, String>` 类型
3. **序列化**: 使用 serde 进行数据序列化/反序列化
4. **数据库操作**: 使用 SQLx 进行异步数据库操作

### 样式规范
1. **CSS 变量**: 使用 CSS 自定义属性管理主题色彩
2. **响应式设计**: 优先使用 Flexbox 布局
3. **动画效果**: 统一使用 Animate.css 类名
4. **字体**: 默认使用 'DaoLiTi' 字体
5. **样式重置**: 统一在 `base.css` 中进行样式重置

## 功能模块规范

### 窗口管理
- **主窗口**: 450x250px，无边框，置顶，透明背景
- **设置窗口**: 570x400px，有边框，居中显示
- **系统托盘**: 支持左键显示/隐藏，右键菜单操作

### 数据模型
1. **Todo 数据结构**:
   - `id`: 唯一标识符
   - `title`: 待办标题
   - `level`: 优先级 (0-3)
   - `cycle`: 循环类型 ('one'|'day'|'week'|'month'|'year')
   - `status`: 状态 (0:未开始, 1:已完成, 2:已删除)

2. **时间戳**: 统一使用秒级时间戳
3. **数据库**: 使用 SQLite，支持数据迁移

### 路由规范
- **主页面**: `/` - 倒计时显示
- **列表页**: `/list` - 待办事项列表
- **添加页**: `/add` - 添加待办事项
- **日历页**: `/calendar` - 日历视图
- **设置页**: `/settings` - 应用设置
- **关于页**: `/about` - 关于信息

### 状态管理
1. **App Store**: 应用全局状态
2. **Mode Store**: 模式切换状态
3. **Weather Store**: 天气信息状态

## 开发工具配置

### 构建配置
- **开发服务器**: 端口自动分配，支持 HMR
- **环境变量**: 使用 `VITE_` 和 `TAURI_ENV_*` 前缀
- **SVG 加载**: 使用 vite-svg-loader 插件

### 全局快捷键
- 支持全局快捷键快速添加待办事项
- 自动聚焦到输入框

## 安全规范
1. **CSP**: 根据需要配置内容安全策略
2. **数据验证**: 前后端双重数据验证
3. **SQL 注入防护**: 使用参数化查询

## 性能优化
1. **懒加载**: 路由组件使用动态导入
2. **代码分割**: 按需加载第三方库
3. **资源优化**: 使用 SVG 图标，避免大图片
4. **数据库优化**: 合理使用索引，避免 N+1 查询

## 测试规范
- 优先编写单元测试
- 关键功能需要集成测试
- 使用 TypeScript 确保类型安全

## 版本控制
- 使用语义化版本号
- 提交信息使用约定式提交格式
- 重要功能变更需要更新文档