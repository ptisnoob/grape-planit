<template>
  <i :style="style" class="icon" v-html="iconContent"></i>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';

interface Props {
  name: string;
  size?: number | string;
  color?: string;
}

const props = withDefaults(defineProps<Props>(), {
  size: 24,
  color: 'currentColor'
});

const iconContent = ref('');

const style = computed(() => ({
  fontSize: typeof props.size === 'number' ? `${props.size}px` : props.size,
  color: props.color
}));

// 自动化图标加载 - 使用动态import但确保打包兼容性
const loadIcon = async () => {
  try {
    // 使用动态import，但通过字符串模板确保Vite能够静态分析
    const iconModule = await import(`../assets/icons/${props.name}.svg?raw`);
    iconContent.value = iconModule.default;
  } catch (error) {
    console.error(`Icon not found: ${props.name}`, error);
    iconContent.value = '';
  }
};

watch(() => props.name, loadIcon, { immediate: true });
</script>

<style lang="scss" scoped>
.icon {
  display: inline-block;
  vertical-align: middle;
  line-height: 1;
  fill: currentColor;
  transition: all var(--transition-normal);
  cursor: pointer;
}

.icon :deep(svg) {
  width: 1em;
  height: 1em;
}
</style>