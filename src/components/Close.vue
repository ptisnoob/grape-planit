<template>
    <button class="nav-link close-btn" @click="handleClose">
        <Icon name="close" :size="18" />
    </button>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'

const handleClose = async () => {
    try {
        await invoke('hide_to_tray')
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