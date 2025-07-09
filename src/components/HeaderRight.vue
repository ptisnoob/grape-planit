<template>
    <div class="flex-r-c-c header-right">
        <div v-if="currentNavIndex === 1" class="link-text activate-text" @click="toggleMode">{{ currentMode.label }}</div>
        <div class="link-text" @click="toggleNav">{{ currentNav.icon }}</div>
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

const { initTheme, toggleTheme, getThemeIcon } = useTheme()
const router = useRouter();
const route = useRoute();
const emit = defineEmits(['changeMode'])

const navs = [
    { path: '/', icon: '时间' },
    { path: '/list', icon: 'TODO' },
];
const modes = [
    { label: '列表', value: 'list' },
    { label: '分类', value: 'category' }
    // { label: '日历', value: 'calendar' },
]
const currentModeIndex = ref(0)
const currentMode = computed(() => modes[currentModeIndex.value])

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

const toggleMode = () => {
    currentModeIndex.value = (currentModeIndex.value + 1) % modes.length;
    emit('changeMode', currentMode.value.value)
}

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

    .link-text {
        font-size: 14px;
        cursor: pointer;
        user-select: none;
    }

    .activate-text{
        color: var(--accent-color);
    }
}
</style>