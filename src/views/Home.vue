<template>
  <div class="fulled">
    <AppHeader :is-visible="isHeaderVisible" />
    <DefaultTime></DefaultTime>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import DefaultTime from '@/components/DefaultTime.vue'
import AppHeader from '@/components/AppHeader.vue'
import { useTheme } from '@/composables/useTheme'
import { WindowSettings } from '@/model/settings'

const isHeaderVisible = ref(false)
const { initTheme } = useTheme()

const showHeader = () => {
  isHeaderVisible.value = true
}

const hideHeader = () => {
  isHeaderVisible.value = false
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
})

onUnmounted(() => {
  document.documentElement.removeEventListener('mouseenter', showHeader)
  document.documentElement.removeEventListener('mouseleave', hideHeader)
})
</script>

<style scoped></style>