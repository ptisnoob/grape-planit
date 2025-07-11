<template>
  <div class="holiday-sync">
    <div class="sync-section">
      <div class="sync-header">
        <h3>节假日数据同步</h3>
        <p class="description">从 holiday-cn 开源仓库同步中国法定节假日数据</p>
      </div>

      <div class="sync-controls">
        <div class="year-input">
          <label>同步年份:</label>
          <input v-model.number="syncYear" type="number" :min="2020" :max="2030" placeholder="如: 2024" />
        </div>
        <button @click="syncHolidayData" :disabled="syncing || !syncYear" class="sync-btn">
          <span v-if="syncing" class="loading">同步中...</span>
          <span v-else>同步数据</span>
        </button>
      </div>

      <div class="proxy-section">
        <div class="proxy-header">
          <label class="proxy-toggle">
            <input type="checkbox" v-model="useProxy" />
            <span>使用代理服务器</span>
          </label>
        </div>
        <div v-if="useProxy" class="proxy-input">
          <input v-model="proxyUrl" type="text" placeholder="如: https://ghfast.top/" class="proxy-url-input" />
          <div class="proxy-help">
            <small>代理后的请求地址示例: {{ proxyExample }}</small>
          </div>
        </div>
      </div>

      <div v-if="syncMessage" class="sync-message" :class="syncMessageType">
        {{ syncMessage }}
      </div>
    </div>

    <div class="stored-data-section">
      <div class="section-header">
        <h4>已存储数据</h4>
        <button @click="refreshStoredData" class="refresh-btn" :disabled="loading">
          <span v-if="loading">刷新中...</span>
          <span v-else>刷新</span>
        </button>
      </div>

      <div v-if="loading && storedYears.length === 0" class="loading-state">
        加载中...
      </div>

      <div v-else-if="storedYears.length === 0" class="empty-state">
        暂无已存储的节假日数据
      </div>

      <div v-else class="stored-list">
        <div v-for="yearData in storedYears" :key="yearData.year" class="stored-item">
          <div class="year-info">
            <span class="year">{{ yearData.year }}年</span>
            <span class="count">{{ yearData.count }}个节假日</span>
          </div>
          <div class="sync-info">
            <span class="sync-time">{{ formatSyncTime(yearData.syncTime) }}</span>
          </div>
          <div class="actions">
            <button @click="resyncYear(yearData.year)" :disabled="syncing" class="resync-btn">
              重新同步
            </button>
            <button @click="deleteYear(yearData.year)" :disabled="syncing" class="delete-btn">
              删除
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import { holidayApi } from '@/api/holiday';
import type { StoredHolidayYear } from '@/model/holiday';

const syncYear = ref<number>(new Date().getFullYear());
const syncing = ref(false);
const loading = ref(false);
const syncMessage = ref('');
const syncMessageType = ref<'success' | 'error' | ''>('');
const storedYears = ref<StoredHolidayYear[]>([]);
const useProxy = ref(false);
const proxyUrl = ref('');
const savingProxy = ref(false);

// 同步节假日数据
const syncHolidayData = async () => {
  if (!syncYear.value || syncing.value) return;

  syncing.value = true;
  syncMessage.value = '';
  syncMessageType.value = '';

  try {
    const success = await holidayApi.sync({ year: syncYear.value });
    if (success) {
      syncMessage.value = `${syncYear.value}年节假日数据同步成功！`;
      syncMessageType.value = 'success';
      await refreshStoredData();
    } else {
      syncMessage.value = '同步失败，请稍后重试';
      syncMessageType.value = 'error';
    }
  } catch (error: any) {
    console.error('同步节假日数据失败:', error);
    if (error.message?.includes('数据为空')) {
      syncMessage.value = `${syncYear.value}年的节假日数据尚未发布，请稍后再试`;
    } else if (error.message?.includes('网络')) {
      syncMessage.value = '网络连接失败，请检查网络后重试';
    } else {
      syncMessage.value = `同步失败: ${error.message || '未知错误'}`;
    }
    syncMessageType.value = 'error';
  } finally {
    syncing.value = false;
    // 3秒后清除消息
    setTimeout(() => {
      syncMessage.value = '';
      syncMessageType.value = '';
    }, 3000);
  }
};

// 重新同步指定年份
const resyncYear = async (year: number) => {
  syncYear.value = year;
  await syncHolidayData();
};

// 获取代理示例URL
const proxyExample = computed(() => {
  if (!useProxy.value || !proxyUrl.value.trim()) {
    return `https://raw.githubusercontent.com/NateScarlet/holiday-cn/master/${syncYear.value}.json`;
  }
  const proxy = proxyUrl.value.trim().replace(/\/$/, '');
  return `${proxy}/https://raw.githubusercontent.com/NateScarlet/holiday-cn/master/${syncYear.value}.json`;
});

// 删除指定年份数据
const deleteYear = async (year: number) => {
  if (!confirm(`确定要删除${year}年的节假日数据吗？`)) return;

  try {
    const success = await holidayApi.deleteYear(year);
    if (success) {
      await refreshStoredData();
    } else {
      alert('删除失败，请稍后重试');
    }
  } catch (error) {
    console.error('删除节假日数据失败:', error);
    alert('删除失败，请稍后重试');
  }
};

// 刷新已存储数据
const refreshStoredData = async () => {
  loading.value = true;
  try {
    storedYears.value = await holidayApi.getStoredYears();
  } catch (error) {
    console.error('获取已存储数据失败:', error);
  } finally {
    loading.value = false;
  }
};

// 格式化同步时间
const formatSyncTime = (syncTime: string) => {
  try {
    const date = new Date(syncTime);
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    });
  } catch {
    return syncTime;
  }
};

// 加载代理配置
const loadProxySettings = async () => {
  try {
    const settings = await holidayApi.getProxySettings();
    if (settings) {
      useProxy.value = settings.enabled;
      proxyUrl.value = settings.proxyUrl ?? 'https://ghfast.top/';
    }
  } catch (error) {
    console.error('加载代理配置失败:', error);
  }
};

// 保存代理配置
const saveProxySettings = async () => {
  if (savingProxy.value) return;

  savingProxy.value = true;
  try {
    await holidayApi.saveProxySettings({
      enabled: useProxy.value,
      proxyUrl: proxyUrl.value || ''
    });
  } catch (error) {
    console.error('保存代理配置失败:', error);
  } finally {
    savingProxy.value = false;
  }
};

// 监听代理配置变化，自动保存
watch([useProxy, proxyUrl], () => {
  saveProxySettings();
}, { deep: true });

// 组件挂载时加载数据
onMounted(() => {
  refreshStoredData();
  loadProxySettings();
});
</script>

<style scoped>
.holiday-sync {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.sync-section {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 16px;
  border: 1px solid var(--border-color);
}

.sync-header h3 {
  margin: 0 0 4px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.description {
  margin: 0 0 16px 0;
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.4;
}

.sync-controls {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.year-input {
  display: flex;
  align-items: center;
  gap: 8px;
}

.year-input label {
  font-size: 13px;
  color: var(--text-primary);
  white-space: nowrap;
}

.year-input input {
  width: 80px;
  padding: 6px 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 13px;
}

.year-input input:focus {
  outline: none;
  border-color: var(--accent-color);
}

.sync-btn {
  padding: 6px 12px;
  background: var(--accent-color);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.sync-btn:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.sync-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.loading {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.loading::after {
  content: '';
  width: 12px;
  height: 12px;
  border: 2px solid transparent;
  border-top: 2px solid currentColor;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.sync-message {
  padding: 8px 12px;
  border-radius: 4px;
  font-size: 13px;
  line-height: 1.4;
}

.sync-message.success {
  background: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
}

.sync-message.error {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}

.proxy-section {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
}

.proxy-header {
  margin-bottom: 12px;
}

.proxy-toggle {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 13px;
  color: var(--text-primary);
}

.proxy-toggle input[type="checkbox"] {
  margin: 0;
  cursor: pointer;
}

.proxy-input {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.proxy-url-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 13px;
  font-family: 'Consolas', 'Monaco', monospace;
}

.proxy-url-input:focus {
  outline: none;
  border-color: var(--accent-color);
}

.proxy-help {
  padding: 8px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-family: 'Consolas', 'Monaco', monospace;
}

.proxy-help small {
  color: var(--text-secondary);
  font-size: 12px;
  word-break: break-all;
}

.stored-data-section {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 16px;
  border: 1px solid var(--border-color);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.section-header h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.refresh-btn {
  padding: 4px 8px;
  background: transparent;
  color: var(--accent-color);
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.refresh-btn:hover:not(:disabled) {
  background: var(--accent-color);
  color: white;
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.loading-state,
.empty-state {
  text-align: center;
  padding: 20px;
  color: var(--text-secondary);
  font-size: 13px;
}

.stored-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.stored-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  transition: all 0.2s;
}

.stored-item:hover {
  border-color: var(--accent-color);
}

.year-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.year {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.count {
  font-size: 12px;
  color: var(--text-secondary);
}

.sync-info {
  flex: 1;
  text-align: center;
}

.sync-time {
  font-size: 12px;
  color: var(--text-secondary);
}

.actions {
  display: flex;
  gap: 6px;
}

.resync-btn,
.delete-btn {
  padding: 4px 8px;
  border: none;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.resync-btn {
  background: var(--accent-color);
  color: white;
}

.resync-btn:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.delete-btn {
  background: #dc3545;
  color: white;
}

.delete-btn:hover:not(:disabled) {
  background: #c82333;
}

.resync-btn:disabled,
.delete-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>