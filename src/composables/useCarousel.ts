import { ref, computed, type Ref } from 'vue'

/**
 * 轮播功能的 composable
 * 提供独立的轮播控制逻辑，避免与业务逻辑耦合
 */
export function useCarousel(itemsCount: Ref<number>, interval: number = 10000) {
  const currentIndex = ref(0)
  let carouselTimer: ReturnType<typeof setInterval> | null = null

  // 启动轮播
  const startCarousel = () => {
    // 如果只有一个或没有项目，不启动轮播
    if (itemsCount.value <= 1) {
      console.log(`轮播未启动：总项目数 ${itemsCount.value} <= 1`)
      return
    }

    // 如果已经在运行，先停止
    if (carouselTimer) {
      stopCarousel()
    }

    console.log(`启动轮播：总项目数 ${itemsCount.value}，当前索引 ${currentIndex.value}`)
    carouselTimer = setInterval(() => {
      const newIndex = (currentIndex.value + 1) % itemsCount.value
      console.log(`轮播切换：${currentIndex.value} -> ${newIndex}`)
      currentIndex.value = newIndex
    }, interval)
  }

  // 停止轮播
  const stopCarousel = () => {
    if (carouselTimer) {
      clearInterval(carouselTimer)
      carouselTimer = null
      console.log('轮播已停止')
    }
  }

  // 重启轮播（当项目数量变化时使用）
  const restartCarousel = () => {
    console.log(`重启轮播：项目数量 ${itemsCount.value}`)
    stopCarousel()
    // 重置索引到有效范围内
    if (currentIndex.value >= itemsCount.value) {
      currentIndex.value = 0
    }
    startCarousel()
  }

  // 手动设置索引
  const setIndex = (index: number) => {
    if (index >= 0 && index < itemsCount.value) {
      currentIndex.value = index
    }
  }

  // 跳转到下一个
  const next = () => {
    if (itemsCount.value > 1) {
      currentIndex.value = (currentIndex.value + 1) % itemsCount.value
    }
  }

  // 跳转到上一个
  const prev = () => {
    if (itemsCount.value > 1) {
      currentIndex.value = currentIndex.value === 0 
        ? itemsCount.value - 1 
        : currentIndex.value - 1
    }
  }

  // 检查轮播是否正在运行
  const isRunning = computed(() => carouselTimer !== null)

  return {
    currentIndex,
    isRunning,
    startCarousel,
    stopCarousel,
    restartCarousel,
    setIndex,
    next,
    prev
  }
}