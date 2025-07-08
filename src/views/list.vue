<template>
  <WeatherBackground :show-weather-info="false" container-class="list-view">
    <div class="list-view" @contextmenu.prevent>
      <!-- 顶部时间显示组件 -->
      <TopTimeDisplay />
    
    <div class="add-button-container">
      <router-link to="/add" class="add-btn">+</router-link>
    </div>
    <div class="list-container">
      <div v-if="list.length > 0" class="drag-area">
        <div v-for="(item, index) in list" :key="item.id" class="list-item"
          :class="{ 'is-expanded': item.expanded, 'is-dragging': isDragging && dragIndex === index }"
          @mousedown="prepareLongPress($event, index)" @mouseup="cancelLongPressAction" @mouseleave="cancelLongPressAction"
          @click="handleClick(item)" @contextmenu.prevent="showContextMenu($event, item, index)">
          <div class="item-header">
            <div class="title-with-level">
              <div class="level-color-block" :class="getLevelClass(item.level)" :title="getLevelText(item.level)"></div>
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

    <!-- 右键菜单 -->
    <div v-if="contextMenu.visible" class="context-menu" :style="contextMenuStyle" @click.stop>
      <div class="context-menu-item" @click="completeTodo">
        <span class="menu-icon">✅</span>
        <span>完成</span>
      </div>
      <div class="context-menu-item" @click="editTodo">
        <span class="menu-icon">✏️</span>
        <span>修改</span>
      </div>
      <div class="context-menu-item danger" @click="deleteTodo">
        <span class="menu-icon">🗑️</span>
        <span>删除</span>
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
  </WeatherBackground>
</template>

<script setup lang="ts">
import { RouterLink, useRouter } from 'vue-router';
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { Todo } from '@/model/todo';
import { GDate } from "@/common/date"
import Empty from '@/components/Empty.vue';
import TopTimeDisplay from '@/components/TopTimeDisplay.vue';
import WeatherBackground from '@/components/WeatherBackground.vue';
import { useLongPressTimer, useUIFeedbackTimer } from '@/composables/useTimer';
import { databaseApi, todoApi } from '@/api/services';

const router = useRouter();

const list = ref<Todo[]>([]);
const filterDays = ref(5); // 默认显示最近5天

const loadTodos = async () => {
  try {
    // 从数据库加载窗口设置，获取recent_days配置
    const settings = await databaseApi.window.load();
    filterDays.value = (settings as any).recent_days || 5;
    
    const todos = await todoApi.getRecent(filterDays.value);
    list.value = (todos as Todo[]).map(todo => ({ ...todo, expanded: false }));
    console.log('Loaded todos with recent days:', filterDays.value, todos);
  } catch (error) {
    console.error('Failed to load todos:', error);
  }
};



onMounted(() => {
  loadTodos();
  document.addEventListener('click', handleGlobalClick);
});

onUnmounted(() => {
  document.removeEventListener('click', handleGlobalClick);
});

const isDragging = ref(false);
const dragIndex = ref(-1);
const dragPreview = ref<any>(null);
const dragAction = ref('');
const pointer = ref({ x: 0, y: 0 });
const pressedIndex = ref<number | null>(null);
const justFinishedDragging = ref(false); // 新增：标记是否刚完成拖拽

// 右键菜单状态
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  todo: null as Todo | null,
  index: -1
});

// 使用长按定时器管理
const { startLongPress, cancelLongPress } = useLongPressTimer();
// 使用UI反馈定时器管理
const { createFeedbackTimer } = useUIFeedbackTimer();

const previewStyle = computed(() => ({
  top: pointer.value.y + 'px',
  left: pointer.value.x + 'px'
}));

// 右键菜单样式
const contextMenuStyle = computed(() => ({
  top: contextMenu.value.y + 'px',
  left: contextMenu.value.x + 'px'
}));

// 显示右键菜单
const showContextMenu = (event: MouseEvent, todo: Todo, index: number) => {
  event.preventDefault();
  event.stopPropagation();
  
  const menuWidth = 120;
  const menuHeight = 120;
  const windowWidth = window.innerWidth;
  const windowHeight = window.innerHeight;
  
  let x = event.clientX;
  let y = event.clientY;
  
  // 防止菜单超出屏幕边界
  if (x + menuWidth > windowWidth) {
    x = windowWidth - menuWidth - 10;
  }
  if (y + menuHeight > windowHeight) {
    y = windowHeight - menuHeight - 10;
  }
  
  contextMenu.value = {
    visible: true,
    x,
    y,
    todo,
    index
  };
};

// 隐藏右键菜单
const hideContextMenu = () => {
  contextMenu.value.visible = false;
};

// 全局点击事件处理
const handleGlobalClick = (event: MouseEvent) => {
  // 如果点击的不是右键菜单区域，则隐藏菜单
  const target = event.target as HTMLElement;
  if (!target.closest('.context-menu')) {
    hideContextMenu();
  }
};

// 完成todo
const completeTodo = async () => {
  if (!contextMenu.value.todo) return;
  
  try {
    const updatedTodo = { ...contextMenu.value.todo, status: 1 };
    await todoApi.update(updatedTodo);
    list.value.splice(contextMenu.value.index, 1);
  } catch (error) {
    console.error('Failed to complete todo:', error);
  }
  hideContextMenu();
};

// 编辑todo
const editTodo = () => {
  if (!contextMenu.value.todo) return;
  
  router.push(`/add?id=${contextMenu.value.todo.id}`);
  hideContextMenu();
};

// 删除todo
const deleteTodo = async () => {
  if (!contextMenu.value.todo) return;
  
  try {
    await todoApi.delete(contextMenu.value.todo.id);
    list.value.splice(contextMenu.value.index, 1);
  } catch (error) {
    console.error('Failed to delete todo:', error);
  }
  hideContextMenu();
};

const prepareLongPress = (e: MouseEvent, index: number) => {
  pressedIndex.value = index;
  startLongPress(() => {
    startDrag(e, index);
  }, 300);
};

const cancelLongPressAction = () => {
  cancelLongPress();
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
  cancelLongPressAction();
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

// 获取截止时间显示文字
const getDueDateText = (item: Todo) => {
  // 优先使用endTime，如果没有则使用startTime
  const dueTime = item.endTime || item.startTime;
  if (!dueTime) return '未设置截止时间';

  const dueDate = new GDate(dueTime);
  const today = new GDate();
  
  // 使用日期的开始时间进行比较，确保计算准确
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

// 获取优先级文本
const getLevelText = (level: number) => {
  switch (level) {
    case 0: return '重要不紧急';
    case 1: return '重要且紧急';
    case 2: return '不重要不紧急';
    case 3: return '不重要但紧急';
    default: return '未分类';
  }
};

// 获取优先级样式类名
const getLevelClass = (level: number) => {
  switch (level) {
    case 0: return 'level-important-not-urgent';
    case 1: return 'level-important-urgent';
    case 2: return 'level-not-important-not-urgent';
    case 3: return 'level-not-important-urgent';
    default: return 'level-uncategorized';
  }
};

// 获取截止时间样式类名
const getDueDateClass = (item: Todo) => {
  // 优先使用endTime，如果没有则使用startTime
  const dueTime = item.endTime || item.startTime;
  if (!dueTime) return 'due-date-none';

  const dueDate = new GDate(dueTime);
  const today = new GDate();
  
  // 使用日期的开始时间进行比较，确保计算准确
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

const onMouseUp = async () => {
  let wasDragging = isDragging.value; // 记录是否在拖拽状态

  if (dragIndex.value >= 0) {
    const todo = list.value[dragIndex.value];
    if (dragAction.value === 'delete') {
      try {
        await todoApi.delete(todo.id);
        list.value.splice(dragIndex.value, 1);
      } catch (error) {
        console.error('Failed to delete todo:', error);
      }
    } else if (dragAction.value === 'complete') {
      try {
        const updatedTodo = { ...todo, status: 1 };
        await todoApi.update(updatedTodo);
        list.value.splice(dragIndex.value, 1); // 从列表中移除
      } catch (error) {
        console.error('Failed to complete todo:', error);
      }
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
    createFeedbackTimer(() => {
      justFinishedDragging.value = false;
    }, 50, 'dragFinished');
  }

  // 恢复页面滚动
  document.body.style.overflow = '';

  window.removeEventListener('mousemove', onMouseMove);
  window.removeEventListener('mouseup', onMouseUp);
};
</script>

<style scoped>
.filter-container {
  position: fixed;
  top: 20px;
  left: 20px;
  z-index: 50;
  background: var(--bg-secondary);
  padding: 10px 15px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(10px);
}

.filter-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--text-primary);
}

.filter-item label {
  font-weight: 500;
}

.filter-item select {
  padding: 4px 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: border-color 0.2s ease;
}

.filter-item select:focus {
  outline: none;
  border-color: var(--accent-color);
}

.filter-item select:hover {
  border-color: var(--accent-color);
}

.add-button-container {
  position: fixed;
  bottom: 30px;
  right: 30px;
  z-index: 100;
}

.add-btn {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background-color: var(--accent-color);
  color: white;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 20px;
  text-decoration: none;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  transition: all 0.3s ease;
  border: 2px solid transparent;
}

.add-btn:hover {
  transform: scale(1.05);
  background-color: var(--accent-color-hover, var(--accent-color));
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
  border-color: var(--bg-primary);
}

.add-btn:active {
  transform: scale(0.95);
  transition: all 0.1s ease;
}

.list-view {
  width: 100%;
  height: 100%;
  position: relative;
  background: var(--bg-primary);
  overflow: hidden;
}

.list-container {
  padding: 5px 20px;
  overflow-y: auto;
  height: 100%;
  /* 优化滚动条样式 */
  scrollbar-width: thin;
  scrollbar-color: rgba(155, 155, 155, 0.5) transparent;
  padding-top: 46px;
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
  align-items: center;
  font-weight: bold;
}

.title-with-level {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
}

.level-color-block {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  flex-shrink: 0;
  transition: all 0.3s ease;
  cursor: help;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

/* 优先级颜色块样式 */
.level-important-urgent {
  background: var(--level-important-urgent-color, #ff4757);
  box-shadow: 0 0 8px rgba(255, 71, 87, 0.4);
}

.level-important-not-urgent {
  background: var(--level-important-not-urgent-color, #ffa726);
  box-shadow: 0 0 8px rgba(255, 167, 38, 0.3);
}

.level-not-important-urgent {
  background: var(--level-not-important-urgent-color, #ffca28);
  box-shadow: 0 0 8px rgba(255, 202, 40, 0.3);
}

.level-not-important-not-urgent {
  background: var(--level-not-important-not-urgent-color, #66bb6a);
  box-shadow: 0 0 8px rgba(102, 187, 106, 0.3);
}

.level-uncategorized {
  background: var(--level-uncategorized-color, #bdbdbd);
  box-shadow: 0 0 8px rgba(189, 189, 189, 0.2);
}

/* 颜色块悬停效果 */
.level-color-block:hover {
  transform: scale(1.2);
  box-shadow: 0 0 12px rgba(0, 0, 0, 0.3);
}

.item-title {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.item-due-date {
  font-size: 12px;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 12px;
  transition: all 0.3s ease;
}

/* 截止时间样式 */
.due-date-overdue {
  background: linear-gradient(135deg, #ff4757, #ff3742);
  color: white;
  animation: pulse-urgent 2s ease-in-out infinite;
  box-shadow: 0 2px 8px rgba(255, 71, 87, 0.4);
}

.due-date-today {
  background: linear-gradient(135deg, #ff6b35, #f7931e);
  color: white;
  animation: shake-gentle 1s ease-in-out infinite;
  box-shadow: 0 2px 8px rgba(255, 107, 53, 0.4);
}

.due-date-tomorrow {
  background: linear-gradient(135deg, #ffa726, #ffb74d);
  color: white;
  box-shadow: 0 2px 6px rgba(255, 167, 38, 0.3);
}

.due-date-urgent {
  background: linear-gradient(135deg, #ffca28, #ffd54f);
  color: #333;
  box-shadow: 0 2px 6px rgba(255, 202, 40, 0.3);
}

.due-date-soon {
  background: linear-gradient(135deg, #66bb6a, #81c784);
  color: white;
  box-shadow: 0 2px 6px rgba(102, 187, 106, 0.3);
}

.due-date-normal {
  background: linear-gradient(135deg, #42a5f5, #64b5f6);
  color: white;
  box-shadow: 0 2px 6px rgba(66, 165, 245, 0.3);
}

.due-date-none {
  background: linear-gradient(135deg, #bdbdbd, #e0e0e0);
  color: #666;
  box-shadow: 0 2px 6px rgba(189, 189, 189, 0.2);
}

/* 动画效果 */
@keyframes pulse-urgent {

  0%,
  100% {
    transform: scale(1);
    opacity: 1;
  }

  50% {
    transform: scale(1.05);
    opacity: 0.9;
  }
}

@keyframes shake-gentle {

  0%,
  100% {
    transform: translateX(0);
  }

  25% {
    transform: translateX(-1px);
  }

  75% {
    transform: translateX(1px);
  }
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
  /* animation: shake 0.5s ease-in-out infinite; */
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

/* 右键菜单样式 */
.context-menu {
  position: fixed;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  z-index: 1001;
  min-width: 120px;
  padding: 4px 0;
  backdrop-filter: blur(10px);
  animation: contextMenuFadeIn 0.15s ease-out;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  font-size: 14px;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;
}

.context-menu-item:hover {
  background: var(--accent-color);
  color: white;
}

.context-menu-item.danger:hover {
  background: #ff4757;
  color: white;
}

.menu-icon {
  font-size: 16px;
  width: 18px;
  text-align: center;
  flex-shrink: 0;
}

@keyframes contextMenuFadeIn {
  from {
    opacity: 0;
    transform: scale(0.95) translateY(-5px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}
</style>
