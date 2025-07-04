<template>
  <div class="general-settings">
    <div class="settings-section">
      <h3 class="section-title">主题设置</h3>
      <ConfigTip 
        icon="🎨" 
        title="主题设置说明" 
        description="选择应用的外观主题。自动模式会根据系统设置自动切换明暗主题。" 
      />
      <div class="theme-options">
        <label v-for="option in themeOptions" :key="option.value" class="theme-option"
          :class="{ active: currentSettings.theme === option.value }">
          <input type="radio" :value="option.value" :checked="currentSettings.theme === option.value"
            @change="handleThemeChange(option.value)">
          <span>{{ option.label }}</span>
        </label>
      </div>

      <div class="accent-color-section">
        <h4 class="subsection-title">主题色</h4>
        <div class="color-picker-container">
          <div class="preset-colors">
            <button v-for="color in presetColors" :key="color.value" class="color-preset"
              :class="{ active: currentSettings.accent_color === color.value }"
              :style="{ backgroundColor: color.value }" @click="handleAccentColorChange(color.value)"
              :title="color.name"></button>
          </div>
          <div class="custom-color">
            <input type="color" v-model="currentSettings.accent_color"
              @change="handleAccentColorChange(currentSettings.accent_color)" class="color-input">
            <span class="color-value">{{ currentSettings.accent_color }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <h3 class="section-title">窗口位置</h3>
      <ConfigTip 
        icon="📍" 
        title="窗口位置设置" 
        description="设置主窗口在屏幕上的显示位置。建议选择不影响日常工作的角落位置。" 
      />
      <div class="position-grid">
        <button v-for="option in positionOptions" :key="option.value" class="position-btn"
          :class="{ active: currentSettings.window_position === option.value }"
          @click="handlePositionChange(option.value)">
          {{ option.label }}
        </button>
      </div>
    </div>

    <div class="settings-section">
      <h3 class="section-title">窗口透明度</h3>
      <ConfigTip 
        icon="👻" 
        title="透明度调节" 
        description="调整窗口的透明度。较低的透明度可以让窗口更好地融入桌面背景，但可能影响内容可读性。" 
      />
      <div class="opacity-control">
        <input type="range" min="0.1" max="1" step="0.05" v-model="currentSettings.opacity"
          @input="handleOpacityChange(Number(currentSettings.opacity))" class="opacity-slider">
        <span class="opacity-value">{{ Math.round(currentSettings.opacity * 100) }}%</span>
      </div>
    </div>

    <div class="settings-section">
      <h3 class="section-title">窗口选项</h3>
      <ConfigTip 
        icon="📌" 
        title="窗口置顶说明" 
        description="开启后窗口将始终显示在其他应用程序之上，方便随时查看。" 
      />
      <div class="toggle-options">
        <label class="toggle-option">
          <input type="checkbox" v-model="currentSettings.always_on_top"
            @change="handleAlwaysOnTopChange(currentSettings.always_on_top)">
          <span class="toggle-text">窗口置顶</span>
        </label>
      </div>
    </div>

    <div class="settings-section">
      <h3 class="section-title">应用设置</h3>
      <ConfigTip 
        icon="⚙️" 
        title="应用行为设置" 
        description="TODO列表最近事项的范围和启动时默认显示的页面，自动模式会根据是否有最近事项来自动判断。" 
      />
      <div class="app-settings">
        <div class="setting-item">
          <label class="setting-label">最近事项范围</label>
          <div class="number-input-container">
            <input type="number" min="1" max="30" v-model="currentSettings.recent_days"
              @change="handleRecentDaysChange(currentSettings.recent_days)" class="number-input">
            <span class="input-suffix">天</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">默认启动页面</label>
          <div class="startup-options">
            <label v-for="option in startupOptions" :key="option.value" class="startup-option"
              :class="{ active: currentSettings.default_startup === option.value }">
              <input type="radio" :value="option.value" :checked="currentSettings.default_startup === option.value"
                @change="handleStartupChange(option.value)">
              <span>{{ option.label }}</span>
            </label>
          </div>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { WindowSettings } from '@/model/settings'
import { SelOption } from "@/model/public"
import { useTheme } from '@/composables/useTheme'
import ConfigTip from '@/components/ConfigTip.vue'

// 当前设置状态
const currentSettings = ref<WindowSettings>({
  theme: 'auto',
  window_position: 'bottom-right',
  opacity: 0.35,
  always_on_top: true,
  accent_color: '#007bff',
  recent_days: 5,
  default_startup: 'auto'
})

// 使用主题管理
const { setTheme } = useTheme()



// 主题选项
const themeOptions: SelOption[] = [
  { value: 'light', label: '白天' },
  { value: 'dark', label: '黑夜' },
  { value: 'auto', label: '自动' }
]

// 位置选项（只显示5个位置）
const positionOptions = [
  { value: 'top-left', label: '左上' },
  { value: 'top-right', label: '右上' },
  { value: 'center', label: '居中' },
  { value: 'bottom-left', label: '左下' },
  { value: 'bottom-right', label: '右下' }
]

// 预设主题色
const presetColors = [
  { name: '蓝色', value: '#007bff' },
  { name: '绿色', value: '#28a745' },
  { name: '红色', value: '#dc3545' },
  { name: '橙色', value: '#fd7e14' },
  { name: '紫色', value: '#6f42c1' },
  { name: '青色', value: '#20c997' },
  { name: '粉色', value: '#e83e8c' },
  { name: '黄色', value: '#ffc107' }
]

// 启动选项
const startupOptions: SelOption[] = [
  { value: 'auto', label: '自动' },
  { value: 'todo', label: 'Todo列表' },
  { value: 'home', label: '时间首页' }
]

// 保存设置到数据库
const saveSettings = async () => {
  try {
    console.log('🔧 [前端] 开始保存设置到数据库:', currentSettings.value)
    await invoke('save_window_settings_to_db', { settings: currentSettings.value })
    console.log('✅ [前端] 设置已保存到数据库')
  } catch (error) {
    console.error('❌ [前端] 保存设置失败:', error)
  }
}

// 获取系统主题
const getSystemTheme = (): 'light' | 'dark' => {
  const isDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;
  return isDarkMode ? 'dark' : 'light'
}

// 应用主题到主窗口
const applyThemeToMainWindow = async (theme: string) => {
  try {
    // 如果是auto模式，需要获取实际的系统主题
    let actualTheme = theme
    if (theme === 'auto') {
      actualTheme = getSystemTheme()
    }
    
    // 通过JavaScript在主窗口中设置主题
    const script = `document.documentElement.setAttribute('data-theme', '${actualTheme}')`
    await invoke('eval_script_in_main_window', { script })
  } catch (error) {
    console.error('应用主题到主窗口失败:', error)
  }
}

// 处理主题变更
const handleThemeChange = async (newTheme: string) => {
  currentSettings.value.theme = newTheme

  // 使用useTheme统一管理主题应用
  setTheme(newTheme as 'light' | 'dark' | 'auto')

  // 应用到主窗口
  await applyThemeToMainWindow(newTheme)

  // 保存到数据库
  await saveSettings()
}

// 处理窗口位置变更
const handlePositionChange = async (position: string) => {
  currentSettings.value.window_position = position
  try {
    await invoke('set_main_window_position', { position })
    await saveSettings()
    console.log('窗口位置设置成功:', position)
  } catch (error) {
    console.error('设置窗口位置失败:', error)
  }
}

// 处理透明度变更
const handleOpacityChange = async (opacity: number) => {
  console.log('🔧 [前端] 透明度变更触发:', opacity, typeof opacity)
  currentSettings.value.opacity = opacity
  try {
    await invoke('set_window_opacity', { opacity: currentSettings.value.opacity })
    await saveSettings()
    console.log('✅ [前端] 透明度设置成功:', currentSettings.value.opacity)
  } catch (error) {
    console.error('❌ [前端] 设置透明度失败:', error)
  }
}

// 处理置顶状态变更
const handleAlwaysOnTopChange = async (isOnTop: boolean) => {
  currentSettings.value.always_on_top = isOnTop
  try {
    await invoke('set_always_on_top', { alwaysOnTop: isOnTop })
    await saveSettings()
    console.log('窗口置顶设置成功:', isOnTop)
  } catch (error) {
    console.error('设置窗口置顶失败:', error)
  }
}

// 处理主题色变更
const handleAccentColorChange = async (color: string) => {
  currentSettings.value.accent_color = color

  // 应用主题色到当前窗口
  document.documentElement.style.setProperty('--accent-color', color)

  // 应用主题色到主窗口
  try {
    const script = `document.documentElement.style.setProperty('--accent-color', '${color}')`
    await invoke('eval_script_in_main_window', { script })
  } catch (error) {
    console.error('应用主题色到主窗口失败:', error)
  }

  // 保存到数据库
  await saveSettings()
  console.log('主题色设置成功:', color)
}

// 处理最近事项范围变更
const handleRecentDaysChange = async (days: number) => {
  currentSettings.value.recent_days = days
  await saveSettings()
  console.log('最近事项范围设置成功:', days)
}

// 处理默认启动设置变更
const handleStartupChange = async (startup: string) => {
  currentSettings.value.default_startup = startup
  await saveSettings()
  console.log('默认启动设置成功:', startup)
}



// 加载设置
const loadSettings = async () => {
  try {
    const settings = await invoke<WindowSettings>('load_window_settings_from_db')
    currentSettings.value = settings

    // 使用useTheme统一管理主题应用
    setTheme(settings.theme as 'light' | 'dark' | 'auto')

    // 应用主题色到当前窗口
    if (settings.accent_color) {
      document.documentElement.style.setProperty('--accent-color', settings.accent_color)
    }

    // 应用透明度
    await invoke('set_window_opacity', { opacity: settings.opacity })

    console.log('设置加载成功:', settings)
  } catch (error) {
    console.error('加载设置失败:', error)
  }
}



// 初始化设置
onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.general-settings {
  padding: 0 20px;
}

.settings-section {
  margin-bottom: 10px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 8px;
}

/* 主题选项样式 */
.theme-options {
  display: flex;
  gap: 12px;
}

.theme-option {
  display: flex;
  align-items: center;
  padding: 6px 16px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  cursor: pointer;
  transition: all var(--transition-normal);
  background: var(--bg-secondary);
}

.theme-option:hover {
  border-color: var(--accent-color);
}

.theme-option.active {
  border-color: var(--accent-color);
  background: var(--accent-color);
  color: white;
}

.theme-option input {
  display: none;
}

.theme-option span {
  font-size: 14px;
}

/* 位置网格样式 */
.position-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 8px;
  /* max-width: 240px; */
}

.position-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  transition: all var(--transition-normal);
  font-size: 12px;
}

.position-btn:hover {
  border-color: var(--accent-color);
}

.position-btn.active {
  border-color: var(--accent-color);
  background: var(--accent-color);
  color: white;
}

/* 透明度控制样式 */
.opacity-control {
  display: flex;
  align-items: center;
  gap: 16px;
}

.opacity-slider {
  flex: 1;
  max-width: 200px;
  height: 4px;
  border-radius: 2px;
  background: var(--border-color);
  outline: none;
  cursor: pointer;
}

.opacity-slider::-webkit-slider-thumb {
  appearance: none;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--accent-color);
  cursor: pointer;
}

.opacity-value {
  font-size: 14px;
  color: var(--text-secondary);
  min-width: 40px;
}



/* 切换选项样式 */
.toggle-options {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.toggle-option {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.toggle-option input[type="checkbox"] {
  width: 16px;
  height: 16px;
  accent-color: var(--accent-color);
}

.toggle-text {
  font-size: 14px;
  color: var(--text-primary);
}

/* 主题色设置样式 */
.accent-color-section {
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
}

.subsection-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 12px;
}

.color-picker-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.preset-colors {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.color-preset {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 2px solid transparent;
  cursor: pointer;
  transition: all var(--transition-fast);
  position: relative;
}

.color-preset:hover {
  transform: scale(1.1);
  border-color: var(--border-color);
}

.color-preset.active {
  border-color: var(--text-primary);
  transform: scale(1.15);
}

.color-preset.active::after {
  content: '✓';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: white;
  font-size: 12px;
  font-weight: bold;
  text-shadow: 0 0 2px rgba(0, 0, 0, 0.5);
}

.custom-color {
  display: flex;
  align-items: center;
  gap: 12px;
}

.color-input {
  width: 40px;
  height: 24px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  background: none;
}

.color-input::-webkit-color-swatch-wrapper {
  padding: 0;
}

.color-input::-webkit-color-swatch {
  border: none;
  border-radius: 3px;
}

.color-value {
  font-size: 12px;
  color: var(--text-secondary);
  font-family: monospace;
  text-transform: uppercase;
}

/* 应用设置样式 */
.app-settings {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.setting-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

/* 数字输入框样式 */
.number-input-container {
  display: flex;
  align-items: center;
  gap: 8px;
}

.number-input {
  width: 80px;
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color var(--transition-normal);
}

.number-input:focus {
  border-color: var(--accent-color);
}

.input-suffix {
  font-size: 14px;
  color: var(--text-secondary);
}

/* 启动选项样式 */
.startup-options {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.startup-option {
  display: flex;
  align-items: center;
  padding: 6px 16px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  cursor: pointer;
  transition: all var(--transition-normal);
  background: var(--bg-secondary);
}

.startup-option:hover {
  border-color: var(--accent-color);
}

.startup-option.active {
  border-color: var(--accent-color);
  background: var(--accent-color);
  color: white;
}

.startup-option input {
  display: none;
}

.startup-option span {
  font-size: 14px;
}
</style>