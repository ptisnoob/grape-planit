<template>
    <header class="app-header" :class="{ 'visible': isVisible }">
        <div class="header-left">
            <Icon :name="currentNav.icon" :size="18" @click="toggleNav" />
            <ModeToggle v-if="route.path === '/'" @mode-changed="handleModeChange" />
        </div>
        <div class="header-right">
            <ThemeToggle />
            <button class="nav-link settings-btn">
                <Icon name="settings" :size="18" />
            </button>
            <button class="nav-link close-btn" @click="closeApp">
                <Icon name="close" :size="18" />
            </button>
        </div>
    </header>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import ModeToggle from './ModeToggle.vue';
import ThemeToggle from './ThemeToggle.vue';
// import { appWindow } from '@tauri-apps/api/window';

defineProps<{ isVisible: boolean }>();

const route = useRoute();
const router = useRouter();
const emit = defineEmits(['mode-changed']);

const navs = [
    { path: '/', icon: 'clock-circle' },
    { path: '/list', icon: 'list' },
    { path: '/calendar', icon: 'calendar' },
];

const currentNavIndex = ref(0);

const currentNav = computed(() => navs[currentNavIndex.value]);

const toggleNav = () => {
    currentNavIndex.value = (currentNavIndex.value + 1) % navs.length;
    router.push(navs[currentNavIndex.value].path);
};

const handleModeChange = (mode: string) => {
    emit('mode-changed', mode);
};

const closeApp = () => {
    // appWindow.close();
};
</script>

<style lang="scss" scoped>
.app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 16px;
    width: 100%;
    position: fixed;
    top: -60px;
    /* Start hidden */
    left: 0;
    z-index: 1000;
    // background-color: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
    // border-bottom: 1px solid var(--border-color);
    transition: top 0.2s ease-in-out;
    -webkit-app-region: drag;

    &.visible {
        top: 0;
    }
}


.header-left,
.header-right {
    display: flex;
    align-items: center;
    gap: 8px;
}

.nav-links {
    display: flex;
    gap: 8px;
}

.nav-link {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary);
    padding: 8px 12px;
    border-radius: 8px;
    transition: all 0.2s ease;
    text-decoration: none;
    white-space: nowrap;
    cursor: pointer;

    .icon {
        color: inherit;
    }



    &.active,
    &:hover {
        color: var(--accent-color);
        background-color: rgba(var(--accent-color-rgb), 0.1);
        transform: scale(1.1);
    }
}

.settings-btn {
    border: none;
    background: none;
    cursor: pointer;
}

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