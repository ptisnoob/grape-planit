<template>
  <div class="calendar-view">
    <div class="calendar-container">
      <div class="calendar-header">
        <h2>日历视图</h2>
        <p>按时间线显示待办事项</p>
      </div>
      
      <div class="calendar-content">
        <div class="timeline">
          <div v-for="day in timelineDays" :key="day.date" class="timeline-day"
            :class="{ 'is-today': day.isToday, 'is-past': day.isPast }">
            <div class="day-header">
              <div class="day-date">{{ day.dateText }}</div>
              <div class="day-name">{{ day.dayName }}</div>
              <div class="day-count" v-if="day.items.length > 0">({{ day.items.length }})</div>
            </div>
            
            <div class="day-items">
              <div v-if="day.items.length > 0" class="items-list">
                <div v-for="item in day.items" :key="item.id" class="timeline-item"
                  :class="getLevelClass(item.level)"
                  @click="handleItemClick(item)" @dblclick="enterFocusMode(item)">
                  <div class="item-content">
                    <div class="item-title">{{ item.title }}</div>
                    <div class="item-time" v-if="item.startTime">
                      {{ formatTime(item.startTime) }}
                      <span v-if="item.endTime && item.endTime !== item.startTime">
                        - {{ formatTime(item.endTime) }}
                      </span>
                    </div>
                  </div>
                  <div class="level-indicator" :class="getLevelClass(item.level)"></div>
                </div>
              </div>
              <div v-else class="empty-day">
                <span>无事项</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { Todo } from '@/model/todo';
import { GDate } from "@/common/date";

interface Props {
  list: Todo[];
}

interface Emits {
  (e: 'enterFocusMode', todo: Todo): void;
  (e: 'itemClick', todo: Todo): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// 生成时间线数据
const timelineDays = computed(() => {
  const days = [];
  const today = new GDate();
  
  // 生成过去3天到未来7天的时间线
  for (let i = -3; i <= 7; i++) {
    const date = new GDate(today.getTime() + i * 24 * 60 * 60 * 1000);
    const dateStart = date.getStartOfDay();
    const dateEnd = new GDate(dateStart.getTime() + 24 * 60 * 60 * 1000 - 1);
    
    // 筛选当天的待办事项
    const dayItems = props.list.filter(item => {
      const itemTime = item.startTime || item.endTime;
      if (!itemTime) return false;
      
      const itemDate = new GDate(itemTime);
      return itemDate.getTime() >= dateStart.getTime() && itemDate.getTime() <= dateEnd.getTime();
    });
    
    days.push({
      date: date.format('YYYY-MM-DD'),
      dateText: date.format('MM/DD'),
      dayName: getDayName(date.getDay()),
      isToday: i === 0,
      isPast: i < 0,
      items: dayItems.sort((a, b) => {
        const timeA = a.startTime || a.endTime || 0;
        const timeB = b.startTime || b.endTime || 0;
        return timeA - timeB;
      })
    });
  }
  
  return days;
});

// 获取星期名称
const getDayName = (dayIndex: number) => {
  const days = ['周日', '周一', '周二', '周三', '周四', '周五', '周六'];
  return days[dayIndex];
};

// 格式化时间
const formatTime = (timestamp: number) => {
  const date = new GDate(timestamp);
  return date.format('HH:mm');
};

// 事件处理
const handleItemClick = (item: Todo) => {
  emit('itemClick', item);
};

const enterFocusMode = (item: Todo) => {
  emit('enterFocusMode', item);
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
</script>

<style scoped>
.calendar-view {
  flex: 1;
  overflow-y: auto;
  padding: 0 10px;
}

.calendar-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.calendar-header {
  text-align: center;
  margin-bottom: 8px;
}

.calendar-header h2 {
  color: var(--text-primary);
  font-size: 18px;
  margin: 0 0 4px 0;
}

.calendar-header p {
  color: var(--text-secondary);
  font-size: 12px;
  margin: 0;
}

.timeline {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.timeline-day {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 10px;
  border: 1px solid var(--border-color);
  transition: all var(--transition-fast);
  backdrop-filter: blur(10px);
}

.timeline-day:hover {
  background: var(--bg-primary);
  box-shadow: 0 2px 8px var(--shadow);
}

.timeline-day.is-today {
  background: var(--bg-primary);
  border-color: var(--accent-color);
  box-shadow: 0 0 10px var(--shadow);
}

.timeline-day.is-past {
  opacity: 0.7;
}

.day-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
  padding-bottom: 6px;
  border-bottom: 1px solid var(--border-color);
}

.day-date {
  color: var(--text-primary);
  font-weight: 600;
  font-size: 14px;
}

.day-name {
  color: var(--text-secondary);
  font-size: 12px;
}

.day-count {
  color: var(--text-secondary);
  font-size: 11px;
  margin-left: auto;
  opacity: 0.8;
}

.is-today .day-date {
  color: #48dbfb;
}

.is-today .day-name {
  color: #48dbfb;
}

.day-items {
  min-height: 30px;
}

.items-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.timeline-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: var(--bg-primary);
  border-radius: 6px;
  padding: 6px 8px;
  cursor: pointer;
  transition: all var(--transition-fast);
  border-left: 3px solid transparent;
  border: 1px solid var(--border-color);
  backdrop-filter: blur(5px);
}

.timeline-item:hover {
  background: var(--bg-secondary);
  transform: translateX(2px);
  box-shadow: 0 2px 6px var(--shadow);
}

.timeline-item.level-important-urgent {
  border-left-color: #ff4757;
}

.timeline-item.level-important-not-urgent {
  border-left-color: #ffa502;
}

.timeline-item.level-not-important-urgent {
  border-left-color: #3742fa;
}

.timeline-item.level-not-important-not-urgent {
  border-left-color: #2ed573;
}

.timeline-item.level-uncategorized {
  border-left-color: #747d8c;
}

.item-content {
  flex: 1;
  min-width: 0;
}

.item-title {
  color: var(--text-primary);
  font-size: 12px;
  font-weight: 500;
  margin-bottom: 2px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.item-time {
  color: var(--text-secondary);
  font-size: 10px;
  opacity: 0.8;
}

.level-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.level-indicator.level-important-urgent {
  background: #ff4757;
}

.level-indicator.level-important-not-urgent {
  background: #ffa502;
}

.level-indicator.level-not-important-urgent {
  background: #3742fa;
}

.level-indicator.level-not-important-not-urgent {
  background: #2ed573;
}

.level-indicator.level-uncategorized {
  background: #747d8c;
}

.empty-day {
  text-align: center;
  padding: 12px;
  color: var(--text-secondary);
  font-size: 11px;
  opacity: 0.6;
}
</style>