<template>
  <div id="app" class="theme-transition">
    <AppHeader :is-visible="isHeaderVisible" />

    <main class="app-main" :class="{ 'header-visible': isHeaderVisible }">
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in" enter-active-class="animate__animated animate__fadeIn animate__faster"
          leave-active-class="animate__animated animate__fadeOut animate__faster">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import AppHeader from '@/components/AppHeader.vue';
import { useTheme } from '@/composables/useTheme.ts';

const { initTheme } = useTheme();
const isHeaderVisible = ref(false);

const showHeader = () => {
  isHeaderVisible.value = true;
};

const hideHeader = () => {
  isHeaderVisible.value = false;
};

onMounted(() => {
  initTheme();
  document.documentElement.addEventListener('mouseenter', showHeader);
  document.documentElement.addEventListener('mouseleave', hideHeader);
});

onUnmounted(() => {
  document.documentElement.removeEventListener('mouseenter', showHeader);
  document.documentElement.removeEventListener('mouseleave', hideHeader);
});
</script>

<style scoped>
#app {
  width: 100%;
  height: 100vh;
  background: var(--bg-primary);
  color: var(--text-primary);
  transition: all var(--transition-normal);
  display: flex;
  flex-direction: column;
}

.app-main {
  flex: 1;
  overflow-y: auto;
  height: 100vh;
  transition: padding-top 0.2s ease-in-out;
}

/* .app-main.header-visible { */
  /* padding-top: 10px; */
/* } */

/* 路由过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease-in-out;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* 自定义动画时长 */
:root {
  --animate-duration: 0.2s;
}
</style>