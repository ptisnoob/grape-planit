<template>
  <div class="database-example">
    <h3>数据库操作示例</h3>
    
    <!-- 配置管理 -->
    <div class="config-section">
      <h4>配置管理</h4>
      <button @click="loadConfig" :disabled="loading">从数据库加载配置</button>
      <button @click="saveConfig" :disabled="loading">保存配置到数据库</button>
      
      <div v-if="config" class="config-display">
        <p>工作结束时间: {{ config.workEndTime }}</p>
        <p>自定义倒计时: {{ config.customCountdown.name }}</p>
        <p>显示秒数: {{ config.showSeconds ? '是' : '否' }}</p>
        <p>时间显示模式: {{ config.timeDisplayMode }}</p>
      </div>
    </div>
    
    <!-- 倒计时记录 -->
    <div class="record-section">
      <h4>倒计时记录</h4>
      <button @click="saveRecord" :disabled="loading">保存倒计时记录</button>
      <button @click="startTimer" :disabled="loading">启动倒计时</button>
    </div>
    
    <!-- 状态显示 -->
    <div v-if="message" class="message" :class="messageType">
      {{ message }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useDatabase } from '../composables/useDatabase'
import type { CountdownConfig } from '../model/countdown'

const { 
  loadConfigFromDb, 
  saveConfigToDb, 
  updateCountdownConfig, 
  saveCountdownRecord, 
  startCountdownTimer 
} = useDatabase()

const loading = ref(false)
const message = ref('')
const messageType = ref<'success' | 'error'>('success')
const config = ref<CountdownConfig | null>(null)

// 加载配置
const loadConfig = async () => {
  loading.value = true
  try {
    config.value = await loadConfigFromDb()
    showMessage('配置加载成功', 'success')
  } catch (error) {
    showMessage(`加载配置失败: ${error}`, 'error')
  } finally {
    loading.value = false
  }
}

// 保存配置
const saveConfig = async () => {
  if (!config.value) {
    showMessage('请先加载配置', 'error')
    return
  }
  
  loading.value = true
  try {
    await updateCountdownConfig(config.value)
    showMessage('配置保存成功', 'success')
  } catch (error) {
    showMessage(`保存配置失败: ${error}`, 'error')
  } finally {
    loading.value = false
  }
}

// 保存倒计时记录
const saveRecord = async () => {
  loading.value = true
  try {
    await saveCountdownRecord('test', new Date().toISOString(), 3600)
    showMessage('倒计时记录保存成功', 'success')
  } catch (error) {
    showMessage(`保存记录失败: ${error}`, 'error')
  } finally {
    loading.value = false
  }
}

// 启动倒计时
const startTimer = async () => {
  loading.value = true
  try {
    await startCountdownTimer()
    showMessage('倒计时启动成功', 'success')
  } catch (error) {
    showMessage(`启动倒计时失败: ${error}`, 'error')
  } finally {
    loading.value = false
  }
}

// 显示消息
const showMessage = (msg: string, type: 'success' | 'error') => {
  message.value = msg
  messageType.value = type
  setTimeout(() => {
    message.value = ''
  }, 3000)
}
</script>

<style scoped>
.database-example {
  padding: 20px;
  max-width: 600px;
  margin: 0 auto;
}

.config-section,
.record-section {
  margin-bottom: 30px;
  padding: 15px;
  border: 1px solid #ddd;
  border-radius: 8px;
}

.config-display {
  margin-top: 15px;
  padding: 10px;
  background-color: #f5f5f5;
  border-radius: 4px;
}

button {
  margin-right: 10px;
  margin-bottom: 10px;
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  background-color: #007bff;
  color: white;
  cursor: pointer;
  transition: background-color 0.2s;
}

button:hover:not(:disabled) {
  background-color: #0056b3;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.message {
  padding: 10px;
  border-radius: 4px;
  margin-top: 15px;
}

.message.success {
  background-color: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
}

.message.error {
  background-color: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}

h3, h4 {
  color: #333;
  margin-bottom: 15px;
}
</style>