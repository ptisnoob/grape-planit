<template>
    <header class="app-header" :class="{ 'visible': isVisible }">
        <div class="header-left">
            <ModeToggle v-if="route.path === '/'" @mode-changed="handleModeChange" />
        </div>
        <HeaderRight />
    </header>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router';
import ModeToggle from './ModeToggle.vue';
import HeaderRight from './HeaderRight.vue'

defineProps<{ isVisible: boolean }>();

const route = useRoute();
const emit = defineEmits(['mode-changed']);



const handleModeChange = (mode: string) => {
    emit('mode-changed', mode);
};

</script>

<style lang="scss" scoped>
.app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 10px;
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