<template>
  <div class="theme-toggle">
    <button @click="toggleTheme" class="theme-btn">
      <Icon :name="themeIcon" :size="18" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useTheme, type ThemeType } from '@/composables/useTheme'

const { currentTheme, setTheme, initTheme } = useTheme()

const themes = [
  { value: 'light' as ThemeType, label: '浅色' },
  { value: 'dark' as ThemeType, label: '深色' }
]

const themeIcon = computed(() => {
  return currentTheme.value === 'dark' ? 'moon' : 'sun';
});

const toggleTheme = () => {
  const currentIndex = themes.findIndex(t => t.value === currentTheme.value)
  const nextIndex = (currentIndex + 1) % themes.length
  setTheme(themes[nextIndex].value)
}

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