<template>
  <div class="shortcut-settings">
    <div class="settings-section">
      <h3 class="section-title">全局快捷键</h3>
      <div class="shortcut-items">
        <div class="shortcut-item" :class="{ recording: recordingKey === 'toggle_window' }"
          @click="startRecordingShortcut('toggle_window')" @keydown="handleKeyDown($event, 'toggle_window')"
          tabindex="0" role="button" :aria-label="`设置快捷键: 显示/隐藏主窗口，当前值: ${currentSettings.toggle_window || '未设置'}`">
          <div class="shortcut-info">
            <label class="shortcut-label">显示/隐藏主窗口</label>
            <span class="shortcut-description">快速切换主窗口的显示状态</span>
          </div>
          <div class="shortcut-input-container">
            <div class="shortcut-display" :class="{
              recording: recordingKey === 'toggle_window',
              'has-value': currentSettings.toggle_window
            }">
              <span v-if="recordingKey === 'toggle_window'" class="recording-text">
                <span class="recording-icon">⌨️</span>
                录制中... (按任意组合键)
              </span>
              <span v-else-if="currentSettings.toggle_window" class="shortcut-keys">
                <span v-for="(key, index) in formatShortcutKeys(currentSettings.toggle_window)" :key="index"
                  class="key-badge">
                  {{ key }}
                </span>
              </span>
              <span v-else class="placeholder-text">
                <span class="click-icon">👆</span>
                点击设置快捷键
              </span>
            </div>
            <button @click.stop="resetShortcut('toggle_window')" class="reset-btn" title="重置为默认"
              :disabled="recordingKey === 'toggle_window'">
              ×
            </button>
          </div>
        </div>

        <div class="shortcut-item" :class="{ recording: recordingKey === 'quick_add_todo' }"
          @click="startRecordingShortcut('quick_add_todo')" @keydown="handleKeyDown($event, 'quick_add_todo')"
          tabindex="0" role="button" :aria-label="`设置快捷键: 快速创建待办，当前值: ${currentSettings.quick_add_todo || '未设置'}`">
          <div class="shortcut-info">
            <label class="shortcut-label">快速创建待办</label>
            <span class="shortcut-description">快速打开添加待办事项界面</span>
          </div>
          <div class="shortcut-input-container">
            <div class="shortcut-display" :class="{
              recording: recordingKey === 'quick_add_todo',
              'has-value': currentSettings.quick_add_todo
            }">
              <span v-if="recordingKey === 'quick_add_todo'" class="recording-text">
                <span class="recording-icon">⌨️</span>
                录制中... (按任意组合键)
              </span>
              <span v-else-if="currentSettings.quick_add_todo" class="shortcut-keys">
                <span v-for="(key, index) in formatShortcutKeys(currentSettings.quick_add_todo)" :key="index"
                  class="key-badge">
                  {{ key }}
                </span>
              </span>
              <span v-else class="placeholder-text">
                <span class="click-icon">👆</span>
                点击设置快捷键
              </span>
            </div>
            <button @click.stop="resetShortcut('quick_add_todo')" class="reset-btn" title="重置为默认"
              :disabled="recordingKey === 'quick_add_todo'">
              ×
            </button>
          </div>
        </div>
      </div>

      <!-- 使用说明 -->
      <div class="shortcut-help">
        <h4 class="help-title">使用说明</h4>
        <div class="help-items">
          <div class="help-item">
            <span class="help-key">点击</span>
            <span class="help-text">开始录制快捷键</span>
          </div>
          <div class="help-item">
            <span class="help-key">Enter/Space</span>
            <span class="help-text">键盘开始录制</span>
          </div>
          <div class="help-item">
            <span class="help-key">Esc</span>
            <span class="help-text">取消录制或重置</span>
          </div>
          <div class="help-item">
            <span class="help-key">Delete</span>
            <span class="help-text">重置快捷键</span>
          </div>
        </div>
      </div>
    </div>


  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, onUnmounted } from 'vue'
import { shortcutApi } from '@/api/services'
import { ShortcutSettings } from '@/model/settings'
import { useRecordingTimer } from '@/composables/useTimer'

// 当前快捷键设置
const currentSettings = ref<ShortcutSettings>({
  toggle_window: 'Alt+G',
  quick_add_todo: 'Alt+N'
})

// 默认快捷键设置
const defaultSettings: ShortcutSettings = {
  toggle_window: 'Alt+G',
  quick_add_todo: 'Alt+N'
}

// 录制状态管理
const recordingKey = ref<keyof ShortcutSettings | null>(null)

// 使用录制定时器管理
const { startRecording, stopRecording: stopRecordingTimer } = useRecordingTimer()

// 开始录制快捷键
const startRecordingShortcut = async (type: keyof ShortcutSettings) => {
  // 如果已经在录制其他快捷键，先停止
  if (recordingKey.value && recordingKey.value !== type) {
    stopRecording()
  }

  recordingKey.value = type

  // 8秒后自动停止录制（增加时间）
  startRecording(() => {
    stopRecording()
  }, 8000)

  await nextTick()
  // 聚焦到对应的快捷键项
  const element = document.querySelector(`[aria-label*="${type === 'toggle_window' ? '显示/隐藏主窗口' : '快速创建待办'}"]`) as HTMLElement
  if (element) {
    element.focus()
  }
}

// 停止录制
const stopRecording = () => {
  recordingKey.value = null
  stopRecordingTimer()
}

// 处理按键事件
const handleKeyDown = (event: KeyboardEvent, key: keyof ShortcutSettings) => {
  // 如果不在录制状态，检查是否是启动录制的按键
  if (recordingKey.value !== key) {
    // 按 Enter 或 Space 开始录制
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault()
      startRecordingShortcut(key)
    }
    // 按 Escape 或 Delete 重置快捷键
    else if (event.key === 'Escape' || event.key === 'Delete') {
      event.preventDefault()
      resetShortcut(key)
    }
    return
  }

  event.preventDefault()
  event.stopPropagation()

  // 按 Escape 取消录制
  if (event.key === 'Escape') {
    stopRecording()
    return
  }

  // 忽略单独的修饰键
  if (['Control', 'Alt', 'Shift', 'Meta'].includes(event.key)) {
    return
  }

  const keys: string[] = []

  // 修饰键
  if (event.ctrlKey) keys.push('Ctrl')
  if (event.altKey) keys.push('Alt')
  if (event.shiftKey) keys.push('Shift')
  if (event.metaKey) keys.push('Win')

  // 主键
  if (event.key && !['Control', 'Alt', 'Shift', 'Meta'].includes(event.key)) {
    let mainKey = event.key.toUpperCase()

    // 特殊键名映射
    const keyMap: { [key: string]: string } = {
      ' ': 'Space',
      'ARROWUP': 'Up',
      'ARROWDOWN': 'Down',
      'ARROWLEFT': 'Left',
      'ARROWRIGHT': 'Right',
      'ESCAPE': 'Esc',
      'DELETE': 'Del',
      'INSERT': 'Ins',
      'HOME': 'Home',
      'END': 'End',
      'PAGEUP': 'PageUp',
      'PAGEDOWN': 'PageDown',
      'BACKSPACE': 'Backspace',
      'TAB': 'Tab'
    }

    if (keyMap[mainKey]) {
      mainKey = keyMap[mainKey]
    }

    keys.push(mainKey)
  }

  if (keys.length >= 2) { // 至少需要一个修饰键
    const shortcut = keys.join('+')
    currentSettings.value[key] = shortcut
    stopRecording()
    saveSettings()
  }
}

// 重置快捷键
const resetShortcut = async (key: keyof ShortcutSettings) => {
  currentSettings.value[key] = defaultSettings[key]
  await saveSettings()
}

// 注册全局快捷键
const registerGlobalShortcuts = async () => {
  try {
    await shortcutApi.register(currentSettings.value)
    console.log('全局快捷键已注册')
  } catch (error) {
    console.error('注册全局快捷键失败:', error)
  }
}

// 保存设置到数据库
const saveSettings = async () => {
  try {
    console.log('🔧 [前端] 开始保存快捷键设置到数据库:', currentSettings.value)
    await shortcutApi.save(currentSettings.value)

    // 注册全局快捷键
    await registerGlobalShortcuts()

    console.log('✅ [前端] 快捷键设置已保存到数据库')
  } catch (error) {
    console.error('❌ [前端] 保存快捷键设置失败:', error)
  }
}

// 加载设置
const loadSettings = async () => {
  try {
    const settings = await shortcutApi.load()
    if (!settings) return
    currentSettings.value = settings

    // 注册全局快捷键
    await registerGlobalShortcuts()

    console.log('快捷键设置加载成功:', settings)
  } catch (error) {
    console.error('加载快捷键设置失败:', error)
  }
}

// 点击其他地方停止录制
const handleClickOutside = (event: Event) => {
  if (recordingKey.value && !(event.target as Element)?.closest('.shortcut-display')) {
    stopRecording()
  }
}

// 初始化设置
onMounted(() => {
  loadSettings()
  document.addEventListener('click', handleClickOutside)
})

// 格式化快捷键显示
const formatShortcutKeys = (shortcut: string) => {
  const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0
  return shortcut.split('+').map(key => {
    const trimmedKey = key.trim()
    if (isMac) {
      switch (trimmedKey.toLowerCase()) {
        case 'meta':
        case 'cmd':
        case 'super':
          return '⌘'
        case 'alt':
          return '⌥'
        case 'shift':
          return '⇧'
        case 'ctrl':
        case 'control':
          return '⌃'
        default:
          return trimmedKey
      }
    } else {
      switch (trimmedKey.toLowerCase()) {
        case 'meta':
        case 'cmd':
        case 'super':
          return 'Win'
        default:
          return trimmedKey
      }
    }
  })
}

// 组件卸载时清理
onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  // 录制定时器会在useRecordingTimer的onUnmounted中自动清理
})
</script>

<style scoped>
.shortcut-settings {
  padding: 20px;
}

.settings-section {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 24px;
  border: 1px solid var(--border-color);
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 20px 0;
}

.shortcut-items {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.shortcut-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  background: var(--bg-primary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;
  cursor: pointer;
  outline: none;
  position: relative;
}

.shortcut-item:hover {
  border-color: var(--accent-color);
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  transform: translateY(-1px);
}

.shortcut-item:focus {
  border-color: var(--accent-color);
  box-shadow: 0 0 0 3px rgba(var(--accent-color-rgb), 0.2);
  outline: none;
}

.shortcut-item.recording {
  border-color: var(--accent-color);
  box-shadow: 0 0 0 3px rgba(var(--accent-color-rgb), 0.3);
  background: linear-gradient(135deg, var(--bg-primary) 0%, rgba(var(--accent-color-rgb), 0.08) 100%);
  animation: recording-pulse 2s infinite;
}

.shortcut-item.recording::before {
  content: '';
  position: absolute;
  top: -2px;
  left: -2px;
  right: -2px;
  bottom: -2px;
  border-radius: 10px;
  background: linear-gradient(45deg, var(--accent-color), transparent, var(--accent-color));
  z-index: -1;
  animation: recording-border 3s linear infinite;
}

.shortcut-info {
  flex: 1;
}

.shortcut-label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.shortcut-description {
  font-size: 12px;
  color: var(--text-secondary);
}

.shortcut-input-container {
  display: flex;
  align-items: center;
  gap: 8px;
}

.shortcut-display {
  min-width: 140px;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12px;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  outline: none;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
}

.shortcut-display:hover {
  border-color: var(--accent-color);
  background: rgba(var(--accent-color-rgb), 0.05);
}

.shortcut-display:focus {
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px rgba(var(--accent-color-rgb), 0.2);
}

.shortcut-display.recording {
  border-color: var(--accent-color);
  background: rgba(var(--accent-color-rgb), 0.05);
  box-shadow: 0 0 0 1px rgba(var(--accent-color-rgb), 0.3);
}

.shortcut-display.has-value {
  background: rgba(var(--accent-color-rgb), 0.08);
}

.recording-text {
  color: var(--accent-color);
  font-weight: 500;
  font-size: 12px;
  animation: blink 1.5s infinite;
  display: flex;
  align-items: center;
  gap: 6px;
}

.recording-icon {
  font-size: 14px;
  animation: bounce 1s infinite;
}

.click-icon {
  font-size: 12px;
  margin-right: 4px;
  opacity: 0.8;
}

.shortcut-keys {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  flex-wrap: wrap;
}

.key-badge {
  display: inline-flex;
  align-items: center;
  padding: 1px 6px;
  background: var(--accent-color);
  color: white;
  border-radius: 3px;
  font-size: 10px;
  font-weight: 500;
  min-height: 16px;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.15);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', monospace;
}

.placeholder-text {
  color: var(--text-secondary);
  font-size: 12px;
  opacity: 0.8;
  display: flex;
  align-items: center;
  gap: 4px;
  font-weight: 400;
}

.reset-btn {
  width: 24px;
  height: 24px;
  padding: 0;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 50%;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 14px;
  font-weight: bold;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  line-height: 1;
}

.reset-btn:hover:not(:disabled) {
  background: var(--error-color, #ff4757);
  color: white;
  border-color: var(--error-color, #ff4757);
}

.reset-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

/* 快捷键说明样式 */
.shortcut-help {
  margin-top: 24px;
  padding: 20px;
  background: rgba(var(--accent-color-rgb), 0.03);
  border-radius: 8px;
  border: 1px solid rgba(var(--accent-color-rgb), 0.1);
}

.help-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 16px 0;
}

.help-items {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
  gap: 12px;
}

.help-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 6px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  transition: all 0.2s ease;
}

.help-item:hover {
  border-color: rgba(var(--accent-color-rgb), 0.3);
  background: rgba(var(--accent-color-rgb), 0.02);
}

.help-key {
  padding: 3px 8px;
  border-radius: 4px;
  background: var(--accent-color);
  color: white;
  font-size: 11px;
  font-weight: 500;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', monospace;
  min-width: fit-content;
  text-align: center;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.help-text {
  font-size: 12px;
  color: var(--text-secondary);
  flex: 1;
}

@keyframes blink {

  0%,
  50% {
    opacity: 1;
  }

  51%,
  100% {
    opacity: 0.6;
  }
}

@keyframes bounce {

  0%,
  20%,
  50%,
  80%,
  100% {
    transform: translateY(0);
  }

  40% {
    transform: translateY(-3px);
  }

  60% {
    transform: translateY(-2px);
  }
}

@keyframes recording-pulse {
  0% {
    box-shadow: 0 0 0 3px rgba(var(--accent-color-rgb), 0.3);
  }

  50% {
    box-shadow: 0 0 0 6px rgba(var(--accent-color-rgb), 0.1);
  }

  100% {
    box-shadow: 0 0 0 3px rgba(var(--accent-color-rgb), 0.3);
  }
}

@keyframes recording-border {
  0% {
    background-position: 0% 50%;
  }

  100% {
    background-position: 200% 50%;
  }
}

/* Windows/Mac 兼容性样式 */
@media (prefers-color-scheme: dark) {
  .shortcut-display {
    border-color: rgba(255, 255, 255, 0.1);
  }
}

/* Mac 样式调整 */
@supports (-webkit-backdrop-filter: blur(10px)) {
  .key-badge {
    backdrop-filter: blur(10px);
  }
}
</style>