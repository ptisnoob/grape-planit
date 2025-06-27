<template>
  <div class="modal-overlay" v-if="visible" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <div class="modal-header" v-if="title">
        <h3>{{ title }}</h3>
        <button class="close-btn" @click="$emit('close')" v-if="showCloseButton">
          ✕
        </button>
      </div>
      <div class="modal-body">
        <slot></slot>
      </div>
      <div class="modal-footer" v-if="$slots.footer">
        <slot name="footer"></slot>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { watch } from 'vue'

interface Props {
  visible: boolean
  title?: string
  showCloseButton?: boolean
  closeOnOverlay?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  showCloseButton: true,
  closeOnOverlay: true
})

const emit = defineEmits<{
  close: []
}>()

const handleOverlayClick = () => {
  if (props.closeOnOverlay) {
    emit('close')
  }
}

// 控制body滚动
watch(() => props.visible, (newVisible) => {
  if (newVisible) {
    document.body.style.overflow = 'hidden'
  } else {
    document.body.style.overflow = ''
  }
})
</script>

<style lang="scss" scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.modal-content {
  background: var(--bg-primary);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  border: 1px solid var(--border-color);
  max-width: 90vw;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-body {
  padding: 16px;
  flex: 1;
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px 8px;
  border-bottom: 1px solid var(--border-color);

  h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 16px;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 2px;
    border-radius: 4px;
    transition: all var(--transition-normal);

    &:hover {
      background: var(--bg-hover);
      color: var(--text-primary);
    }
  }
}



.modal-footer {
  padding: 12px 16px;
  border-top: 1px solid var(--border-color);
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}
</style>