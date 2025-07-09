<template>
  <div class="list-container">
    <div v-if="list.length > 0" class="drag-area">
      <div v-for="(item, index) in list" :key="item.id" class="list-item"
        :class="{ 'is-expanded': item.expanded, 'is-dragging': isDragging && dragIndex === index }"
        @mousedown="prepareLongPress($event, index)" @mouseup="cancelLongPressAction"
        @mouseleave="cancelLongPressAction" @click="handleClick(item)" @dblclick="enterFocusMode(item)"
        @contextmenu="handleContextMenu($event, item, index)">
        <div class="item-header">
          <div class="title-with-level">
            <div class="level-color-block" :class="getLevelClass(item.level)" :title="getLevelText(item.level)">
            </div>
            <span class="item-title">{{ item.title }}</span>
          </div>
          <span class="item-due-date" :class="getDueDateClass(item)">{{ getDueDateText(item)
            }}</span>
        </div>
        <transition name="expand">
          <div v-if="item.expanded" class="item-notes">
            <p>{{ item.notes }}</p>
          </div>
        </transition>
      </div>
    </div>
    <Empty v-else>暂无待办事项</Empty>
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
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { Todo } from '@/model/todo';
import { GDate } from "@/common/date"
import Empty from '@/components/Empty.vue';
import { useLongPressTimer, useUIFeedbackTimer } from '@/composables/useTimer';
import { todoApi } from '@/api/services';

interface Props {
  list: Todo[];
}

interface Emits {
  (e: 'update:list', value: Todo[]): void;
  (e: 'enterFocusMode', todo: Todo): void;
  (e: 'contextMenu', event: MouseEvent, todo: Todo, index: number): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// 拖拽相关状态
const isDragging = ref(false);
const dragIndex = ref(-1);
const dragPreview = ref<any>(null);
const dragAction = ref('');
const pointer = ref({ x: 0, y: 0 });
const pressedIndex = ref<number | null>(null);
const justFinishedDragging = ref(false);



// 使用定时器管理
const { startLongPress, cancelLongPress } = useLongPressTimer();
const { createFeedbackTimer } = useUIFeedbackTimer();

// 计算属性
const previewStyle = computed(() => ({
  top: pointer.value.y + 'px',
  left: pointer.value.x + 'px'
}));



// 事件处理函数
const prepareLongPress = (e: MouseEvent, index: number) => {
  pressedIndex.value = index;
  startLongPress(() => {
    startDrag(e, index);
  }, 300);
};

const cancelLongPressAction = () => {
  cancelLongPress();
};

const handleClick = (item: Todo) => {
  if (!isDragging.value && !justFinishedDragging.value) {
    item.expanded = !item.expanded;
  }
  justFinishedDragging.value = false;
};

const enterFocusMode = (item: Todo) => {
  emit('enterFocusMode', item);
};

const handleContextMenu = (event: MouseEvent, todo: Todo, index: number) => {
  event.preventDefault();
  emit('contextMenu', event, todo, index);
};

// 拖拽相关函数
const startDrag = (e: MouseEvent, index: number) => {
  cancelLongPressAction();
  e.preventDefault();
  isDragging.value = true;
  dragIndex.value = index;
  dragPreview.value = { ...props.list[index] };

  pointer.value = { x: e.clientX + 10, y: e.clientY + 10 };
  document.body.style.overflow = 'hidden';

  window.addEventListener('mousemove', onMouseMove);
  window.addEventListener('mouseup', onMouseUp);
};

const onMouseMove = (e: MouseEvent) => {
  const maxX = window.innerWidth;
  const maxY = window.innerHeight;

  pointer.value = {
    x: Math.min(Math.max(e.clientX + 10, 10), maxX),
    y: Math.min(Math.max(e.clientY + 10, 10), maxY)
  };

  const width = window.innerWidth;
  const deleteThreshold = width * 0.3;
  const completeThreshold = width * 0.7;

  if (e.clientX < deleteThreshold) {
    dragAction.value = 'delete';
  } else if (e.clientX > completeThreshold) {
    dragAction.value = 'complete';
  } else {
    dragAction.value = '';
  }
};

const onMouseUp = async () => {
  let wasDragging = isDragging.value;

  if (dragIndex.value >= 0) {
    const todo = props.list[dragIndex.value];
    const newList = [...props.list];
    
    if (dragAction.value === 'delete') {
      try {
        await todoApi.delete(todo.id);
        newList.splice(dragIndex.value, 1);
        emit('update:list', newList);
      } catch (error) {
        console.error('Failed to delete todo:', error);
      }
    } else if (dragAction.value === 'complete') {
      try {
        const updatedTodo = { ...todo, status: 1 };
        await todoApi.update(updatedTodo);
        newList.splice(dragIndex.value, 1);
        emit('update:list', newList);
      } catch (error) {
        console.error('Failed to complete todo:', error);
      }
    }
  }

  isDragging.value = false;
  dragAction.value = '';
  dragIndex.value = -1;
  dragPreview.value = null;

  if (wasDragging) {
    justFinishedDragging.value = true;
    createFeedbackTimer(() => {
      justFinishedDragging.value = false;
    }, 50, 'dragFinished');
  }

  document.body.style.overflow = '';
  window.removeEventListener('mousemove', onMouseMove);
  window.removeEventListener('mouseup', onMouseUp);
};



// 工具函数
const getDueDateText = (item: Todo) => {
  const dueTime = item.endTime || item.startTime;
  if (!dueTime) return '未设置截止时间';

  const dueDate = new GDate(dueTime);
  const today = new GDate();
  const dueDateStart = dueDate.getStartOfDay();
  const todayStart = today.getStartOfDay();
  const diffDays = Math.round((dueDateStart.getTime() - todayStart.getTime()) / (1000 * 60 * 60 * 24));

  if (diffDays < 0) {
    const overdueDays = Math.abs(diffDays);
    return `已逾期 ${overdueDays} 天`;
  } else if (diffDays === 0) {
    return '今天是最后一天啦！';
  } else if (diffDays === 1) {
    return '明天截止';
  } else if (diffDays <= 3) {
    return `还有 ${diffDays} 天`;
  } else if (diffDays <= 7) {
    return `还有 ${diffDays} 天`;
  } else {
    return `还有 ${diffDays} 天`;
  }
};

const getLevelText = (level: number) => {
  switch (level) {
    case 0: return '重要不紧急';
    case 1: return '重要且紧急';
    case 2: return '不重要不紧急';
    case 3: return '不重要但紧急';
    default: return '未分类';
  }
};

const getLevelClass = (level: number) => {
  switch (level) {
    case 0: return 'level-important-not-urgent';
    case 1: return 'level-important-urgent';
    case 2: return 'level-not-important-not-urgent';
    case 3: return 'level-not-important-urgent';
    default: return 'level-uncategorized';
  }
};

const getDueDateClass = (item: Todo) => {
  const dueTime = item.endTime || item.startTime;
  if (!dueTime) return 'due-date-none';

  const dueDate = new GDate(dueTime);
  const today = new GDate();
  const dueDateStart = dueDate.getStartOfDay();
  const todayStart = today.getStartOfDay();
  const diffDays = Math.round((dueDateStart.getTime() - todayStart.getTime()) / (1000 * 60 * 60 * 24));

  if (diffDays < 0) {
    return 'due-date-overdue';
  } else if (diffDays === 0) {
    return 'due-date-today';
  } else if (diffDays === 1) {
    return 'due-date-tomorrow';
  } else if (diffDays <= 3) {
    return 'due-date-urgent';
  } else if (diffDays <= 7) {
    return 'due-date-soon';
  } else {
    return 'due-date-normal';
  }
};


</script>

<style scoped>
/* ListView组件样式 - 支持深色/浅色主题 */
.list-container {
  flex: 1;
  overflow-y: auto;
  padding: 0 10px;
}

.drag-area {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.list-item {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 12px;
  cursor: pointer;
  transition: all var(--transition-fast);
  border: 1px solid var(--border-color);
  user-select: none;
  backdrop-filter: blur(10px);
}

.list-item:hover {
  background: var(--bg-primary);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px var(--shadow);
}

.list-item.is-expanded {
  background: var(--bg-primary);
  border-color: var(--accent-color);
}

.list-item.is-dragging {
  opacity: 0.5;
  transform: scale(0.95);
}

.item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.title-with-level {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  min-width: 0;
}

.level-color-block {
  width: 4px;
  height: 16px;
  border-radius: 2px;
  flex-shrink: 0;
}

.level-important-urgent {
  background: #ff4757;
}

.level-important-not-urgent {
  background: #ffa502;
}

.level-not-important-urgent {
  background: #3742fa;
}

.level-not-important-not-urgent {
  background: #2ed573;
}

.level-uncategorized {
  background: #747d8c;
}

.item-title {
  color: var(--text-primary);
  font-size: 14px;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.item-due-date {
  font-size: 12px;
  flex-shrink: 0;
  margin-left: 8px;
}

.due-date-overdue {
  color: #ff4757;
  font-weight: bold;
}

.due-date-today {
  color: #ffa502;
  font-weight: bold;
}

.due-date-tomorrow {
  color: #ff6b6b;
}

.due-date-urgent {
  color: #feca57;
}

.due-date-soon {
  color: #48dbfb;
}

.due-date-normal {
  color: var(--text-secondary);
}

.due-date-none {
  color: var(--text-secondary);
  opacity: 0.7;
}

.item-notes {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--border-color);
}

.item-notes p {
  color: var(--text-secondary);
  font-size: 12px;
  line-height: 1.4;
  margin: 0;
}

.expand-enter-active,
.expand-leave-active {
  transition: all 0.3s ease;
  overflow: hidden;
}

.expand-enter-from,
.expand-leave-to {
  max-height: 0;
  opacity: 0;
}

.expand-enter-to,
.expand-leave-from {
  max-height: 100px;
  opacity: 1;
}



/* 拖拽区域样式 */
.drop-zone {
  position: fixed;
  top: 50%;
  transform: translateY(-50%);
  width: 80px;
  height: 80px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  z-index: 999;
}

.delete-zone {
  left: 20px;
  background: rgba(255, 71, 87, 0.3);
  border: 2px dashed rgba(255, 71, 87, 0.5);
}

.complete-zone {
  right: 20px;
  background: rgba(46, 213, 115, 0.3);
  border: 2px dashed rgba(46, 213, 115, 0.5);
}

.drop-zone.is-active {
  transform: translateY(-50%) scale(1.2);
  background: rgba(255, 255, 255, 0.2);
}

.delete-zone.is-active {
  background: rgba(255, 71, 87, 0.6);
  border-color: #ff4757;
}

.complete-zone.is-active {
  background: rgba(46, 213, 115, 0.6);
  border-color: #2ed573;
}

.drop-zone-content {
  text-align: center;
  color: white;
}

.drop-zone-icon {
  font-size: 24px;
  margin-bottom: 4px;
}

.drop-zone-content span {
  font-size: 10px;
  font-weight: bold;
}

/* 拖拽预览样式 */
.drag-preview {
  position: fixed;
  background: var(--bg-primary);
  color: var(--text-primary);
  padding: 8px 12px;
  border-radius: 6px;
  font-size: 12px;
  pointer-events: none;
  z-index: 1001;
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  backdrop-filter: blur(5px);
  border: 1px solid var(--border-color);
  box-shadow: 0 4px 12px var(--shadow);
}
</style>