<template>
    <header class="app-header" :class="{ 'visible': isVisible }">
        <div class="header-left">
            <button v-if="route.path === '/add'" class="nav-link back-btn" @click="goBack">
                返回
            </button>
            <Icon v-else :name="currentNav.icon" :size="18" @click="toggleNav" />
            <ModeToggle v-if="route.path === '/'" @mode-changed="handleModeChange" />
            <button v-if="route.path === '/list'" class="nav-link settings-btn" @click="toAddTodo">
                <Icon name="plus" :size="18" />
            </button>
        </div>
        <div class="header-right">
            <ThemeToggle />
            <SettingsBtn />
            <Close />
        </div>
    </header>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import ModeToggle from './ModeToggle.vue';
import ThemeToggle from './ThemeToggle.vue';
import SettingsBtn from "./SettingsBtn.vue"
import Close from './Close.vue';

defineProps<{ isVisible: boolean }>();

const route = useRoute();
const router = useRouter();
const emit = defineEmits(['mode-changed']);

const navs = [
    { path: '/', icon: 'clock-circle' },
    { path: '/list', icon: 'list' },
    // { path: '/calendar', icon: 'calendar' },
];

const getCurrentNavIndex = () => {
    const currentPath = route.path;
    const index = navs.findIndex(nav => nav.path === currentPath);
    return index >= 0 ? index : 0;
};

const currentNavIndex = ref(getCurrentNavIndex());

const currentNav = computed(() => navs[currentNavIndex.value]);
const toAddTodo = () => {
    router.push('/add');
}

const goBack = () => {
    router.go(-1);
}

const toggleNav = () => {
    currentNavIndex.value = (currentNavIndex.value + 1) % navs.length;
    router.push(navs[currentNavIndex.value].path);
};

const handleModeChange = (mode: string) => {
    emit('mode-changed', mode);
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


.back-btn {
    border: none;
    background: none;
    cursor: pointer;
    font-size: 12px;
    padding: 4px 8px;

    &:hover {
        color: var(--accent-color);
        background-color: rgba(var(--accent-color-rgb), 0.1);
    }
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