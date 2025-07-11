import { defineStore } from 'pinia'
import { ref } from 'vue'
import { CountdownData } from '@/model/countdown'

export const useFinalCountdownStore = defineStore('finalCountdown', () => {
  // 状态
  const isVisible = ref(false)
  const countdownData = ref<CountdownData | null>(null)
  const beforeTime = ref(60) // 进入最后倒计时的阈值（秒）
  const isInEndState = ref(false)
  const isInFinalCountdown = ref(false)
  const userManuallyExited = ref(false) // 用户是否主动退出标记

  // 显示最后倒计时overlay
  const showOverlay = (data: CountdownData, threshold: number = 60) => {
    countdownData.value = data
    beforeTime.value = threshold
    isVisible.value = true
    
    if (data.status === 'finished') {
      isInEndState.value = true
    } else {
      isInFinalCountdown.value = true
    }
  }

  // 隐藏最后倒计时overlay
  const hideOverlay = (manualExit: boolean = false) => {
    isVisible.value = false
    countdownData.value = null
    isInEndState.value = false
    isInFinalCountdown.value = false
    
    if (manualExit) {
      userManuallyExited.value = true
    }
  }

  // 检查是否应该显示最后倒计时
  const shouldShowFinalCountdown = (data: CountdownData, threshold: number = 60): boolean => {
    if (!data) return false
    
    // 如果状态是finished，显示最后倒计时效果
    if (data.status === 'finished') {
      return true
    }
    
    // 如果状态是running且时间小于等于threshold，显示最后倒计时
    if (data.status === 'running' && data.timestamp > 0 && data.timestamp <= threshold) {
      return true
    }
    
    return false
  }

  // 更新倒计时数据
  const updateCountdownData = (data: CountdownData, threshold: number = 60) => {
    // 如果用户已手动退出，忽略后续更新
    if (userManuallyExited.value) {
      console.log('🚫 [FinalCountdownStore] 用户已主动退出，忽略倒计时更新')
      return
    }
    
    const wasVisible = isVisible.value
    const shouldShow = shouldShowFinalCountdown(data, threshold)
    
    if (shouldShow && !wasVisible) {
      // 需要显示但当前未显示
      showOverlay(data, threshold)
    } else if (shouldShow && wasVisible) {
      // 需要显示且当前已显示，更新数据
      countdownData.value = data
      
      // 更新状态
      if (data.status === 'finished') {
        isInEndState.value = true
        isInFinalCountdown.value = false
      } else {
        isInEndState.value = false
        isInFinalCountdown.value = true
      }
    } else if (!shouldShow && wasVisible) {
      // 不需要显示但当前已显示，隐藏
      hideOverlay()
    }
  }

  // 重置用户手动退出标志
  const resetManualExit = () => {
    userManuallyExited.value = false
  }

  return {
    // 状态
    isVisible,
    countdownData,
    beforeTime,
    isInEndState,
    isInFinalCountdown,
    userManuallyExited,
    
    // 方法
    showOverlay,
    hideOverlay,
    shouldShowFinalCountdown,
    updateCountdownData,
    resetManualExit
  }
})