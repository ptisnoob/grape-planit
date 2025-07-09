<template>
  <div class="top-time-display flex-r">
    <div class="time-carousel flex-1" :style="{ transform: shouldShowCarousel ? `translateY(-${currentIndex * 100}%)` : 'translateY(0)' }" @click="navToTime">
      <!-- 当前时间 -->
      <div class="time-item current-time">
        <span class="time-text">{{ currentTimeText }}</span>
      </div>

      <!-- 倒计时列表 -->
      <div v-for="countdown in countdowns" :key="countdown.id" class="time-item countdown-item">
        <span class="countdown-text">{{ countdown.text }}</span>
      </div>

      <!-- 当开启下班倒计时但没有倒计时数据时显示占位符 -->
      <div v-if="shouldShowCarousel && countdowns.length === 0" class="time-item placeholder-item">
        <span class="placeholder-text">等待下班倒计时...</span>
      </div>
    </div>
    <SettingsBtn />
    <ThemeToggle />
    <Close />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { useRouter, useRoute } from 'vue-router'
import { CountdownData, CountdownConfig } from '@/model/countdown'

import { useTime, useCountdown } from '@/composables/useTime'
import { useCarousel } from '@/composables/useCarousel'
import { databaseApi } from '@/api/services'
import Close from './Close.vue'
import ThemeToggle from './ThemeToggle.vue';
import SettingsBtn from './SettingsBtn.vue'
interface CountdownItem {
  id: string
  text: string
  data: CountdownData
}

const router = useRouter()
const route = useRoute()

const countdowns = ref<CountdownItem[]>([])
const totalItems = computed(() => {
  // 1个当前时间 + 倒计时数量 + (开启轮播但没有倒计时时的占位符)
  const baseItems = 1 + countdowns.value.length
  const hasPlaceholder = shouldShowCarousel.value && countdowns.value.length === 0
  const total = hasPlaceholder ? baseItems + 1 : baseItems
  return total
})
const countdownConfig = ref<CountdownConfig | null>(null)
const shouldShowCarousel = computed(() => {
  return countdownConfig.value?.enableWorkEndCountdown === true
})

// 记录用户最后手动操作的时间，用于避免自动跳转干扰用户操作
const lastUserActionTime = ref(0)

let unlistenCountdown: (() => void) | null = null
let unlistenConfigUpdate: (() => void) | null = null

// 使用时间相关的 composable
const { currentTimeTextForTop, startTimer, stopTimer, updateTimeForTopDisplay } = useTime()
const { formatCountdownText } = useCountdown()

// 使用轮播 composable
const { currentIndex, startCarousel, stopCarousel, restartCarousel } = useCarousel(totalItems, 10000)

// 为了保持兼容性，创建一个别名
const currentTimeText = currentTimeTextForTop



// 设置配置更新监听
const setupConfigListener = async () => {
  try {
    unlistenConfigUpdate = await listen('config-updated', async () => {
      await loadCountdownConfig()
    })
  } catch (error) {
    console.error('Failed to setup config listener:', error)
  }
}

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
        countdowns.value[existingIndex] = {
          id: newData.mode,
          text: formatCountdownText(newData),
          data: newData
        }

      } else {
        // 添加新的倒计时
        countdowns.value.push({
          id: newData.mode,
          text: formatCountdownText(newData),
          data: newData
        })
      }

      // 只有在开启下班倒计时且倒计时数量发生变化时才重新启动轮播
      const currentCount = countdowns.value.length
      if (currentCount !== previousCount && shouldShowCarousel.value) {
        restartCarousel()
      }
    })
  } catch (error) {
    console.error('Failed to setup countdown listener:', error)
  }
}

// 加载倒计时配置
const loadCountdownConfig = async () => {
  try {
    countdownConfig.value = await databaseApi.countdown.load()
  } catch (error) {
    console.error('Failed to load countdown config:', error)
  }
}

// 初始化倒计时数据
const initCountdowns = async () => {
  try {
    // 先加载配置
    await loadCountdownConfig()
    // 启动倒计时服务
    await databaseApi.countdown.startTimer()
  } catch (error) {
    console.error('Failed to start countdown timer:', error)
  }
}

const navToTime = () => {
  router.push('/')
}

// 监听路由变化，记录用户手动操作时间
watch(() => route.path, (newPath, oldPath) => {
  if (newPath !== oldPath) {
    lastUserActionTime.value = Date.now()
  }
})

// 监听轮播显示状态变化
watch(shouldShowCarousel, (newValue, oldValue) => {
  if (newValue !== oldValue) {
    if (newValue) {
      startCarousel()
    } else {
      stopCarousel()
    }
  }
})

onMounted(async () => {
  startTimer(updateTimeForTopDisplay)

  await setupConfigListener()
  await setupCountdownListener()
  await initCountdowns()

  // 只有在开启下班倒计时时才启动轮播
  if (shouldShowCarousel.value) {
    startCarousel()
  }
  
  // 初始化用户操作时间
  lastUserActionTime.value = Date.now()
})

onUnmounted(() => {
  stopTimer()
  stopCarousel()
  if (unlistenCountdown) {
    unlistenCountdown()
  }
  if (unlistenConfigUpdate) {
    unlistenConfigUpdate()
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

.placeholder-item {
  color: var(--text-secondary);
  font-weight: 400;
  border-left: 3px solid var(--text-secondary);
  opacity: 0.7;
}

.placeholder-text {
  font-size: 14px;
  font-weight: 400;
  text-align: center;
  white-space: nowrap;
  user-select: none;
  font-style: italic;
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