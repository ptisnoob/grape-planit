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
import { CustomCountdown } from '@/model/countdown'
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

// 时间格式转换工具函数
const convertRfc3339ToDatetimeLocal = (rfc3339Time: string): string => {
  if (!rfc3339Time || !rfc3339Time.includes('T') || 
      !(rfc3339Time.includes('Z') || rfc3339Time.includes('+') || rfc3339Time.includes('-'))) {
    return rfc3339Time // 如果不是 RFC3339 格式，直接返回
  }
  
  // 转换为 datetime-local 格式 (YYYY-MM-DDTHH:mm)
  const date = new Date(rfc3339Time)
  return date.toISOString().slice(0, 16) // 截取到分钟，去掉秒和时区
}

const convertDatetimeLocalToRfc3339 = (datetimeLocal: string): string => {
  if (!datetimeLocal) return datetimeLocal
  
  // 将 datetime-local 格式转换为 RFC3339 格式
  const targetDateTime = new Date(datetimeLocal)
  return targetDateTime.toISOString()
}

// 监听props变化，同步到本地状态
watch(() => props.customCountdown, (newCountdown) => {
  localCountdown.value = {
    ...newCountdown,
    target: convertRfc3339ToDatetimeLocal(newCountdown.target)
  }
}, { immediate: true, deep: true })

// 监听visible变化，重置本地状态
watch(() => props.visible, (newVisible) => {
  if (newVisible) {
    localCountdown.value = {
      ...props.customCountdown,
      target: convertRfc3339ToDatetimeLocal(props.customCountdown.target)
    }
  }
})

const handleSave = async () => {
  if (localCountdown.value.name.trim() && localCountdown.value.target) {
    try {
      // 从数据库加载配置
      const config = await loadConfigFromDb()

      // 转换时间格式：从 datetime-local 格式转换为 RFC3339 格式
      config.customCountdown = {
        ...localCountdown.value,
        target: convertDatetimeLocalToRfc3339(localCountdown.value.target)
      }

      // 更新数据库配置
      await updateConfigInDb(config)

      emit('close')
    } catch (error) {
      console.error('Failed to save custom countdown:', error)
    }
  }
}

const handleCancel = () => {
  // 重置为原始值，并转换时间格式
  localCountdown.value = {
    ...props.customCountdown,
    target: convertRfc3339ToDatetimeLocal(props.customCountdown.target)
  }
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