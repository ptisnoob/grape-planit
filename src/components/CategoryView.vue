<template>
  <div class="category-view">
    <div class="category-container">
      <!-- Canvas坐标系 -->
      <div class="canvas-container" ref="canvasContainer">
        <canvas ref="quadrantCanvas" @click="handleCanvasClick" @dblclick="handleCanvasDoubleClick"
          @mousemove="handleCanvasMouseMove" @mouseleave="handleCanvasMouseLeave" @contextmenu="handleCanvasContextMenu"></canvas>

        <!-- 悬浮提示 -->
        <div v-if="hoveredItem" class="item-tooltip"
          :style="{ left: tooltipPosition.x + 'px', top: tooltipPosition.y + 'px' }">
          <div class="tooltip-title">{{ hoveredItem.title }}</div>
          <div class="tooltip-date" :class="getDueDateClass(hoveredItem)">{{ getDueDateText(hoveredItem) }}</div>
        </div>

        <!-- 象限内图例 -->
        <div class="quadrant-legends">
          <!-- 左上象限图例 -->
          <div class="quadrant-legend quadrant-legend-0">
            <div class="legend-dot quadrant-2-color"></div>
            <span>重要不紧急 ({{ getCategoryCount(0) }})</span>
          </div>

          <!-- 右上象限图例 -->
          <div class="quadrant-legend quadrant-legend-1">
            <div class="legend-dot quadrant-1-color"></div>
            <span>重要且紧急 ({{ getCategoryCount(1) }})</span>
          </div>

          <!-- 左下象限图例 -->
          <div class="quadrant-legend quadrant-legend-2">
            <div class="legend-dot quadrant-4-color"></div>
            <span>不重要不紧急 ({{ getCategoryCount(2) }})</span>
          </div>

          <!-- 右下象限图例 -->
          <div class="quadrant-legend quadrant-legend-3">
            <div class="legend-dot quadrant-3-color"></div>
            <span>不重要但紧急 ({{ getCategoryCount(3) }})</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue';
import { Todo } from '@/model/todo';
import { GDate } from "@/common/date";

interface Props {
  list: Todo[];
}

interface Emits {
  (e: 'enterFocusMode', todo: Todo): void;
  (e: 'itemClick', todo: Todo): void;
  (e: 'contextMenu', event: MouseEvent, todo: Todo, index: number): void;
}

interface ItemPosition {
  item: Todo;
  x: number;
  y: number;
  radius: number;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// Canvas相关引用
const canvasContainer = ref<HTMLDivElement>();
const quadrantCanvas = ref<HTMLCanvasElement>();
let ctx: CanvasRenderingContext2D | null = null;
let animationId: number | null = null;

// 交互状态
const hoveredItem = ref<Todo | null>(null);
const tooltipPosition = ref({ x: 0, y: 0 });
const itemPositions = ref<ItemPosition[]>([]);

// 画布尺寸
const canvasWidth = ref(0);
const canvasHeight = ref(0);

// 颜色配置
const quadrantColors = {
  1: { bg: 'rgba(255, 71, 87, 0.1)', border: '#ff4757', dot: '#ff4757' },
  0: { bg: 'rgba(255, 165, 2, 0.1)', border: '#ffa502', dot: '#ffa502' },
  3: { bg: 'rgba(55, 66, 250, 0.1)', border: '#3742fa', dot: '#3742fa' },
  2: { bg: 'rgba(46, 213, 115, 0.1)', border: '#2ed573', dot: '#2ed573' }
};

// 获取指定分类的待办事项
const getCategoryItems = (level: number) => {
  return props.list.filter(item => item.level === level);
};

// 获取指定分类的数量
const getCategoryCount = (level: number) => {
  return getCategoryItems(level).length;
};

// 事件处理
const handleItemClick = (item: Todo) => {
  emit('itemClick', item);
};

const enterFocusMode = (item: Todo) => {
  emit('enterFocusMode', item);
};

// 初始化Canvas
const initCanvas = () => {
  if (!quadrantCanvas.value || !canvasContainer.value) return;

  ctx = quadrantCanvas.value.getContext('2d');
  if (!ctx) return;

  updateCanvasSize();
  drawQuadrants();
  positionItems();
};

// 更新Canvas尺寸
const updateCanvasSize = () => {
  if (!quadrantCanvas.value || !canvasContainer.value) return;

  const rect = canvasContainer.value.getBoundingClientRect();
  const dpr = window.devicePixelRatio || 1;

  canvasWidth.value = rect.width;
  canvasHeight.value = rect.height;

  quadrantCanvas.value.width = rect.width * dpr;
  quadrantCanvas.value.height = rect.height * dpr;
  quadrantCanvas.value.style.width = rect.width + 'px';
  quadrantCanvas.value.style.height = rect.height + 'px';

  if (ctx) {
    ctx.scale(dpr, dpr);
  }
};

// 绘制象限
const drawQuadrants = () => {
  if (!ctx) return;

  const width = canvasWidth.value;
  const height = canvasHeight.value;
  const centerX = width / 2;
  const centerY = height / 2;

  // 清空画布
  ctx.clearRect(0, 0, width, height);

  // 绘制象限背景
  const quadrants = [
    { level: 1, x: centerX, y: 0, w: centerX, h: centerY }, // 右上
    { level: 0, x: 0, y: 0, w: centerX, h: centerY }, // 左上
    { level: 2, x: 0, y: centerY, w: centerX, h: centerY }, // 左下
    { level: 3, x: centerX, y: centerY, w: centerX, h: centerY } // 右下
  ];

  quadrants.forEach(quad => {
    const color = quadrantColors[quad.level as keyof typeof quadrantColors];
    ctx!.fillStyle = color.bg;
    ctx?.fillRect(quad.x, quad.y, quad.w, quad.h);
  });

  // 绘制坐标轴
  ctx.strokeStyle = 'var(--border-color)';
  ctx.lineWidth = 2;
  ctx.beginPath();
  // 垂直线
  ctx.moveTo(centerX, 0);
  ctx.lineTo(centerX, height);
  // 水平线
  ctx.moveTo(0, centerY);
  ctx.lineTo(width, centerY);
  ctx.stroke();

  // 绘制轴标签
  ctx.fillStyle = 'var(--text-secondary)';
  ctx.font = '11px sans-serif';
  ctx.textAlign = 'center';

  // 重要性轴（垂直）
  ctx.save();
  ctx.translate(8, centerY);
  ctx.rotate(-Math.PI / 2);
  ctx.fillText('重要性', 0, 0);
  ctx.restore();

  // 紧急性轴（水平）
  ctx.fillText('紧急性', centerX, height - 5);

  // 添加轴的箭头指示
  ctx.fillStyle = 'var(--text-secondary)';
  ctx.font = '10px sans-serif';

  // 重要性轴箭头和标识
  ctx.save();
  ctx.translate(20, 15);
  ctx.rotate(-Math.PI / 2);
  ctx.fillText('高', 0, 0);
  ctx.restore();

  ctx.save();
  ctx.translate(20, height - 15);
  ctx.rotate(-Math.PI / 2);
  ctx.fillText('低', 0, 0);
  ctx.restore();

  // 紧急性轴箭头和标识
  ctx.textAlign = 'left';
  ctx.fillText('低', 15, centerY - 5);
  ctx.textAlign = 'right';
  ctx.fillText('高', width - 15, centerY - 5);
};

// 定位待办事项
const positionItems = () => {
  if (!ctx) return;

  const width = canvasWidth.value;
  const height = canvasHeight.value;
  const centerX = width / 2;
  const centerY = height / 2;

  itemPositions.value = [];

  // 为每个象限的项目分配位置
  [0, 1, 2, 3].forEach(level => {
    const items = getCategoryItems(level);
    if (items.length === 0) return;

    const quadrantBounds = getQuadrantBounds(level, centerX, centerY);
    const positions = generateItemPositions(items, quadrantBounds);

    positions.forEach(pos => {
      itemPositions.value.push(pos);
    });
  });

  drawItems();
};

// 获取象限边界
const getQuadrantBounds = (level: number, centerX: number, centerY: number) => {
  const padding = 60; // 增加padding以缩小象限可用区域

  switch (level) {
    case 1: // 重要且紧急（右上）
      return {
        x: centerX + padding,
        y: padding,
        width: centerX - padding * 2,
        height: centerY - padding * 2
      };
    case 0: // 重要不紧急（左上）
      return {
        x: padding,
        y: padding,
        width: centerX - padding * 2,
        height: centerY - padding * 2
      };
    case 2: // 不重要不紧急（左下）
      return {
        x: padding,
        y: centerY + padding,
        width: centerX - padding * 2,
        height: centerY - padding * 2
      };
    case 3: // 不重要但紧急（右下）
      return {
        x: centerX + padding,
        y: centerY + padding,
        width: centerX - padding * 2,
        height: centerY - padding * 2
      };
    default:
      return { x: 0, y: 0, width: 0, height: 0 };
  }
};

// 生成项目位置
const generateItemPositions = (items: Todo[], bounds: any): ItemPosition[] => {
  const positions: ItemPosition[] = [];
  const radius = 8;
  const minDistance = radius * 3;

  items.forEach((item) => {
    let x: number, y: number;
    let attempts: number = 0;
    const maxAttempts = 50;

    do {
      x = bounds.x + Math.random() * bounds.width;
      y = bounds.y + Math.random() * bounds.height;
      attempts++;
    } while (
      attempts < maxAttempts &&
      positions.some(pos =>
        Math.sqrt((pos.x - x) ** 2 + (pos.y - y) ** 2) < minDistance
      )
    );

    positions.push({ item, x, y, radius });
  });

  return positions;
};

// 绘制待办事项
const drawItems = () => {
  if (!ctx) return;

  itemPositions.value.forEach(pos => {
    const color = quadrantColors[pos.item.level as keyof typeof quadrantColors];

    // 绘制圆点
    ctx!.beginPath();
    ctx!.arc(pos.x, pos.y, pos.radius, 0, 2 * Math.PI);
    ctx!.fillStyle = color.dot;
    ctx!.fill();

    // 如果是悬停项目，添加高亮效果
    if (hoveredItem.value?.id === pos.item.id) {
      ctx!.strokeStyle = color.border;
      ctx!.lineWidth = 3;
      ctx!.stroke();

      // 绘制外圈
      ctx!.beginPath();
      ctx!.arc(pos.x, pos.y, pos.radius + 5, 0, 2 * Math.PI);
      ctx!.strokeStyle = color.border;
      ctx!.lineWidth = 1;
      ctx!.stroke();
    }
  });
};

// 处理Canvas鼠标移动
const handleCanvasMouseMove = (event: MouseEvent) => {
  if (!quadrantCanvas.value) return;

  const rect = quadrantCanvas.value.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  // 检查是否悬停在某个项目上
  const hoveredPosition = itemPositions.value.find(pos => {
    const distance = Math.sqrt((pos.x - x) ** 2 + (pos.y - y) ** 2);
    return distance <= pos.radius + 5;
  });

  if (hoveredPosition) {
    hoveredItem.value = hoveredPosition.item;
    tooltipPosition.value = { x: event.clientX, y: event.clientY };
    quadrantCanvas.value.style.cursor = 'pointer';
  } else {
    hoveredItem.value = null;
    quadrantCanvas.value.style.cursor = 'default';
  }

  // 重绘以更新高亮效果
  drawQuadrants();
  drawItems();
};

// 处理Canvas鼠标点击
const handleCanvasClick = (event: MouseEvent) => {
  if (!quadrantCanvas.value) return;

  const rect = quadrantCanvas.value.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  const clickedPosition = itemPositions.value.find(pos => {
    const distance = Math.sqrt((pos.x - x) ** 2 + (pos.y - y) ** 2);
    return distance <= pos.radius + 5;
  });

  if (clickedPosition) {
    handleItemClick(clickedPosition.item);
  }
};

// 处理Canvas双击
const handleCanvasDoubleClick = (event: MouseEvent) => {
  if (!quadrantCanvas.value) return;

  const rect = quadrantCanvas.value.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  const clickedPosition = itemPositions.value.find(pos => {
    const distance = Math.sqrt((pos.x - x) ** 2 + (pos.y - y) ** 2);
    return distance <= pos.radius + 5;
  });

  if (clickedPosition) {
    enterFocusMode(clickedPosition.item);
  }
};

// 处理Canvas右键菜单
const handleCanvasContextMenu = (event: MouseEvent) => {
  event.preventDefault();
  if (!quadrantCanvas.value) return;

  const rect = quadrantCanvas.value.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  const clickedPosition = itemPositions.value.find(pos => {
    const distance = Math.sqrt((pos.x - x) ** 2 + (pos.y - y) ** 2);
    return distance <= pos.radius + 5;
  });

  if (clickedPosition) {
    const index = props.list.findIndex(item => item.id === clickedPosition.item.id);
    emit('contextMenu', event, clickedPosition.item, index);
  }
};

// 处理Canvas鼠标离开
const handleCanvasMouseLeave = () => {
  hoveredItem.value = null;
  if (quadrantCanvas.value) {
    quadrantCanvas.value.style.cursor = 'default';
  }
  drawQuadrants();
  drawItems();
};

// 处理窗口大小变化
const handleResize = () => {
  updateCanvasSize();
  drawQuadrants();
  positionItems();
};

// 生命周期钩子
onMounted(() => {
  nextTick(() => {
    initCanvas();
    window.addEventListener('resize', handleResize);
  });
});

onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
  if (animationId) {
    cancelAnimationFrame(animationId);
  }
});

// 监听数据变化
watch(() => props.list, () => {
  nextTick(() => {
    positionItems();
  });
}, { deep: true });

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
    return '今天截止';
  } else if (diffDays === 1) {
    return '明天截止';
  } else if (diffDays <= 3) {
    return `${diffDays} 天后`;
  } else {
    return `${diffDays} 天后`;
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
  } else {
    return 'due-date-normal';
  }
};
</script>

<style scoped>
.category-view {
  height: calc(100vh - 36px);
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}

.category-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 8px;
  overflow: hidden;
}

/* Canvas容器 */
.canvas-container {
  position: relative;
  flex: 1;
  height: 100%;
  background: var(--bg-secondary);
  border-radius: 12px;
  border: 2px solid var(--border-color);
  overflow: hidden;
}

.canvas-container canvas {
  width: 100%;
  height: 100%;
  display: block;
}

/* 悬浮提示 */
.item-tooltip {
  position: fixed;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 8px 12px;
  box-shadow: 0 4px 12px var(--shadow);
  z-index: 1000;
  pointer-events: none;
  backdrop-filter: blur(10px);
  transform: translate(-50%, -100%);
  margin-top: -8px;
}

.tooltip-title {
  color: var(--text-primary);
  font-size: 12px;
  font-weight: 600;
  margin-bottom: 4px;
}

.tooltip-date {
  font-size: 10px;
}

/* 象限颜色定义 */
.quadrant-1-color {
  background: #ff4757;
  border-color: #ff4757;
}

.quadrant-2-color {
  background: #ffa502;
  border-color: #ffa502;
}

.quadrant-3-color {
  background: #3742fa;
  border-color: #3742fa;
}

.quadrant-4-color {
  background: #2ed573;
  border-color: #2ed573;
}

/* 象限内图例 */
.quadrant-legends {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 10;
}

.quadrant-legend {
  position: absolute;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 11px;
  color: var(--text-secondary);
  backdrop-filter: blur(5px);
  box-shadow: 0 2px 8px var(--shadow);
  opacity: 0.95;
}

.quadrant-legend-0 {
  top: 10px;
  left: 10px;
}

.quadrant-legend-1 {
  top: 10px;
  right: 10px;
}

.quadrant-legend-2 {
  bottom: 10px;
  left: 10px;
}

.quadrant-legend-3 {
  bottom: 10px;
  right: 10px;
}

.legend-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
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

.due-date-normal {
  color: var(--text-secondary);
}

.due-date-none {
  color: var(--text-secondary);
  opacity: 0.7;
}

/* 日期状态样式 */
.overdue {
  color: #ff4757;
  font-weight: 600;
}

.urgent {
  color: #ffa502;
  font-weight: 600;
}

.warning {
  color: #ff6b35;
  font-weight: 500;
}

.normal {
  color: var(--text-secondary);
  opacity: 0.7;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .quadrant-legend {
    font-size: 10px;
    padding: 6px 8px;
  }

  .legend-dot {
    width: 6px;
    height: 6px;
  }
}

@media (max-width: 480px) {
  .category-container {
    padding: 4px;
  }

  .quadrant-legend {
    font-size: 9px;
    padding: 4px 6px;
  }
}

/* 高度适配 */
@media (max-height: 600px) {
  .category-container {
    padding: 4px;
  }

  .quadrant-legend {
    font-size: 10px;
    padding: 4px 6px;
  }
}
</style>