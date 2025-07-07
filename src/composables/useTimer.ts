import { ref, onUnmounted } from 'vue'

/**
 * 定时器管理 composable
 * 提供统一的定时器创建、管理和清理功能
 */
export function useTimer() {
  // 存储所有活跃的定时器
  const timers = ref<Map<string, NodeJS.Timeout>>(new Map())

  /**
   * 创建一个命名的定时器
   * @param name 定时器名称，用于标识和管理
   * @param callback 回调函数
   * @param delay 延迟时间（毫秒）
   * @param autoCleanup 是否在组件卸载时自动清理，默认true
   * @returns 定时器ID
   */
  const createTimer = (
    name: string,
    callback: () => void,
    delay: number,
    autoCleanup: boolean = true
  ): NodeJS.Timeout => {
    // 如果已存在同名定时器，先清除
    if (timers.value.has(name)) {
      clearTimer(name)
    }

    const timerId = setTimeout(() => {
      callback()
      // 执行完成后从管理器中移除
      timers.value.delete(name)
    }, delay)

    // 如果需要自动清理，则添加到管理器中
    if (autoCleanup) {
      timers.value.set(name, timerId)
    }

    return timerId
  }

  /**
   * 清除指定名称的定时器
   * @param name 定时器名称
   */
  const clearTimer = (name: string): void => {
    const timer = timers.value.get(name)
    if (timer) {
      clearTimeout(timer)
      timers.value.delete(name)
    }
  }

  /**
   * 清除所有定时器
   */
  const clearAllTimers = (): void => {
    timers.value.forEach((timer) => {
      clearTimeout(timer)
    })
    timers.value.clear()
  }

  /**
   * 检查指定名称的定时器是否存在
   * @param name 定时器名称
   * @returns 是否存在
   */
  const hasTimer = (name: string): boolean => {
    return timers.value.has(name)
  }

  /**
   * 获取当前活跃的定时器数量
   * @returns 定时器数量
   */
  const getTimerCount = (): number => {
    return timers.value.size
  }

  /**
   * 获取所有活跃的定时器名称
   * @returns 定时器名称数组
   */
  const getTimerNames = (): string[] => {
    return Array.from(timers.value.keys())
  }

  /**
   * 创建一个延迟执行的Promise
   * @param delay 延迟时间（毫秒）
   * @returns Promise
   */
  const delay = (delay: number): Promise<void> => {
    return new Promise((resolve) => {
      setTimeout(resolve, delay)
    })
  }

  /**
   * 创建一个可取消的延迟Promise
   * @param name 定时器名称
   * @param delay 延迟时间（毫秒）
   * @returns Promise和取消函数
   */
  const createCancelableDelay = (name: string, delay: number) => {
    let timeoutId: NodeJS.Timeout
    
    const promise = new Promise<void>((resolve) => {
      timeoutId = setTimeout(() => {
        timers.value.delete(name)
        resolve()
      }, delay)
      
      timers.value.set(name, timeoutId)
    })

    const cancel = () => {
      if (timers.value.has(name)) {
        clearTimer(name)
      }
    }

    return { promise, cancel }
  }

  // 组件卸载时自动清理所有定时器
  onUnmounted(() => {
    clearAllTimers()
  })

  return {
    // 定时器管理
    createTimer,
    clearTimer,
    clearAllTimers,
    hasTimer,
    getTimerCount,
    getTimerNames,
    
    // 延迟工具
    delay,
    createCancelableDelay
  }
}

/**
 * 专门用于长按操作的定时器管理
 */
export function useLongPressTimer() {
  const { createTimer, clearTimer, hasTimer } = useTimer()

  /**
   * 开始长按定时器
   * @param callback 长按触发的回调
   * @param delay 长按延迟时间，默认300ms
   * @returns 定时器ID
   */
  const startLongPress = (callback: () => void, delay: number = 300): NodeJS.Timeout => {
    return createTimer('longPress', callback, delay)
  }

  /**
   * 取消长按定时器
   */
  const cancelLongPress = (): void => {
    clearTimer('longPress')
  }

  /**
   * 检查是否有活跃的长按定时器
   */
  const isLongPressing = (): boolean => {
    return hasTimer('longPress')
  }

  return {
    startLongPress,
    cancelLongPress,
    isLongPressing
  }
}

/**
 * 专门用于录制操作的定时器管理
 */
export function useRecordingTimer() {
  const { createTimer, clearTimer, hasTimer } = useTimer()

  /**
   * 开始录制定时器
   * @param callback 录制超时的回调
   * @param timeout 录制超时时间，默认5000ms
   * @returns 定时器ID
   */
  const startRecording = (callback: () => void, timeout: number = 5000): NodeJS.Timeout => {
    return createTimer('recording', callback, timeout)
  }

  /**
   * 停止录制定时器
   */
  const stopRecording = (): void => {
    clearTimer('recording')
  }

  /**
   * 检查是否正在录制
   */
  const isRecording = (): boolean => {
    return hasTimer('recording')
  }

  return {
    startRecording,
    stopRecording,
    isRecording
  }
}

/**
 * 专门用于结束状态保持的定时器管理
 */
export function useEndStateTimer() {
  const { createTimer, clearTimer, hasTimer } = useTimer()

  /**
   * 开始结束状态保持定时器
   * @param callback 结束状态超时的回调
   * @param keepMinutes 保持时间（分钟），默认5分钟
   * @returns 定时器ID
   */
  const startEndStateTimer = (callback: () => void, keepMinutes: number = 5): NodeJS.Timeout => {
    const keepMilliseconds = keepMinutes * 60 * 1000
    return createTimer('endState', callback, keepMilliseconds)
  }

  /**
   * 清除结束状态定时器
   */
  const clearEndStateTimer = (): void => {
    clearTimer('endState')
  }

  /**
   * 检查是否有活跃的结束状态定时器
   */
  const hasEndStateTimer = (): boolean => {
    return hasTimer('endState')
  }

  return {
    startEndStateTimer,
    clearEndStateTimer,
    hasEndStateTimer
  }
}

/**
 * 专门用于UI反馈的定时器管理
 */
export function useUIFeedbackTimer() {
  const { createTimer, clearTimer } = useTimer()

  /**
   * 创建一个用于清除UI反馈的定时器
   * @param callback 清除反馈的回调
   * @param delay 延迟时间，默认3000ms
   * @param name 定时器名称，默认'uiFeedback'
   */
  const createFeedbackTimer = (
    callback: () => void,
    delay: number = 3000,
    name: string = 'uiFeedback'
  ): void => {
    createTimer(name, callback, delay)
  }

  /**
   * 清除UI反馈定时器
   * @param name 定时器名称，默认'uiFeedback'
   */
  const clearFeedbackTimer = (name: string = 'uiFeedback'): void => {
    clearTimer(name)
  }

  return {
    createFeedbackTimer,
    clearFeedbackTimer
  }
}

/**
 * 专门用于焦点管理的定时器
 */
export function useFocusTimer() {
  const { createTimer } = useTimer()

  /**
   * 延迟聚焦到指定元素
   * @param selector 元素选择器
   * @param delay 延迟时间，默认200ms
   * @param shouldSelect 是否选中文本，默认true
   */
  const delayedFocus = (
    selector: string,
    delay: number = 200,
    shouldSelect: boolean = true
  ): void => {
    createTimer(`focus_${Date.now()}`, () => {
      const element = document.querySelector(selector) as HTMLInputElement | HTMLTextAreaElement
      if (element) {
        // 检查元素是否可见
        const isVisible = element.offsetParent !== null && 
                         getComputedStyle(element).display !== 'none' &&
                         getComputedStyle(element).visibility !== 'hidden'
        
        if (isVisible) {
          element.focus()
          if (shouldSelect && (element.select || element.setSelectionRange)) {
            if (element.select) {
              element.select()
            } else if (element.setSelectionRange) {
              element.setSelectionRange(0, element.value.length)
            }
          }
          console.log(`✅ 聚焦到元素: ${selector}`)
        } else {
          console.log(`⚠️ 元素不可见: ${selector}`)
        }
      } else {
        console.log(`❌ 未找到元素: ${selector}`)
      }
    }, delay, false) // 不自动清理，因为是一次性操作
  }

  return {
    delayedFocus
  }
}