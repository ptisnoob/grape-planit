<template>
  <div class="ai-settings">
    <div class="settings-section">
      <h3 class="section-title">AI 配置</h3>
      <ConfigTip icon="🤖" title="AI服务配置" description="配置AI服务的连接信息,推荐采用智谱的免费flash模型." />
      <div v-if="testResult" class="test-result" :class="{ success: testResult.success, error: !testResult.success }">
        {{ testResult.message }}
      </div>
      <div class="ai-config">
        <div class="input-group">
          <label class="input-label">API Key</label>
          <ConfigTip icon="🔑" title="API密钥" description="从AI服务提供商获取的身份验证密钥，用于访问AI服务。请妥善保管，不要泄露给他人。"
            :show="!aiSettings.apiKey" />
          <input type="password" v-model="aiSettings.apiKey" placeholder="请输入 API Key" class="config-input">
        </div>
        <div class="input-group">
          <label class="input-label">Base URL</label>
          <ConfigTip icon="🌐" title="服务地址" description="AI服务的API基础地址,符合openai格式都可,例：https://open.bigmodel.cn/api/paas/v4"
            :show="!aiSettings.baseUrl || aiSettings.baseUrl === 'https://open.bigmodel.cn/api/paas/v4'" />
          <input type="text" v-model="aiSettings.baseUrl" placeholder="https://open.bigmodel.cn/api/paas/v4" class="config-input">
        </div>
        <div class="input-group">
          <label class="input-label">模型</label>
          <input type="text" v-model="aiSettings.model" placeholder="glm-4-flash-250414" class="config-input">
        </div>
        <div class="ai-actions">
          <button @click="testAIConnection" :disabled="isTestingConnection" class="test-btn"
            :class="{ testing: isTestingConnection }">
            {{ isTestingConnection ? '测试中...' : '测试连接' }}
          </button>
          <button @click="saveAISettings" :disabled="isSavingAI" class="save-btn" :class="{ saving: isSavingAI }">
            {{ isSavingAI ? '保存中...' : '保存配置' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import defaultAIService from '@/common/ai'
import { useUIFeedbackTimer } from '@/composables/useTimer'
import ConfigTip from '@/components/ConfigTip.vue'

// AI配置状态
const aiSettings = ref({
  apiKey: '',
  baseUrl: 'https://open.bigmodel.cn/api/paas/v4',
  model: 'glm-4-flash-250414'
})

// AI相关状态
const isTestingConnection = ref(false)
const isSavingAI = ref(false)
const testResult = ref(null)

// 使用UI反馈定时器管理
const { createFeedbackTimer } = useUIFeedbackTimer()

// 加载AI设置
const loadAISettings = async () => {
  try {
    await defaultAIService.loadConfigFromDB()
    const config = defaultAIService.getConfig()
    aiSettings.value = {
      apiKey: config.apiKey,
      baseUrl: config.baseUrl,
      model: config.model
    }
    console.log('AI设置加载成功:', config)
  } catch (error) {
    console.error('加载AI设置失败:', error)
  }
}

// 保存AI设置
const saveAISettings = async () => {
  isSavingAI.value = true
  testResult.value = null

  try {
    await defaultAIService.saveConfigToDB(aiSettings.value)
    testResult.value = {
      success: true,
      message: 'AI配置保存成功！'
    }
    console.log('AI设置保存成功')
  } catch (error) {
    console.error('保存AI设置失败:', error)
    testResult.value = {
      success: false,
      message: `保存失败: ${error.message || error}`
    }
  } finally {
    isSavingAI.value = false
    // 3秒后清除结果提示
    createFeedbackTimer(() => {
      testResult.value = null
    }, 3000, 'aiSaveResult')
  }
}

// 测试AI连接
const testAIConnection = async () => {
  isTestingConnection.value = true
  testResult.value = null

  try {
    // 先更新AI服务的配置
    defaultAIService.updateConfig(aiSettings.value)

    // 测试连接
    const result = await defaultAIService.testConnection()
    testResult.value = result

    console.log('AI连接测试结果:', result)
  } catch (error) {
    console.error('AI连接测试失败:', error)
    testResult.value = {
      success: false,
      message: `测试失败: ${error.message || error}`
    }
  } finally {
    isTestingConnection.value = false
    // 5秒后清除结果提示
    // setTimeout(() => {
    //   testResult.value = null
    // }, 5000)
  }
}

// 初始化设置
onMounted(() => {
  loadAISettings()
})
</script>

<style scoped>
.ai-settings {
  padding: 0 20px;
}

.settings-section {
  margin-bottom: 10px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 10px;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 8px;
}

/* AI配置样式 */
.ai-config {
  display: flex;
  flex-direction: column;
  gap: 11px 16px;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.input-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.config-input {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  transition: all var(--transition-normal);
}

.config-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px rgba(var(--accent-color-rgb), 0.1);
}

.config-input::placeholder {
  color: var(--text-secondary);
}

.ai-actions {
  display: flex;
  gap: 12px;
  margin-top: 4px;
}

.test-btn,
.save-btn {
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: all var(--transition-normal);
}

.test-btn:hover,
.save-btn:hover {
  border-color: var(--accent-color);
  background: var(--accent-color);
  color: white;
}

.test-btn:disabled,
.save-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.test-btn.testing,
.save-btn.saving {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.test-result {
  padding: 8px 12px;
  border-radius: 6px;
  font-size: 14px;
  margin-bottom: 8px;
}

.test-result.success {
  background: rgba(34, 197, 94, 0.1);
  color: #22c55e;
  border: 1px solid rgba(34, 197, 94, 0.2);
}

.test-result.error {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
  border: 1px solid rgba(239, 68, 68, 0.2);
}
</style>