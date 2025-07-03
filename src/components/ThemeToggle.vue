<template>
  <div class="theme-toggle">
    <button @click="toggleTheme" class="theme-btn">
      <Icon v-if="currentTheme !== 'auto'" :name="getThemeIcon" :size="18" />
      <span v-else class="auto-theme-text">{{ getThemeIcon }}</span>
    </button>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useTheme } from '@/composables/useTheme'
import Icon from '@/components/Icon.vue'

const { initTheme, toggleTheme, getThemeIcon, currentTheme } = useTheme()

onMounted(() => {
  initTheme()
})
</script>

<style lang="scss" scoped>
.theme-toggle {
  position: relative;
  z-index: 100;
}

.theme-btn {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
  background: none;
  border: none;
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    background-color: var(--bg-secondary);
    color: var(--text-primary);
  }
}

.auto-theme-text {
  font-size: 16px;
  font-weight: bold;
  color: var(--text-secondary);
  transition: color 0.2s ease;
}

.theme-options {
  position: absolute;
  top: 40px;
  left: 50%;
  transform: translateX(-50%);
  min-width: 120px;
  background: var(--bg-primary);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border: 1px solid var(--border-color);
  opacity: 0;
  transform: translateY(-10px) translateX(-50%);
  pointer-events: none;
  transition: all 0.3s ease;
  overflow: hidden;
  z-index: 101;

  &.visible {
    opacity: 1;
    transform: translateY(0) translateX(-50%);
    pointer-events: auto;
  }
}

.theme-option {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 12px 16px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 14px;
  text-align: left;
  cursor: pointer;
  transition: all var(--transition-normal);

  &:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  &.active {
    background: var(--accent-color);
    color: white;

    &:hover {
      background: var(--accent-color-hover);
    }
  }
}
</style>