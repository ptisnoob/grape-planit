<template>
  <main class="app-main theme-transition">
    <router-view v-slot="{ Component }">
      <transition name="fade" mode="out-in" enter-active-class="animate__animated animate__fadeIn animate__faster"
        leave-active-class="animate__animated animate__fadeOut animate__faster">
        <component :is="Component" />
      </transition>
    </router-view>
  </main>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useTheme } from '@/composables/useTheme.ts';

const { initTheme } = useTheme();

onMounted(() => {
  initTheme();
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