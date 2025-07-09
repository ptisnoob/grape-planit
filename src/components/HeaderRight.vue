<template>
    <!-- <button class="nav-link close-btn" @click="handleClose"> -->
    <view class="flex-r-c-c header-right">
        <Icon :name="getThemeIcon" :size="18" @click="toggleTheme" />
        <Icon name="settings" :size="18" @click="openSettings" />
        <Icon name="close" :size="18" @click="handleClose" />
    </view>
    <!-- </button> -->
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { windowApi } from '@/api/services'
import { useTheme } from '@/composables/useTheme'
const { initTheme, toggleTheme, getThemeIcon } = useTheme()
const handleClose = async () => {
    try {
        await windowApi.hideToTray()
    } catch (error) {
        console.error('Failed to hide to tray:', error)
    }
}
const openSettings = async () => {
    try {
        await windowApi.showSettings();
    } catch (error) {
        console.error('Failed to open settings window:', error);
    }
};

onMounted(() => {
    initTheme()
})
</script>

<style scoped lang="scss">
.header-right{
    padding: 0 10px;
    gap: 10px;
}
</style>