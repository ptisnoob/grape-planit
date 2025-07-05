<template>
  <div class="shortcut-settings">
    <div class="settings-section">
      <h3 class="section-title">全局快捷键</h3>
      <div class="shortcut-items">
        <div class="shortcut-item" :class="{ recording: recordingKey === 'toggle_window' }">
          <div class="shortcut-info">
            <label class="shortcut-label">显示/隐藏主窗口</label>
            <span class="shortcut-description">快速切换主窗口的显示状态</span>
          </div>
          <div class="shortcut-input-container">
            <div 
              class="shortcut-display"
              :class="{ 
                recording: recordingKey === 'toggle_window',
                'has-value': currentSettings.toggle_window
              }"
              @click="startRecording('toggle_window')"
              @keydown="handleKeyDown($event, 'toggle_window')"
              tabindex="0"
            >
              <span v-if="recordingKey === 'toggle_window'" class="recording-text">
                录制中
              </span>
              <span v-else-if="currentSettings.toggle_window" class="shortcut-keys">
                <span 
                  v-for="(key, index) in formatShortcutKeys(currentSettings.toggle_window)"
                  :key="index"
                  class="key-badge"
                >
                  {{ key }}
                </span>
              </span>
              <span v-else class="placeholder-text">点击设置</span>
            </div>
            <button 
              @click="resetShortcut('toggle_window')"
              class="reset-btn"
              title="重置为默认"
              :disabled="recordingKey === 'toggle_window'"
            >
              ×
            </button>
          </div>
        </div>

        <div class="shortcut-item" :class="{ recording: recordingKey === 'quick_add_todo' }">
          <div class="shortcut-info">
            <label class="shortcut-label">快速创建待办</label>
            <span class="shortcut-description">快速打开添加待办事项界面</span>
          </div>
          <div class="shortcut-input-container">
            <div 
              class="shortcut-display"
              :class="{ 
                recording: recordingKey === 'quick_add_todo',
                'has-value': currentSettings.quick_add_todo
              }"
              @click="startRecording('quick_add_todo')"
              @keydown="handleKeyDown($event, 'quick_add_todo')"
              tabindex="0"
            >
              <span v-if="recordingKey === 'quick_add_todo'" class="recording-text">
                录制中
              </span>
              <span v-else-if="currentSettings.quick_add_todo" class="shortcut-keys">
                <span 
                  v-for="(key, index) in formatShortcutKeys(currentSettings.quick_add_todo)"
                  :key="index"
                  class="key-badge"
                >
                  {{ key }}
                </span>
              </span>
              <span v-else class="placeholder-text">点击设置快捷键</span>
            </div>
            <button 
              @click="resetShortcut('quick_add_todo')"
              class="reset-btn"
              title="重置为默认"
              :disabled="recordingKey === 'quick_add_todo'"
            >
              ×
            </button>
          </div>
        </div>
      </div>
    </div>


  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ShortcutSettings } from '@/model/settings'

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
const recordingTimeout = ref<NodeJS.Timeout | null>(null)

// 开始录制快捷键
const startRecording = async (type: keyof ShortcutSettings) => {
  recordingKey.value = type
  
  // 清除之前的超时
  if (recordingTimeout.value) {
    clearTimeout(recordingTimeout.value)
  }
  
  // 5秒后自动停止录制
  recordingTimeout.value = setTimeout(() => {
    stopRecording()
  }, 5000)
  
  await nextTick()
  // 聚焦到对应的显示区域
  const element = document.querySelector(`[tabindex="0"]`) as HTMLElement
  if (element) {
    element.focus()
  }
}

// 停止录制
const stopRecording = () => {
  recordingKey.value = null
  if (recordingTimeout.value) {
    clearTimeout(recordingTimeout.value)
    recordingTimeout.value = null
  }
}

// 处理按键事件
const handleKeyDown = (event: KeyboardEvent, key: keyof ShortcutSettings) => {
  if (recordingKey.value !== key) return
  
  event.preventDefault()
  event.stopPropagation()
  
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
      'PAGEDOWN': 'PageDown'
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
    await invoke('register_global_shortcuts', {
      settings: currentSettings.value
    })
    console.log('全局快捷键已注册')
  } catch (error) {
    console.error('注册全局快捷键失败:', error)
  }
}

// 保存设置到数据库
const saveSettings = async () => {
  try {
    console.log('🔧 [前端] 开始保存快捷键设置到数据库:', currentSettings.value)
    await invoke('save_shortcut_settings_to_db', { settings: currentSettings.value })
    
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
    const settings = await invoke<ShortcutSettings>('load_shortcut_settings_from_db')
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
  if (recordingTimeout.value) {
    clearTimeout(recordingTimeout.value)
  }
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
}

.shortcut-item:hover {
  border-color: var(--accent-color);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.shortcut-item.recording {
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px rgba(var(--accent-color-rgb), 0.2);
  background: linear-gradient(135deg, var(--bg-primary) 0%, rgba(var(--accent-color-rgb), 0.05) 100%);
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
  font-weight: 400;
  font-size: 11px;
  animation: blink 1.5s infinite;
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
  font-size: 11px;
  opacity: 0.7;
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
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 12px;
}

.help-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 6px;
  background: var(--bg-secondary);
}

.help-key {
  padding: 2px 6px;
  border-radius: 3px;
  background: var(--accent-color);
  color: white;
  font-size: 11px;
  font-weight: 500;
  font-family: monospace;
}

.help-text {
  font-size: 12px;
  color: var(--text-secondary);
}

@keyframes blink {
  0%, 50% {
    opacity: 1;
  }
  51%, 100% {
    opacity: 0.6;
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