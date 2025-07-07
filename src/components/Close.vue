<template>
    <button class="nav-link close-btn" @click="handleClose">
        <Icon name="close" :size="18" />
    </button>
</template>

<script setup lang="ts">
import { windowApi } from '@/api/services'

const handleClose = async () => {
    try {
        await windowApi.hideToTray()
    } catch (error) {
        console.error('Failed to hide to tray:', error)
    }
}
</script>

<style scoped lang="scss">
.close-btn {
    border: none;
    background: none;
    cursor: pointer;

    &:hover {
        color: #ff5f57;
        background-color: rgba(255, 95, 87, 0.1);
    }
}
</style>