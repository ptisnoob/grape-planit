<template>
    <button class="nav-link settings-btn" @click="openSettings">
        <Icon name="settings" :size="18" />
    </button>
</template>

<script setup lang="ts">
import { windowApi } from '@/api/services';

const openSettings = async () => {
    try {
        await windowApi.showSettings();
    } catch (error) {
        console.error('Failed to open settings window:', error);
    }
};

</script>

<style scoped lang="scss">
.settings-btn {
    border: none;
    background: none;
    cursor: pointer;
}
</style>