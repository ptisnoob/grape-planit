<template>
  <div class="time-settings">
    <div class="settings-section">
      <h3 class="section-title">倒计时设置</h3>
      <ConfigTip 
        icon="⏰" 
        title="倒计时行为设置" 
        description="配置倒计时的显示行为，包括何时进入最后倒计时阶段以及结束状态的保持时间。" 
      />
      
      <div class="time-settings-grid">
        <div class="setting-item">
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
        
        <div class="setting-item">
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
  workEndTime: '',
  customCountdown: {
    name: '自定义事件',
    target: ''
  },
  showSeconds: true,
  timeDisplayMode: 'current',
  finalCountdownMinutes: 1,  // 默认1分钟
  endStateKeepMinutes: 5     // 默认5分钟
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
      // 确保新字段有默认值
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