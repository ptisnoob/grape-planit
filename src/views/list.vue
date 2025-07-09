<template>
  <WeatherBackground :show-weather-info="false" container-class="list-view">
    <!-- 应用头部 -->
    <!-- <AppHeader @mode-changed="handleModeChange" /> -->
    <!-- 专注模式界面 -->
    <div v-if="focusMode.isActive" class="focus-mode-overlay" @mouseenter="isHoveringFocusMode = true"
      @mouseleave="isHoveringFocusMode = false">
      <div class="focus-mode-container">
        <!-- 退出按钮 -->
        <transition enter-active-class="animate__animated animate__slideInDown animate__faster"
          leave-active-class="animate__animated animate__slideOutUp animate__faster">
          <button v-if="isHoveringFocusMode" class="focus-exit-btn" @click="exitFocusMode" title="退出专注模式">
            <Icon name="close" :size="20" />
          </button>
        </transition>

        <!-- 专注内容 -->
        <div class="focus-content">
          <!-- 任务标题 -->
          <h1 class="focus-title">{{ focusMode.todo?.title }}</h1>
          <!-- 任务描述 -->
          <p v-if="focusMode.todo?.notes" class="focus-notes">{{ focusMode.todo.notes }}</p>
          <!-- 专注计时器 -->
          <div class="focus-timer">
            <div class="timer-display">{{ formatFocusTime(focusMode.elapsedTime) }}</div>
          </div>

          <!-- 操作按钮 -->
          <transition enter-active-class="animate__animated animate__slideInUp animate__faster"
            leave-active-class="animate__animated animate__slideOutDown animate__faster">
            <div v-if="isHoveringFocusMode" class="focus-actions">
              <button class="focus-action-btn pause-btn" @click="toggleFocusTimer">
                {{ focusMode.isPaused ? '继续' : '暂停' }}
              </button>
              <button class="focus-action-btn complete-btn" @click="completeFocusedTodo">
                完成任务
              </button>
            </div>
          </transition>
        </div>
      </div>
    </div>

    <!-- 正常列表界面 -->
    <div v-else class="list-view" @contextmenu.prevent>
      <!-- 顶部时间显示组件 -->
      <TopTimeDisplay @changeMode="handleModeChange" />

      <div class="add-button-container">
        <router-link to="/add" class="add-btn">+</router-link>
      </div>

      <!-- 根据显示模式切换不同的视图组件 -->
      <ListView v-if="displayMode === 'list'" :list="list" @update:list="updateList"
        @enter-focus-mode="enterFocusMode" />

      <CategoryView v-else-if="displayMode === 'category'" :list="list" @enter-focus-mode="enterFocusMode"
        @item-click="handleCategoryItemClick" />

      <CalendarView v-else-if="displayMode === 'calendar'" :list="list" @enter-focus-mode="enterFocusMode"
        @item-click="handleCalendarItemClick" />
    </div>
  </WeatherBackground>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { ref, onMounted, onUnmounted } from 'vue';
import { Todo } from '@/model/todo';
import TopTimeDisplay from '@/components/TopTimeDisplay.vue';
import WeatherBackground from '@/components/WeatherBackground.vue';
import Icon from '@/components/Icon.vue';
import ListView from '@/components/ListView.vue';
import CategoryView from '@/components/CategoryView.vue';
import CalendarView from '@/components/CalendarView.vue';
import { useTimer } from '@/composables/useTimer';
import { databaseApi, todoApi } from '@/api/services';

// 显示模式状态
const displayMode = ref<'list' | 'category' | 'calendar'>('list');

// 专注模式状态
const focusMode = ref({
  isActive: false,
  todo: null as Todo | null,
  startTime: 0,
  elapsedTime: 0,
  isPaused: false
});

// 专注模式鼠标悬停状态
const isHoveringFocusMode = ref(false);

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

// 更新列表数据
const updateList = (newList: Todo[]) => {
  list.value = newList;
};

// 处理模式切换
const handleModeChange = (mode: string) => {
  console.log('changeMode', mode)
  switch (mode) {
    case 'list':
      displayMode.value = 'list';
      break;
    case 'category':
      displayMode.value = 'category';
      break;
    case 'calendar':
      displayMode.value = 'calendar';
      break;
    default:
      displayMode.value = 'list';
  }
};

// 处理分类视图的项目点击
const handleCategoryItemClick = (todo: Todo) => {
  // 可以在这里添加分类视图特有的点击逻辑
  console.log('Category item clicked:', todo);
};

// 处理日历视图的项目点击
const handleCalendarItemClick = (todo: Todo) => {
  // 可以在这里添加日历视图特有的点击逻辑
  console.log('Calendar item clicked:', todo);
};

onMounted(() => {
  loadTodos();
});

onUnmounted(() => {
  clearTimer('focusTimer');
});

// 使用专注模式定时器管理
const { createTimer, clearTimer } = useTimer();


// 专注模式相关函数
const enterFocusMode = (todo: Todo) => {
  focusMode.value = {
    isActive: true,
    todo: todo,
    startTime: Date.now(),
    elapsedTime: 0,
    isPaused: false
  };
  startFocusTimer();
};

const exitFocusMode = () => {
  clearTimer('focusTimer');
  focusMode.value = {
    isActive: false,
    todo: null,
    startTime: 0,
    elapsedTime: 0,
    isPaused: false
  };
};

const startFocusTimer = () => {
  // 清除已存在的专注定时器
  clearTimer('focusTimer');

  // 创建新的专注定时器
  const updateTimer = () => {
    if (!focusMode.value.isPaused) {
      focusMode.value.elapsedTime = Date.now() - focusMode.value.startTime;
    }
    // 递归创建下一个定时器
    createTimer('focusTimer', updateTimer, 1000);
  };

  createTimer('focusTimer', updateTimer, 1000);
};

const toggleFocusTimer = () => {
  if (focusMode.value.isPaused) {
    // 恢复计时
    focusMode.value.startTime = Date.now() - focusMode.value.elapsedTime;
    focusMode.value.isPaused = false;
    startFocusTimer();
  } else {
    // 暂停计时
    focusMode.value.isPaused = true;
    clearTimer('focusTimer');
  }
};

const completeFocusedTodo = async () => {
  if (!focusMode.value.todo) return;

  try {
    const updatedTodo = { ...focusMode.value.todo, status: 1 };
    await todoApi.update(updatedTodo);

    // 从列表中移除已完成的任务
    const index = list.value.findIndex(item => item.id === focusMode.value.todo?.id);
    if (index !== -1) {
      list.value.splice(index, 1);
    }

    exitFocusMode();
  } catch (error) {
    console.error('Failed to complete focused todo:', error);
  }
};

const formatFocusTime = (milliseconds: number) => {
  const totalSeconds = Math.floor(milliseconds / 1000);
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  if (hours > 0) {
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  } else {
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  }
};
</script>

<style scoped>
/* 专注模式样式 */
.focus-mode-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: linear-gradient(135deg,
      #1a1a2e 0%,
      #16213e 25%,
      #0f3460 50%,
      #533483 75%,
      #7209b7 100%);
  overflow: hidden;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: fadeIn 0.5s ease-out;
}

.focus-mode-container {
  position: relative;
  width: 85%;
  max-width: 500px;
  max-height: 85vh;
  background: rgba(255, 255, 255, 0.08);
  border-radius: 20px;
  border: 1px solid rgba(255, 255, 255, 0.15);
  backdrop-filter: blur(15px);
  padding: 30px 25px;
  text-align: center;
  box-shadow: 0 15px 35px rgba(0, 0, 0, 0.2);
  animation: slideUp 0.6s ease-out;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.focus-exit-btn {
  position: absolute;
  top: 20px;
  right: 20px;
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.2);
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.focus-exit-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  transform: scale(1.1);
}

.focus-content {
  color: white;
}

.focus-priority {
  margin-bottom: 15px;
  display: flex;
  justify-content: center;
}

.focus-level-indicator {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  animation: pulse 2s infinite;
}

.focus-level-indicator.level-important-not-urgent {
  background: #10b981;
}

.focus-level-indicator.level-important-urgent {
  background: #f59e0b;
}

.focus-level-indicator.level-not-important-not-urgent {
  background: #ef4444;
}

.focus-level-indicator.level-not-important-urgent {
  background: #8b5cf6;
}

.focus-title {
  font-size: 1.8rem;
  font-weight: bold;
  margin: 0 0 12px 0;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  line-height: 1.3;
  max-height: 2.6rem;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.focus-notes {
  font-size: 0.95rem;
  opacity: 0.85;
  margin: 0 0 20px 0;
  line-height: 1.4;
  max-height: 2.8rem;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.focus-timer {
  margin: 25px 0;
  flex-shrink: 0;
}

.timer-display {
  font-size: 3rem;
  font-weight: 300;
  font-family: 'Courier New', monospace;
  margin-bottom: 8px;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  letter-spacing: 2px;
}

.timer-label {
  font-size: 0.9rem;
  opacity: 0.8;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.focus-indicator {
  margin: 20px 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  flex-shrink: 0;
}

.focus-pulse {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #10b981;
  animation: focusPulse 1.5s infinite;
}

.focus-status {
  font-size: 0.9rem;
  opacity: 0.9;
}

.focus-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
  margin-top: 20px;
  flex-shrink: 0;
}

.focus-action-btn {
  padding: 10px 18px;
  border: 1.5px solid rgba(255, 255, 255, 0.25);
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.08);
  color: white;
  font-size: 0.85rem;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  transition: all 0.3s ease;
  backdrop-filter: blur(5px);
  min-width: 80px;
  justify-content: center;
}

.focus-action-btn:hover {
  background: rgba(255, 255, 255, 0.15);
  border-color: rgba(255, 255, 255, 0.4);
  transform: translateY(-1px);
}

.focus-action-btn.pause-btn:hover {
  background: rgba(255, 193, 7, 0.15);
  border-color: rgba(255, 193, 7, 0.4);
}

.focus-action-btn.complete-btn:hover {
  background: rgba(16, 185, 129, 0.15);
  border-color: rgba(16, 185, 129, 0.4);
}

/* 动画定义 */
@keyframes fadeIn {
  from {
    opacity: 0;
  }

  to {
    opacity: 1;
  }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(30px) scale(0.95);
  }

  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

@keyframes focusPulse {

  0%,
  100% {
    opacity: 1;
    transform: scale(1);
  }

  50% {
    opacity: 0.5;
    transform: scale(1.5);
  }
}

/* 响应式设计 */
@media (max-width: 768px) {
  .focus-mode-container {
    width: 95%;
    max-width: 400px;
    max-height: 90vh;
    padding: 20px 15px;
  }

  .focus-title {
    font-size: 1.5rem;
    margin-bottom: 8px;
  }

  .focus-notes {
    font-size: 0.85rem;
    margin-bottom: 15px;
  }

  .timer-display {
    font-size: 2.5rem;
  }

  .focus-timer {
    margin: 20px 0;
  }

  .focus-indicator {
    margin: 15px 0;
  }

  .focus-actions {
    gap: 8px;
    margin-top: 15px;
  }

  .focus-action-btn {
    padding: 8px 12px;
    font-size: 0.8rem;
    min-width: 70px;
  }
}

@media (max-height: 600px) {
  .focus-mode-container {
    max-height: 95vh;
    padding: 15px;
  }

  .focus-title {
    font-size: 1.4rem;
    margin-bottom: 6px;
  }

  .focus-notes {
    font-size: 0.8rem;
    margin-bottom: 10px;
  }

  .timer-display {
    font-size: 2.2rem;
  }

  .focus-timer {
    margin: 15px 0;
  }

  .focus-indicator {
    margin: 10px 0;
  }

  .focus-actions {
    margin-top: 10px;
  }
}

/* 原有样式保持不变 */
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
  opacity: 0.65;
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
  overflow-y: auto;
  scrollbar-width: none;
  padding-top: 18px;
}

.list-container {
  padding: 5px 20px;
  overflow-y: auto;
  height: 100%;
  /* 优化滚动条样式 */
  scrollbar-width: none;
  scrollbar-color: rgba(155, 155, 155, 0.5) transparent;
  padding-top: 46px;
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
