<template>
  <div class="mode-toggle" @click="toggleMode">
    {{ currentModeName }}
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useModeStore } from '@/store/mode';

const modeStore = useModeStore();

const modes = [
  { key: 'current', name: '默认' },
  { key: 'workEnd', name: '下班' }
];

const currentModeName = computed(() => {
  const currentMode = modes.find(mode => mode.key === modeStore.currentMode);
  return currentMode?.name || '当前';
});

const emit = defineEmits(['mode-changed']);

const toggleMode = async () => {
  const currentIndex = modes.findIndex(mode => mode.key === modeStore.currentMode);
  const nextIndex = (currentIndex + 1) % modes.length;
  const nextMode = modes[nextIndex].key;
  await modeStore.switchMode(nextMode);
  emit('mode-changed', nextMode);
};

onMounted(() => {
  modeStore.loadInitialMode();
});
</script>

<style lang="scss" scoped>
.mode-toggle {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
  background: none;
  border: none;
  // padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    background-color: var(--bg-secondary);
    color: var(--text-primary);
  }
}
</style>