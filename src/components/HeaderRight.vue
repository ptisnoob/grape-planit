<template>
    <div class="flex-r-c-c header-right">
        <Icon :name="currentNav.icon" :size="18" @click="toggleNav" />
        <div class="gap-line"></div>
        <Icon :name="getThemeIcon" :size="18" @click="toggleTheme" />
        <Icon name="settings" :size="18" @click="openSettings" />
        <Icon name="close" :size="18" @click="handleClose" />
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { windowApi } from '@/api/services'
import { useTheme } from '@/composables/useTheme'
import { useRouter } from 'vue-router';
import { useRoute } from 'vue-router';

const router = useRouter();
const route = useRoute();
const navs = [
    { path: '/', icon: 'ordered-list' },
    { path: '/list', icon: 'clock-circle' },
];


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

const getCurrentNavIndex = () => {
    const currentPath = route.path;
    const index = navs.findIndex(nav => nav.path === currentPath);
    return index >= 0 ? index : 0;
};
const currentNavIndex = ref(getCurrentNavIndex());
const currentNav = computed(() => navs[currentNavIndex.value]);
const toggleNav = () => {
    currentNavIndex.value = (currentNavIndex.value + 1) % navs.length;
    router.push(navs[currentNavIndex.value].path);
};

onMounted(() => {
    initTheme()
})
</script>

<style scoped lang="scss">
.header-right {
    padding: 10px;
    gap: 10px;

    .gap-line {
        width: 1px;
        height: 100%;
        background-color: #a5a3a3;
        margin: 0 3px;
    }
}
</style>