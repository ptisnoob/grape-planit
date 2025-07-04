import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAppStore = defineStore('app', () => {
  // 是否已经执行过启动设置检查
  const hasCheckedStartupSettings = ref(false)
  
  // 设置启动设置检查状态
  const setStartupSettingsChecked = (checked: boolean) => {
    hasCheckedStartupSettings.value = checked
  }
  
  // 重置应用状态（用于测试或重新初始化）
  const resetAppState = () => {
    hasCheckedStartupSettings.value = false
  }
  
  return {
    hasCheckedStartupSettings,
    setStartupSettingsChecked,
    resetAppState
  }
})