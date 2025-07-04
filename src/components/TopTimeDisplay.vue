<template>
  <div class="top-time-display flex-r">
    <div class="time-carousel flex-1" :style="{ transform: `translateY(-${currentIndex * 100}%)` }" @click="navToTime">
      <!-- 当前时间 -->
      <div class="time-item current-time">
        <span class="time-text">{{ currentTimeText }}</span>
      </div>

      <!-- 倒计时列表 -->
      <div v-for="countdown in countdowns" :key="countdown.id" class="time-item countdown-item">
        <span class="countdown-text">{{ countdown.text }}</span>
      </div>
    </div>
    <SettingsBtn />
    <ThemeToggle />
    <Close />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useRouter } from 'vue-router'
import { CountdownData } from '@/model/countdown'
import { useModeStore } from '@/store/mode'
import { useTime, useCountdown } from '@/composables/useTime'
import { useCarousel } from '@/composables/useCarousel'
import Close from './Close.vue'
import ThemeToggle from './ThemeToggle.vue';
import SettingsBtn from './SettingsBtn.vue'
interface CountdownItem {
  id: string
  text: string
  data: CountdownData
}

const router = useRouter()
const modeStore = useModeStore()

const countdowns = ref<CountdownItem[]>([])
const totalItems = computed(() => 1 + countdowns.value.length) // 1个当前时间 + 倒计时数量

let unlistenCountdown: (() => void) | null = null

// 使用时间相关的 composable
const { currentTimeTextForTop, startTimer, stopTimer, updateTimeForTopDisplay } = useTime()
const { formatCountdownText } = useCountdown()

// 使用轮播 composable
const { currentIndex, startCarousel, stopCarousel, restartCarousel } = useCarousel(totalItems, 10000)

// 为了保持兼容性，创建一个别名
const currentTimeText = currentTimeTextForTop



// 设置倒计时监听
const setupCountdownListener = async () => {
  try {
    unlistenCountdown = await listen('countdown-update', (event) => {
      const newData = event.payload as CountdownData
      const previousCount = countdowns.value.length

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

      // 只有在倒计时数量发生变化时才重新启动轮播
      const currentCount = countdowns.value.length
      if (currentCount !== previousCount) {
        console.log(`倒计时数量变化：${previousCount} -> ${currentCount}，重新启动轮播`)
        restartCarousel()
      }
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

const navToTime = () => {
  router.push('/')
}

onMounted(async () => {
  startTimer(updateTimeForTopDisplay)

  await setupCountdownListener()
  await initCountdowns()

  startCarousel()
})

onUnmounted(() => {
  stopTimer()
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
  background: transparent;
  color: var(--text-primary);
  z-index: 50;
  overflow: hidden;
  box-shadow: 0 1px 4px var(--shadow);
  /* backdrop-filter: blur(20px); */
  /* border-bottom: 1px solid var(--border-color); */
  transition: all var(--transition-normal);
}

.time-carousel {
  height: 100%;
  transition: transform 0.5s ease-in-out;
  display: flex;
  flex-direction: column;
  cursor: pointer;
}

.time-item {
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: flex-start;
  flex-shrink: 0;
  padding: 0 10px;
}

.time-text,
.countdown-text {
  font-size: 14px;
  font-weight: 500;
  text-align: center;
  white-space: nowrap;
  user-select: none;
}

.current-time {
  background: transparent;
  color: var(--text-primary);
}

.countdown-item {
  /* background: var(--bg-secondary); */
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

</style>