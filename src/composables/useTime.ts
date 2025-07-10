import { ref, computed, onUnmounted } from 'vue'

/**
 * 时间处理相关的 composable
 * 提供统一的时间格式化、定时器管理等功能
 */
export function useTime() {
  // 基础时间数据
  const currentTime = ref('')
  const currentDate = ref('')
  const currentWeekday = ref('')
  
  // 定时器
  let timer: ReturnType<typeof setInterval> | null = null
  
  // 星期数组 - 用于TopTimeDisplay的简短格式
  const weekdaysShort = ['周日', '周一', '周二', '周三', '周四', '周五', '周六']
  
  // 星期数组 - 用于DefaultTime的完整格式
  const weekdaysFull = ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六']
  
  // 月份数组
  const months = ['1月', '2月', '3月', '4月', '5月', '6月', '7月', '8月', '9月', '10月', '11月', '12月']
  
  /**
   * 更新时间 - TopTimeDisplay格式
   * 格式: HH:MM:SS X月X日 周X
   */
  const updateTimeForTopDisplay = () => {
    const now = new Date()
    
    // 格式化时间 HH:MM:SS
    const hours = now.getHours().toString().padStart(2, '0')
    const minutes = now.getMinutes().toString().padStart(2, '0')
    const seconds = now.getSeconds().toString().padStart(2, '0')
    currentTime.value = `${hours}:${minutes}:${seconds}`
    
    // 格式化日期 X月X日
    const month = months[now.getMonth()]
    const day = now.getDate()
    currentDate.value = `${month}${day}日`
    
    // 格式化星期
    currentWeekday.value = weekdaysShort[now.getDay()]
  }
  
  /**
   * 更新时间 - DefaultTime格式
   * 格式: HH:MM:SS YYYY-MM-DD 星期X
   */
  const updateTimeForDefaultDisplay = () => {
    const now = new Date()
    
    // 格式化时间 HH:MM:SS
    const hours = now.getHours().toString().padStart(2, '0')
    const minutes = now.getMinutes().toString().padStart(2, '0')
    const seconds = now.getSeconds().toString().padStart(2, '0')
    currentTime.value = `${hours}:${minutes}:${seconds}`
    
    // 格式化日期 YYYY-MM-DD
    const year = now.getFullYear()
    const month = (now.getMonth() + 1).toString().padStart(2, '0')
    const day = now.getDate().toString().padStart(2, '0')
    currentDate.value = `${year}-${month}-${day}`
    
    // 格式化星期
    currentWeekday.value = weekdaysFull[now.getDay()]
  }
  
  /**
   * 格式化倒计时时间
   * @param totalSeconds 总秒数
   * @returns 格式化后的时间字符串
   */
  const formatCountdownTime = (totalSeconds: number): string => {
    const hours = Math.floor(totalSeconds / 3600)
    const minutes = Math.floor((totalSeconds % 3600) / 60)
    const seconds = totalSeconds % 60
    
    let timeText = ''
    if (hours > 0) {
      timeText = `${hours}小时${minutes}分${seconds}秒`
    } else if (minutes > 0) {
      timeText = `${minutes}分${seconds}秒`
    } else {
      timeText = `${seconds}秒`
    }
    
    return timeText
  }
  
  /**
   * 格式化倒计时为 HH:MM:SS 格式
   * @param totalSeconds 总秒数
   * @returns HH:MM:SS 格式的字符串
   */
  const formatCountdownToHMS = (totalSeconds: number): string => {
    const hours = Math.floor(totalSeconds / 3600)
    const minutes = Math.floor((totalSeconds % 3600) / 60)
    const seconds = totalSeconds % 60
    
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
  }
  
  /**
   * 启动定时器
   * @param updateFunction 更新函数
   * @param interval 间隔时间（毫秒），默认1000ms
   */
  const startTimer = (updateFunction: () => void, interval: number = 1000) => {
    // 先执行一次
    updateFunction()
    // 启动定时器
    timer = setInterval(updateFunction, interval)
  }
  
  /**
   * 停止定时器
   */
  const stopTimer = () => {
    if (timer) {
      clearInterval(timer)
      timer = null
    }
  }
  
  /**
   * 计算距离下个节日的天数
   * @returns 节日信息对象
   */
  const calculateNextHoliday = () => {
    const now = new Date()
    const currentYear = now.getFullYear()
    
    // 节日列表
    const holidays = [
      { name: '元旦', date: new Date(currentYear + 1, 0, 1) },
      { name: '春节', date: new Date(currentYear + 1, 1, 10) }, // 假设日期
      { name: '清明节', date: new Date(currentYear + 1, 3, 5) },
      { name: '劳动节', date: new Date(currentYear + 1, 4, 1) },
      { name: '端午节', date: new Date(currentYear + 1, 5, 14) },
      { name: '中秋节', date: new Date(currentYear + 1, 8, 17) },
      { name: '国庆节', date: new Date(currentYear, 9, 1) }
    ]
    
    // 找到下一个节日
    const nextHolidayData = holidays.find(holiday => holiday.date > now)
    if (nextHolidayData) {
      const diffTime = nextHolidayData.date.getTime() - now.getTime()
      const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
      return {
        name: nextHolidayData.name,
        days: diffDays
      }
    }
    
    return {
      name: '暂无',
      days: 0
    }
  }
  
  // TopTimeDisplay 格式的组合时间文本
  const currentTimeTextForTop = computed(() => {
    return `${currentTime.value} ${currentDate.value} ${currentWeekday.value}`
  })
  
  // 组件卸载时自动清理定时器
  onUnmounted(() => {
    stopTimer()
  })
  
  return {
    // 响应式数据
    currentTime,
    currentDate,
    currentWeekday,
    
    // 计算属性
    currentTimeTextForTop,
    
    // 常量
    weekdaysShort,
    weekdaysFull,
    months,
    
    // 方法
    updateTimeForTopDisplay,
    updateTimeForDefaultDisplay,
    formatCountdownTime,
    formatCountdownToHMS,
    startTimer,
    stopTimer,
    calculateNextHoliday
  }
}

/**
 * 倒计时相关的 composable
 * 提供倒计时监听、格式化等功能
 */
export function useCountdown() {
  const { formatCountdownTime } = useTime()
  
  /**
   * 格式化倒计时文本 - 用于TopTimeDisplay
   * @param data 倒计时数据
   * @returns 格式化后的文本
   */
  const formatCountdownText = (data: any): string => {
    if (data.status === 'finished') {
      return data.mode === 'workEnd' ? '已到下班时间！' : `${data.target_info}已到时间！`
    }
    
    if (data.status === 'running' && data.timestamp > 0) {
      const timeText = formatCountdownTime(data.timestamp)
      return `离${data.target_info}还剩${timeText}`
    }
    
    return `${data.target_info}未开始`
  }
  
  return {
    formatCountdownText
  }
}