<template>
  <div class="top-time-display">
    <div class="time-carousel" :style="{ transform: `translateY(-${currentIndex * 100}%)` }">
      <!-- 当前时间 -->
      <div class="time-item current-time">
        <span class="time-text">{{ currentTimeText }}</span>
      </div>
      
      <!-- 倒计时列表 -->
      <div v-for="countdown in countdowns" :key="countdown.id" class="time-item countdown-item">
        <span class="countdown-text">{{ countdown.text }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useRouter } from 'vue-router'
import { CountdownData } from '@/model/countdown'
import { useModeStore } from '@/store/mode'

interface CountdownItem {
  id: string
  text: string
  data: CountdownData
}

const router = useRouter()
const modeStore = useModeStore()

const currentTime = ref('')
const currentDate = ref('')
const currentWeekday = ref('')
const countdowns = ref<CountdownItem[]>([])
const currentIndex = ref(0)
const totalItems = computed(() => 1 + countdowns.value.length) // 1个当前时间 + 倒计时数量

let timer: ReturnType<typeof setInterval> | null = null
let carouselTimer: ReturnType<typeof setInterval> | null = null
let unlistenCountdown: (() => void) | null = null

const weekdays = ['周日', '周一', '周二', '周三', '周四', '周五', '周六']
const months = ['1月', '2月', '3月', '4月', '5月', '6月', '7月', '8月', '9月', '10月', '11月', '12月']

// 当前时间显示文本
const currentTimeText = computed(() => {
  return `${currentTime.value} ${currentDate.value} ${currentWeekday.value}`
})

// 更新时间
const updateTime = () => {
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
  currentWeekday.value = weekdays[now.getDay()]
}

// 轮播控制
const startCarousel = () => {
  if (totalItems.value <= 1) return
  
  carouselTimer = setInterval(() => {
    currentIndex.value = (currentIndex.value + 1) % totalItems.value
  }, 3000) // 每3秒切换一次
}

const stopCarousel = () => {
  if (carouselTimer) {
    clearInterval(carouselTimer)
    carouselTimer = null
  }
}

// 格式化倒计时文本
const formatCountdownText = (data: CountdownData): string => {
  if (data.status === 'finished') {
    return data.mode === 'workEnd' ? '已到下班时间！' : `${data.target_info}已到时间！`
  }
  
  if (data.status === 'running' && data.timestamp > 0) {
    const totalSeconds = data.timestamp
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
    
    return `距离${data.target_info}还剩${timeText}`
  }
  
  return `${data.target_info}未开始`
}

// 设置倒计时监听
const setupCountdownListener = async () => {
  try {
    unlistenCountdown = await listen('countdown-update', (event) => {
      const newData = event.payload as CountdownData
      
      // 查找是否已存在该倒计时
      const existingIndex = countdowns.value.findIndex(item => item.id === newData.mode)
      
      if (existingIndex >= 0) {
        // 更新现有倒计时
        const oldData = countdowns.value[existingIndex].data
        countdowns.value[existingIndex] = {
          id: newData.mode,
          text: formatCountdownText(newData),
          data: newData
        }
        
        // 检查是否刚进入最终倒计时阶段
        const wasInFinalCountdown = oldData.status === 'running' && oldData.timestamp <= 60
        const isNowInFinalCountdown = newData.status === 'running' && newData.timestamp <= 60
        
        if (!wasInFinalCountdown && isNowInFinalCountdown) {
          // 自动切换到对应的倒计时页面
          console.log(`倒计时${newData.mode}进入最终阶段，自动切换页面`)
          if (newData.mode === 'workEnd') {
            modeStore.switchMode('workEnd')
            router.push('/')
          } else if (newData.mode === 'custom') {
            modeStore.switchMode('custom')
            router.push('/')
          }
        }
      } else {
        // 添加新的倒计时
        countdowns.value.push({
          id: newData.mode,
          text: formatCountdownText(newData),
          data: newData
        })
      }
      
      // 重新启动轮播
      stopCarousel()
      startCarousel()
    })
  } catch (error) {
    console.error('Failed to setup countdown listener:', error)
  }
}

// 初始化倒计时数据
const initCountdowns = async () => {
  try {
    // 启动倒计时服务
    await invoke('start_countdown_timer')
  } catch (error) {
    console.error('Failed to start countdown timer:', error)
  }
}

onMounted(async () => {
  updateTime()
  timer = setInterval(updateTime, 1000)
  
  await setupCountdownListener()
  await initCountdowns()
  
  startCarousel()
})

onUnmounted(() => {
  if (timer) {
    clearInterval(timer)
  }
  stopCarousel()
  if (unlistenCountdown) {
    unlistenCountdown()
  }
})
</script>

<style scoped>
.top-time-display {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 40px;
  background: var(--bg-primary);
  color: var(--text-primary);
  z-index: 50;
  overflow: hidden;
  box-shadow: 0 1px 4px var(--shadow);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid var(--border-color);
  transition: all var(--transition-normal);
}

.time-carousel {
  height: 100%;
  transition: transform 0.5s ease-in-out;
  display: flex;
  flex-direction: column;
}

.time-item {
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  padding: 0 20px;
}

.time-text,
.countdown-text {
  font-size: 14px;
  font-weight: 500;
  text-align: center;
  white-space: nowrap;
}

.current-time {
  background: transparent;
  color: var(--text-primary);
}

.countdown-item {
  background: var(--bg-secondary);
  color: var(--accent-color);
  font-weight: 600;
  border-left: 3px solid var(--accent-color);
}

/* 动画效果 */
@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.time-item {
  animation: slideIn 0.3s ease-out;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .time-text,
  .countdown-text {
    font-size: 12px;
  }
  
  .time-item {
    padding: 0 15px;
  }
}
</style>