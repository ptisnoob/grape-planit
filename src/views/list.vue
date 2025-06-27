<template>
  <div class="list-view">
    <div class="list-container">
      <div class="drag-area">
        <div v-for="(item, index) in list" :key="item.id" class="list-item"
          :class="{ 'is-expanded': item.expanded, 'is-dragging': isDragging && dragIndex === index }"
          @mousedown="prepareLongPress($event, index)" @mouseup="cancelLongPress" @mouseleave="cancelLongPress"
          @click="handleClick(item)">
          <div class="item-header">
            <span class="item-title">{{ item.title }}</span>
            <span class="item-due-date">截止时间: {{ item.startTime }}</span>
          </div>
          <transition name="expand">
            <div v-if="item.expanded" class="item-notes">
              <p>{{ item.notes }}</p>
            </div>
          </transition>
        </div>
      </div>
    </div>

    <!-- 删除区域 -->
    <div v-show="isDragging" class="drop-zone delete-zone" :class="{ 'is-active': dragAction === 'delete' }">
      <div class="drop-zone-content">
        <div class="drop-zone-icon">🗑️</div>
        <span>{{ dragAction === 'delete' ? '松手删除' : '删除' }}</span>
      </div>
    </div>

    <!-- 完成区域 -->
    <div v-show="isDragging" class="drop-zone complete-zone" :class="{ 'is-active': dragAction === 'complete' }">
      <div class="drop-zone-content">
        <div class="drop-zone-icon">✅</div>
        <span>{{ dragAction === 'complete' ? '松手完成' : '完成' }}</span>
      </div>
    </div>

    <!-- 跟随指针的拖拽预览 -->
    <div v-if="isDragging && dragPreview" class="drag-preview" :style="previewStyle">
      {{ dragPreview.title }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { Todo } from '@/model/todo';


const list = ref<Todo[]>([
  {
    id: 1, title: '完成项目报告', startTime: '2024-07-20', notes: '包含Q2数据', expanded: false,
    level: 0,
    status: 0,
    cycle: 'one'
  },
  {
    id: 2, title: '学习 Vue 3', startTime: '2024-07-22', notes: '组合式 API', expanded: false,
    level: 0,
    status: 0,
    cycle: 'one'
  },
  {
    id: 3, title: '整理文件', startTime: '2024-07-25', notes: '清理旧文档', expanded: false,
    level: 0,
    status: 0,
    cycle: 'one'
  },
]);

const isDragging = ref(false);
const dragIndex = ref(-1);
const dragPreview = ref<any>(null);
const dragAction = ref('');
const pointer = ref({ x: 0, y: 0 });
const longPressTimer = ref<number | null>(null);
const pressedIndex = ref<number | null>(null);
const justFinishedDragging = ref(false); // 新增：标记是否刚完成拖拽

const previewStyle = computed(() => ({
  top: pointer.value.y + 'px',
  left: pointer.value.x + 'px'
}));

const prepareLongPress = (e: MouseEvent, index: number) => {
  pressedIndex.value = index;
  longPressTimer.value = window.setTimeout(() => {
    startDrag(e, index);
  }, 300);
};

const cancelLongPress = () => {
  if (longPressTimer.value !== null) {
    clearTimeout(longPressTimer.value);
    longPressTimer.value = null;
  }
};

const handleClick = (item: any) => {
  // 如果刚完成拖拽或正在拖拽，不触发点击事件
  if (!isDragging.value && !justFinishedDragging.value) {
    item.expanded = !item.expanded;
  }
  // 重置拖拽完成标志
  justFinishedDragging.value = false;
};

const startDrag = (e: MouseEvent, index: number) => {
  cancelLongPress();
  e.preventDefault();
  isDragging.value = true;
  dragIndex.value = index;
  dragPreview.value = { ...list.value[index] };

  pointer.value = { x: e.clientX + 10, y: e.clientY + 10 };

  // 防止页面滚动
  document.body.style.overflow = 'hidden';

  window.addEventListener('mousemove', onMouseMove);
  window.addEventListener('mouseup', onMouseUp);
};

const onMouseMove = (e: MouseEvent) => {
  // 限制拖拽预览位置，防止超出视窗边界
  const maxX = window.innerWidth;
  const maxY = window.innerHeight;

  pointer.value = {
    x: Math.min(Math.max(e.clientX + 10, 10), maxX),
    y: Math.min(Math.max(e.clientY + 10, 10), maxY)
  };

  const width = window.innerWidth;
  // 调整触发阈值，让完成区域更容易触发
  const deleteThreshold = width * 0.3; // 删除区域阈值
  const completeThreshold = width * 0.7; // 完成区域阈值，从70%开始就触发

  if (e.clientX < deleteThreshold) {
    dragAction.value = 'delete';
  } else if (e.clientX > completeThreshold) {
    dragAction.value = 'complete';
  } else {
    dragAction.value = '';
  }
};

const onMouseUp = () => {
  let wasDragging = isDragging.value; // 记录是否在拖拽状态

  if (dragIndex.value >= 0) {
    if (dragAction.value === 'delete') {
      list.value.splice(dragIndex.value, 1);
    } else if (dragAction.value === 'complete') {
      list.value[dragIndex.value].title += ' ✅';
    }
  }

  isDragging.value = false;
  dragAction.value = '';
  dragIndex.value = -1;
  dragPreview.value = null;

  // 如果刚才在拖拽，设置标志防止触发点击事件
  if (wasDragging) {
    justFinishedDragging.value = true;
    // 短暂延迟后重置标志，确保点击事件被阻止
    setTimeout(() => {
      justFinishedDragging.value = false;
    }, 50);
  }

  // 恢复页面滚动
  document.body.style.overflow = '';

  window.removeEventListener('mousemove', onMouseMove);
  window.removeEventListener('mouseup', onMouseUp);
};
</script>

<style scoped>
.list-view {
  width: 100%;
  height: 100%;
  position: relative;
  background: var(--bg-primary);
  overflow: hidden;
  padding-top: 6px;
}

.list-container {
  padding: 20px;
  overflow-y: auto;
  height: 100%;
  /* 优化滚动条样式 */
  scrollbar-width: thin;
  scrollbar-color: rgba(155, 155, 155, 0.5) transparent;
}

/* Webkit浏览器滚动条样式 */
.list-container::-webkit-scrollbar {
  width: 6px;
}

.list-container::-webkit-scrollbar-track {
  background: transparent;
}

.list-container::-webkit-scrollbar-thumb {
  background-color: rgba(155, 155, 155, 0.5);
  border-radius: 3px;
  transition: background-color 0.3s ease;
}

.list-container::-webkit-scrollbar-thumb:hover {
  background-color: rgba(155, 155, 155, 0.8);
}

.drag-area {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.list-item {
  background: var(--bg-secondary);
  padding: 10px;
  border-radius: 8px;
  cursor: grab;
  transition: box-shadow 0.2s, transform 0.2s;
  user-select: none;
}

.list-item:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.list-item:active {
  cursor: grabbing;
}

.list-item.is-dragging {
  opacity: 0.5;
  cursor: grabbing;
}

.item-header {
  display: flex;
  justify-content: space-between;
  font-weight: bold;
}

.item-notes {
  margin-top: 8px;
  font-size: 14px;
  color: var(--text-secondary);
}

/* 展开动画 */
.expand-enter-active,
.expand-leave-active {
  transition: all 0.3s ease;
  overflow: hidden;
}

.expand-enter-from,
.expand-leave-to {
  opacity: 0;
  max-height: 0;
  transform: translateY(-10px);
}

.expand-enter-to,
.expand-leave-from {
  opacity: 1;
  max-height: 200px;
  transform: translateY(0);
}

.drop-zone {
  position: absolute;
  top: 0;
  width: 30%;
  /* 增加宽度到30% */
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  font-weight: bold;
  color: white;
  pointer-events: none;
  font-size: 18px;
  transition: all 0.3s ease;
  z-index: 10;
  backdrop-filter: blur(2px);
}

.drop-zone-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  transform: scale(0.9);
  transition: transform 0.3s ease;
}

.drop-zone-icon {
  font-size: 32px;
  opacity: 0.8;
  transition: all 0.3s ease;
}

.delete-zone {
  left: 0;
  background: linear-gradient(to right, rgba(255, 77, 79, 0.4), rgba(255, 77, 79, 0.1));
  border-right: 2px solid rgba(255, 77, 79, 0.3);
}

.complete-zone {
  right: 0;
  background: linear-gradient(to left, rgba(100, 222, 149, 0.4), rgba(100, 222, 149, 0.1));
  border-left: 2px solid rgba(100, 222, 149, 0.3);
}

.delete-zone.is-active {
  background: linear-gradient(to right, rgba(255, 77, 79, 0.9), rgba(255, 77, 79, 0.3));
  border-right: 3px solid rgba(255, 255, 255, 0.8);
  animation: shake 0.5s ease-in-out infinite;
  width: 35%;
  /* 激活时扩展宽度 */
}

.delete-zone.is-active .drop-zone-content {
  transform: scale(1.1);
}

.delete-zone.is-active .drop-zone-icon {
  opacity: 1;
  animation: bounce 0.6s ease-in-out infinite alternate;
}

.complete-zone.is-active {
  background: linear-gradient(to left, rgba(100, 222, 149, 0.9), rgba(100, 222, 149, 0.3));
  border-left: 3px solid rgba(255, 255, 255, 0.8);
  animation: glow 0.8s ease-in-out infinite alternate;
  width: 35%;
  /* 激活时扩展宽度，与删除区域保持一致 */
}

.complete-zone.is-active .drop-zone-content {
  transform: scale(1.1);
}

.complete-zone.is-active .drop-zone-icon {
  opacity: 1;
  animation: pulse-icon 0.8s ease-in-out infinite alternate;
}

@keyframes shake {

  0%,
  100% {
    transform: translateX(0);
  }

  25% {
    transform: translateX(-2px);
  }

  75% {
    transform: translateX(2px);
  }
}

@keyframes bounce {
  from {
    transform: translateY(0);
  }

  to {
    transform: translateY(-5px);
  }
}

@keyframes glow {
  from {
    box-shadow: 0 0 10px rgba(100, 222, 149, 0.5);
  }

  to {
    box-shadow: 0 0 20px rgba(100, 222, 149, 0.8);
  }
}

@keyframes pulse-icon {
  from {
    transform: scale(1);
  }

  to {
    transform: scale(1.2);
  }
}

.drag-preview {
  position: fixed;
  pointer-events: none;
  background: var(--bg-secondary);
  border: 2px solid rgba(100, 149, 237, 0.5);
  padding: 10px 15px;
  border-radius: 8px;
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.3);
  z-index: 1000;
  transform: translate(-50%, -50%) rotate(-2deg);
  font-size: 14px;
  font-weight: bold;
  max-width: 250px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  opacity: 0.95;
  backdrop-filter: blur(4px);
}
</style>
