<template>
  <Modal :visible="visible" title="设置下班时间" @close="handleClose">
    <div class="work-end-settings">
      <div class="time-input-container">
        <div class="time-input-group">
          <div class="input-section">
            <div class="input-pair">
              <input type="text" v-model="workEndHours[0]" @input="onHourInput(0, $event)" maxlength="1"
                class="time-input" placeholder="0">
              <input type="text" v-model="workEndHours[1]" @input="onHourInput(1, $event)" maxlength="1"
                class="time-input" placeholder="0">
            </div>
          </div>
          <span class="time-separator">:</span>
          <div class="input-section">
            <div class="input-pair">
              <input type="text" v-model="workEndMinutes[0]" @input="onMinuteInput(0, $event)" maxlength="1"
                class="time-input" placeholder="0">
              <input type="text" v-model="workEndMinutes[1]" @input="onMinuteInput(1, $event)" maxlength="1"
                class="time-input" placeholder="0">
            </div>
          </div>
        </div>
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
import { CountdownConfig } from '@/model/countdown'
import { useDatabase } from '@/composables/useDatabase'

interface Props {
  visible: boolean
  workEndTime: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  close: []
}>()

const { loadConfigFromDb, updateCountdownConfig: updateConfigInDb } = useDatabase()

const workEndHours = ref(['', ''])
const workEndMinutes = ref(['', ''])

// 监听props变化，初始化输入框
watch(() => props.workEndTime, (newTime) => {
  if (newTime) {
    const [hours, minutes] = newTime.split(':')
    workEndHours.value = [hours[0] || '', hours[1] || '']
    workEndMinutes.value = [minutes[0] || '', minutes[1] || '']
  } else {
    workEndHours.value = ['', '']
    workEndMinutes.value = ['', '']
  }
}, { immediate: true })

// 监听visible变化，重置输入框
watch(() => props.visible, (newVisible) => {
  if (newVisible) {
    if (props.workEndTime) {
      const [hours, minutes] = props.workEndTime.split(':')
      workEndHours.value = [hours[0] || '', hours[1] || '']
      workEndMinutes.value = [minutes[0] || '', minutes[1] || '']
    } else {
      workEndHours.value = ['', '']
      workEndMinutes.value = ['', '']
    }
  }
})

const onHourInput = (index: number, event: Event) => {
  const target = event.target as HTMLInputElement
  let value = target.value.replace(/[^0-9]/g, '')

  // 限制只能输入一个数字
  if (value.length > 1) {
    value = value.slice(-1)
  }

  workEndHours.value[index] = value

  // 验证小时范围
  if (value) {
    const hourStr = workEndHours.value.join('')
    if (hourStr.length === 2) {
      const hour = parseInt(hourStr)
      if (hour > 23) {
        // 如果超过23，重置为23
        workEndHours.value = ['2', '3']
      }
    }
  }
}

const onMinuteInput = (index: number, event: Event) => {
  const target = event.target as HTMLInputElement
  let value = target.value.replace(/[^0-9]/g, '')

  // 限制只能输入一个数字
  if (value.length > 1) {
    value = value.slice(-1)
  }

  workEndMinutes.value[index] = value

  // 验证分钟范围
  if (value) {
    const minuteStr = workEndMinutes.value.join('')
    if (minuteStr.length === 2) {
      const minute = parseInt(minuteStr)
      if (minute > 59) {
        // 如果超过59，重置为59
        workEndMinutes.value = ['5', '9']
      }
    }
  }
}

const handleSave = async () => {
  const hours = workEndHours.value.join('')
  const minutes = workEndMinutes.value.join('')

  if (hours.length === 2 && minutes.length === 2) {
    const timeString = `${hours}:${minutes}`
    try {
      // 优先从数据库加载配置
      let config: CountdownConfig
      try {
        config = await loadConfigFromDb()
      } catch (dbError) {
        console.warn('Failed to load from database, falling back to file:', dbError)
        config = await invoke('get_countdown_config')
      }
      
      config.workEndTime = timeString
      
      // 同时更新数据库和文件
      await updateConfigInDb(config)
      // 备份到文件以保持向后兼容性
      await invoke('update_countdown_config', { config })
      
      emit('close')
    } catch (error) {
      console.error('Failed to save work end time:', error)
    }
  }
}

const handleCancel = () => {
  emit('close')
}

const handleClose = () => {
  emit('close')
}
</script>

<style lang="scss" scoped>
.work-end-settings {
  min-width: 200px;
  overflow: hidden;
}

.time-input-container {
  display: flex;
  justify-content: center;
  margin: 8px 0;
}

.time-input-group {
  display: flex;
  align-items: center;
  gap: 10px;
}

.input-section {
  display: flex;
  align-items: center;
}

.input-pair {
  display: flex;
  gap: 4px;
}

.time-input {
  width: 32px;
  height: 40px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  text-align: center;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  background: var(--bg-secondary);
  transition: all var(--transition-normal);

  &:focus {
    outline: none;
    border-color: var(--accent-color);
    box-shadow: 0 0 0 2px rgba(var(--accent-color-rgb), 0.1);
  }

  &::placeholder {
    color: var(--text-placeholder);
  }
}

.time-separator {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
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