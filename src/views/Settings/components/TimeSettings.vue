<template>
  <div class="time-settings">
    <div class="settings-section">
      <h3 class="section-title">下班倒计时设置</h3>
      <ConfigTip 
        icon="⏰" 
        title="下班倒计时配置" 
        description="配置下班倒计时功能，包括是否启用、下班时间、工作日设置等。" 
      />
      
      <div class="time-settings-grid">
        <!-- 是否开启下班倒计时功能 -->
        <div class="setting-item">
          <label class="setting-label">启用下班倒计时</label>
          <div class="switch-container">
            <input 
              type="checkbox" 
              id="enableWorkEndCountdown"
              v-model="currentSettings.enableWorkEndCountdown"
              class="switch-input"
            >
            <label for="enableWorkEndCountdown" class="switch-label"></label>
          </div>
          <p class="setting-description">开启后将显示下班倒计时功能</p>
        </div>

        <!-- 下班时间设置 -->
        <div class="setting-item" v-if="currentSettings.enableWorkEndCountdown">
          <label class="setting-label">下班时间</label>
          <div class="time-input-container">
            <input 
              type="time" 
              v-model="currentSettings.workEndTime"
              class="time-input"
            >
          </div>
          <p class="setting-description">设置每日下班时间</p>
        </div>

        <!-- 工作日设置 -->
        <div class="setting-item" v-if="currentSettings.enableWorkEndCountdown">
          <label class="setting-label">工作日设置</label>
          <div class="radio-group">
            <label class="radio-item">
              <input 
                type="radio" 
                value="single" 
                v-model="currentSettings.workDays"
                class="radio-input"
              >
              <span class="radio-label">单休（周日休息）</span>
            </label>
            <label class="radio-item">
              <input 
                type="radio" 
                value="double" 
                v-model="currentSettings.workDays"
                class="radio-input"
              >
              <span class="radio-label">双休（周六日休息）</span>
            </label>
          </div>
          <p class="setting-description">选择工作日模式，休息日不会显示下班倒计时</p>
        </div>
        
        <!-- 最后倒计时触发时间 -->
        <div class="setting-item" v-if="currentSettings.enableWorkEndCountdown">
          <label class="setting-label">最后倒计时触发时间</label>
          <div class="number-input-container">
            <input 
              type="number" 
              min="1" 
              max="60" 
              v-model="currentSettings.finalCountdownMinutes"
              class="number-input"
            >
            <span class="input-suffix">分钟</span>
          </div>
          <p class="setting-description">当倒计时剩余时间少于此值时，将进入最后倒计时阶段</p>
        </div>
        
        <!-- 结束状态保持时间 -->
        <div class="setting-item" v-if="currentSettings.enableWorkEndCountdown">
          <label class="setting-label">结束状态保持时间</label>
          <div class="number-input-container">
            <input 
              type="number" 
              min="1" 
              max="30" 
              v-model="currentSettings.endStateKeepMinutes"
              class="number-input"
            >
            <span class="input-suffix">分钟</span>
          </div>
          <p class="setting-description">倒计时结束后，保持结束状态显示的时间</p>
        </div>
      </div>
      
      <div class="action-buttons">
        <button class="btn btn-primary" @click="saveSettings">
          <i class="icon-save"></i>
          保存设置
        </button>
        <button class="btn btn-secondary" @click="resetToDefault">
          <i class="icon-reset"></i>
          恢复默认
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { CountdownConfig } from '@/model/countdown'
import { useDatabase } from '@/composables/useDatabase'
import ConfigTip from '@/components/ConfigTip.vue'

// 当前设置状态
const currentSettings = ref<CountdownConfig>({
  workEndTime: '18:00',
  showSeconds: true,
  timeDisplayMode: 'current',
  enableWorkEndCountdown: true,
  finalCountdownMinutes: 1,  // 默认1分钟
  endStateKeepMinutes: 5,    // 默认5分钟
  workDays: 'double'         // 默认双休
})

const { loadConfigFromDb, updateCountdownConfig } = useDatabase()

// 保存设置
const saveSettings = async () => {
  try {
    console.log('🔧 [前端] 开始保存时间设置:', currentSettings.value)
    
    await updateCountdownConfig(currentSettings.value)
    console.log('✅ [前端] 时间设置已保存')
    
    // 可以添加成功提示
    // showSuccessMessage('设置保存成功')
  } catch (error) {
    console.error('❌ [前端] 保存时间设置失败:', error)
    // 可以添加错误提示
    // showErrorMessage('保存设置失败')
  }
}

// 恢复默认设置
const resetToDefault = () => {
  currentSettings.value.enableWorkEndCountdown = true
  currentSettings.value.workEndTime = '18:00'
  currentSettings.value.workDays = 'double'
  currentSettings.value.finalCountdownMinutes = 1
  currentSettings.value.endStateKeepMinutes = 5
  console.log('🔄 [前端] 时间设置已恢复默认值')
}

// 加载设置
const loadSettings = async () => {
  try {
    const config = await loadConfigFromDb() as CountdownConfig 
    currentSettings.value = {
      ...config,
      // 确保新字段有默认值，但保持 enableWorkEndCountdown 的原始值
      enableWorkEndCountdown: config.enableWorkEndCountdown !== undefined ? config.enableWorkEndCountdown : true,
      workDays: config.workDays || 'double',
      finalCountdownMinutes: config.finalCountdownMinutes || 1,
      endStateKeepMinutes: config.endStateKeepMinutes || 5
    }
    console.log('✅ [前端] 时间设置加载成功:', currentSettings.value)
  } catch (error) {
    console.error('❌ [前端] 加载时间设置失败:', error)
  }
}

// 初始化
onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.time-settings {
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

.time-settings-grid {
  display: flex;
  flex-direction: column;
  gap: 24px;
  margin-bottom: 24px;
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

.setting-description {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0;
  opacity: 0.8;
}

/* 开关样式 */
.switch-container {
  display: flex;
  align-items: center;
}

.switch-input {
  display: none;
}

.switch-label {
  position: relative;
  width: 44px;
  height: 24px;
  background: var(--border-color);
  border-radius: 12px;
  cursor: pointer;
  transition: background-color var(--transition-normal);
}

.switch-label::after {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  background: white;
  border-radius: 50%;
  transition: transform var(--transition-normal);
}

.switch-input:checked + .switch-label {
  background: var(--accent-color);
}

.switch-input:checked + .switch-label::after {
  transform: translateX(20px);
}

/* 时间输入框样式 */
.time-input-container {
  display: flex;
  align-items: center;
}

.time-input {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color var(--transition-normal);
}

.time-input:focus {
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px rgba(var(--accent-color-rgb), 0.1);
}

/* 单选按钮样式 */
.radio-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.radio-item {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.radio-input {
  width: 16px;
  height: 16px;
  border: 2px solid var(--border-color);
  border-radius: 50%;
  background: var(--bg-secondary);
  cursor: pointer;
  transition: all var(--transition-normal);
  appearance: none;
  position: relative;
}

.radio-input:checked {
  border-color: var(--accent-color);
  background: var(--accent-color);
}

.radio-input:checked::after {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 6px;
  height: 6px;
  background: white;
  border-radius: 50%;
}

.radio-label {
  font-size: 14px;
  color: var(--text-primary);
  cursor: pointer;
}

/* 数字输入框样式 */
.number-input-container {
  display: flex;
  align-items: center;
  gap: 8px;
}

.number-input {
  width: 100px;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color var(--transition-normal);
}

.number-input:focus {
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px rgba(var(--accent-color-rgb), 0.1);
}

.input-suffix {
  font-size: 14px;
  color: var(--text-secondary);
}

/* 按钮样式 */
.action-buttons {
  display: flex;
  gap: 12px;
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
}

.btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: all var(--transition-normal);
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px var(--shadow);
}

.btn-primary {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.btn-primary:hover {
  background: var(--accent-color-hover, var(--accent-color));
  filter: brightness(1.1);
}

.btn-secondary {
  background: var(--bg-secondary);
  color: var(--text-secondary);
}

.btn-secondary:hover {
  color: var(--text-primary);
  border-color: var(--accent-color);
}

/* 图标样式 */
.icon-save::before {
  content: '💾';
}

.icon-reset::before {
  content: '🔄';
}
</style>