<template>
  <div class="fulled">
    <AppHeader :is-visible="isHeaderVisible" />
    <DefaultTime></DefaultTime>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'
import DefaultTime from '@/components/DefaultTime.vue'
import AppHeader from '@/components/AppHeader.vue'
import { useTheme } from '@/composables/useTheme'
import { useAppStore } from '@/store/app'
import { WindowSettings } from '@/model/settings'

const isHeaderVisible = ref(false)
const { initTheme } = useTheme()
const router = useRouter()
const appStore = useAppStore()

const showHeader = () => {
  isHeaderVisible.value = true
}

const hideHeader = () => {
  isHeaderVisible.value = false
}

// 检查启动设置并决定显示内容
const checkStartupSettings = async () => {
  try {
    const settings = await invoke<WindowSettings>('load_window_settings_from_db')
    
    if (settings.default_startup === 'todo') {
      // 直接跳转到todo列表
      router.push('/list')
      return
    } else if (settings.default_startup === 'auto') {
      // 检查是否有最近的待办事项
      const recentDays = settings.recent_days || 5
      const todos = await invoke('get_recent_todos', { days: recentDays })
      
      if (Array.isArray(todos) && todos.length > 0) {
        // 有最近的待办事项，跳转到todo列表
        router.push('/list')
        return
      }
    }
    // 默认显示时间页面（default_startup === 'home' 或 auto模式下没有待办事项）
  } catch (error) {
    console.error('❌ [前端] 检查启动设置失败:', error)
    // 出错时默认显示时间页面
  }
}

// 加载窗口设置（仅加载主题色，主题由useTheme处理）
const loadWindowSettings = async () => {
  try {
    const settings = await invoke<WindowSettings>('load_window_settings_from_db')
    console.log('🔧 [前端] 主窗口加载设置:', settings)

    // 应用主题色
    if (settings.accent_color) {
      document.documentElement.style.setProperty('--accent-color', settings.accent_color)
      console.log('✅ [前端] 主题色已应用:', settings.accent_color)
    }
  } catch (error) {
    console.error('❌ [前端] 加载窗口设置失败:', error)
  }
}

onMounted(async () => {
  document.documentElement.addEventListener('mouseenter', showHeader)
  document.documentElement.addEventListener('mouseleave', hideHeader)

  // 初始化主题（从数据库读取）
  await initTheme()

  // 加载其他窗口设置
  await loadWindowSettings()
  
  // 只在第一次启动时检查启动设置
  if (!appStore.hasCheckedStartupSettings) {
    await checkStartupSettings()
    appStore.setStartupSettingsChecked(true)
  }
})

onUnmounted(() => {
  document.documentElement.removeEventListener('mouseenter', showHeader)
  document.documentElement.removeEventListener('mouseleave', hideHeader)
})
</script>

<style scoped></style>