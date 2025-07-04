<template>
  <div class="config-tip" v-if="show">
    <div class="tip-icon">{{ icon }}</div>
    <div class="tip-content">
      <strong>{{ title }}</strong>
      <span>{{ description }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  show?: boolean
  icon?: string
  title: string
  description: string
}

withDefaults(defineProps<Props>(), {
  show: true,
  icon: '💡'
})
</script>

<style scoped>
.config-tip {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px 16px;
  background: var(--bg-secondary);
  border-radius: 8px;
  border-left: 4px solid var(--accent-color);
  margin-bottom: 16px;
  transition: all var(--transition-fast);
}

.config-tip:hover {
  transform: translateX(2px);
  box-shadow: 0 2px 8px var(--shadow);
}

.tip-icon {
  font-size: 18px;
  flex-shrink: 0;
  margin-top: 2px;
}

.tip-content {
  flex: 1;
  line-height: 1.5;
}

.tip-content strong {
  color: var(--text-primary);
  font-weight: 600;
  display: block;
  margin-bottom: 4px;
  font-size: 14px;
}

.tip-content span {
  color: var(--text-secondary);
  font-size: 13px;
}
</style>