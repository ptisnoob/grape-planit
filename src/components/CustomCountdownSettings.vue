<template>
  <Modal :visible="visible" title="自定义倒计时设置" @close="handleClose">
    <div class="custom-countdown-settings">
      <div class="setting-group">
        <input type="text" v-model="localCountdown.name" placeholder="倒计时名称" class="setting-input">
      </div>

      <div class="setting-group">
        <input type="datetime-local" v-model="localCountdown.target" class="setting-input datetime-input"
          placeholder="目标时间">
      </div>
    </div>

    <template #footer>
      <button class="btn-secondary" @click="handleCancel">取消</button>
      <button class="btn-primary" @click="handleSave">保存</button>
    </template>
  </Modal>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue'
import Modal from './Modal.vue'
import { invoke } from '@tauri-apps/api/core'
import { CustomCountdown, CountdownConfig } from '@/model/countdown'
import { useDatabase } from '@/composables/useDatabase'

interface Props {
  visible: boolean
  customCountdown: CustomCountdown
}

const props = defineProps<Props>()

const emit = defineEmits<{
  close: []
}>()

const { loadConfigFromDb, updateCountdownConfig: updateConfigInDb } = useDatabase()

const localCountdown = ref<CustomCountdown>({
  name: '',
  target: ''
})

// 监听props变化，同步到本地状态
watch(() => props.customCountdown, (newCountdown) => {
  localCountdown.value = { ...newCountdown }
}, { immediate: true, deep: true })

// 监听visible变化，重置本地状态
watch(() => props.visible, (newVisible) => {
  if (newVisible) {
    localCountdown.value = { ...props.customCountdown }
  }
})

const handleSave = async () => {
  if (localCountdown.value.name.trim() && localCountdown.value.target) {
    try {
      // 优先从数据库加载配置
      let config: CountdownConfig
      try {
        config = await loadConfigFromDb()
      } catch (dbError) {
        console.warn('Failed to load from database, falling back to file:', dbError)
        config = await invoke('get_countdown_config')
      }
      
      config.customCountdown = { ...localCountdown.value }
      
      // 同时更新数据库和文件
      await updateConfigInDb(config)
      // 备份到文件以保持向后兼容性
      await invoke('update_countdown_config', { config })
      
      emit('close')
    } catch (error) {
      console.error('Failed to save custom countdown:', error)
    }
  }
}

const handleCancel = () => {
  // 重置为原始值
  localCountdown.value = { ...props.customCountdown }
  emit('close')
}

const handleClose = () => {
  emit('close')
}
</script>

<style lang="scss" scoped>
.custom-countdown-settings {
  min-width: 260px;
  overflow: hidden;
}

.setting-group {
  margin-bottom: 8px;

  &:last-child {
    margin-bottom: 0;
  }
}

.setting-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 13px;
  color: var(--text-primary);
  background: var(--bg-secondary);
  transition: all var(--transition-normal);
  box-sizing: border-box;

  &:focus {
    outline: none;
    border-color: var(--accent-color);
    box-shadow: 0 0 0 2px rgba(var(--accent-color-rgb), 0.1);
  }

  &::placeholder {
    color: var(--text-placeholder);
  }
}

.datetime-input {
  font-family: inherit;

  &::-webkit-calendar-picker-indicator {
    background-color: var(--text-secondary);
    border-radius: 3px;
    cursor: pointer;
  }
}

.btn-primary {
  padding: 6px 16px;
  border: none;
  border-radius: 4px;
  background: var(--accent-color);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-normal);

  &:hover {
    background: var(--accent-color-hover);
    transform: translateY(-1px);
  }

  &:disabled {
    background: var(--text-placeholder);
    cursor: not-allowed;
    transform: none;
  }
}

.btn-secondary {
  padding: 6px 16px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-normal);

  &:hover {
    background: var(--bg-hover);
    border-color: var(--accent-color);
  }
}
</style>