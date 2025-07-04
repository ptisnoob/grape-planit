<template>
  <div class="todo-settings">
    <div class="settings-section">
      <h3 class="section-title">优先级颜色设置</h3>
      <p class="section-description">自定义待办事项优先级的颜色显示</p>
      
      <div class="color-settings">
        <div v-for="level in levelConfigs" :key="level.key" class="color-item">
          <div class="color-item-header">
            <span class="level-name">{{ level.name }}</span>
            <div class="color-preview" :style="{ backgroundColor: level.color }" @click="openColorPicker(level.key)"></div>
          </div>
          <div class="color-item-description">{{ level.description }}</div>
          
          <!-- 颜色选择器 -->
          <input 
            v-if="activeColorPicker === level.key"
            type="color" 
            :value="level.color" 
            @input="(e) => e?.target && updateColor(level.key, (e.target as HTMLInputElement).value)"
            @blur="closeColorPicker"
            class="color-picker"
            ref="colorInput"
          />
        </div>
      </div>
      
      <div class="action-buttons">
        <button @click="resetToDefault" class="btn btn-secondary">恢复默认</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// 优先级配置
const levelConfigs = reactive([
  {
    key: 'level-important-urgent',
    name: '重要且紧急',
    description: '需要立即处理的重要事项',
    color: '#ff4757'
  },
  {
    key: 'level-important-not-urgent', 
    name: '重要不紧急',
    description: '重要但可以稍后处理的事项',
    color: '#ffa726'
  },
  {
    key: 'level-not-important-urgent',
    name: '不重要但紧急', 
    description: '紧急但不太重要的事项',
    color: '#ffca28'
  },
  {
    key: 'level-not-important-not-urgent',
    name: '不重要不紧急',
    description: '可以延后处理的一般事项', 
    color: '#66bb6a'
  },
  {
    key: 'level-uncategorized',
    name: '未分类',
    description: '尚未设置优先级的事项',
    color: '#bdbdbd'
  }
]);

// 默认颜色配置
const defaultColors = {
  'level-important-urgent': '#ff4757',
  'level-important-not-urgent': '#ffa726', 
  'level-not-important-urgent': '#ffca28',
  'level-not-important-not-urgent': '#66bb6a',
  'level-uncategorized': '#bdbdbd'
};

const activeColorPicker = ref<string | null>(null);
const colorInput = ref<HTMLInputElement[]>([]);

// 打开颜色选择器
const openColorPicker = async (levelKey: string) => {
  activeColorPicker.value = levelKey;
  await nextTick();
  // 自动聚焦到颜色选择器
  const input = colorInput.value?.[0];
  if (input) {
    input.focus();
    input.click();
  }
};

// 关闭颜色选择器
const closeColorPicker = () => {
  activeColorPicker.value = null;
};

// 更新颜色
const updateColor = async (levelKey: string, color: string) => {
  const level = levelConfigs.find(l => l.key === levelKey);
  if (level) {
    level.color = color;
    updateCSSVariables();
    await saveSettings();
  }
};

// 恢复默认颜色
const resetToDefault = async () => {
  levelConfigs.forEach(level => {
    level.color = defaultColors[level.key as keyof typeof defaultColors];
  });
  updateCSSVariables();
  await saveSettings();
};

// 保存设置
const saveSettings = async () => {
  try {
    const colorSettings = levelConfigs.reduce((acc, level) => {
      acc[level.key] = level.color;
      return acc;
    }, {} as Record<string, string>);
    
    await invoke('save_todo_color_settings', { colors: colorSettings });
    
    // 更新CSS变量
    updateCSSVariables();
    
    // 应用颜色到主窗口
    await applyColorsToMainWindow(colorSettings);
    
    console.log('TODO颜色设置已保存');
  } catch (error) {
    console.error('保存TODO颜色设置失败:', error);
  }
};

// 加载设置
const loadSettings = async () => {
  try {
    const settings = await invoke('load_todo_color_settings');
    if (settings) {
      const colorSettings = settings as Record<string, string>;
      levelConfigs.forEach(level => {
        if (colorSettings[level.key]) {
          level.color = colorSettings[level.key];
        }
      });
      updateCSSVariables();
      console.log('TODO颜色设置已加载:', colorSettings);
    }
  } catch (error) {
    console.error('加载TODO颜色设置失败:', error);
    // 如果加载失败，使用默认颜色并更新CSS变量
    updateCSSVariables();
  }
};

// 更新CSS变量
const updateCSSVariables = () => {
  const root = document.documentElement;
  levelConfigs.forEach(level => {
    root.style.setProperty(`--${level.key}-color`, level.color);
  });
};

// 应用颜色到主窗口
const applyColorsToMainWindow = async (colors: Record<string, string>) => {
  try {
    await invoke('apply_todo_colors_to_main_window', { colors });
  } catch (error) {
    console.error('应用颜色到主窗口失败:', error);
  }
};

onMounted(() => {
  loadSettings();
});
</script>

<style lang="scss" scoped>
.todo-settings {
  max-width: 600px;
}

.settings-section {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 20px;
  border: 1px solid var(--border-color);
  transition: all var(--transition-normal);
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.section-description {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0 0 24px 0;
  line-height: 1.5;
}

.color-settings {
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-bottom: 24px;
}

.color-item {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 16px;
  border: 1px solid var(--border-color);
  transition: all var(--transition-fast);
  
  &:hover {
    border-color: var(--accent-color);
    transform: translateY(-1px);
    box-shadow: 0 2px 8px var(--shadow);
  }
}

.color-item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.level-name {
  font-size: 16px;
  font-weight: 500;
  color: var(--text-primary);
}

.color-preview {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid var(--border-color);
  transition: all var(--transition-fast);
  
  &:hover {
    transform: scale(1.1);
    border-color: var(--accent-color);
  }
}

.color-item-description {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.4;
}

.color-picker {
  position: absolute;
  opacity: 0;
  pointer-events: none;
}

.action-buttons {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.btn {
  padding: 10px 20px;
  border-radius: 8px;
  border: none;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  
  &:hover {
    transform: translateY(-1px);
  }
  
  &:active {
    transform: translateY(0);
  }
}

.btn-primary {
  background: var(--accent-color);
  color: white;
  
  &:hover {
    background: var(--accent-color-hover, var(--accent-color));
    box-shadow: 0 4px 12px var(--shadow);
  }
}

.btn-secondary {
  background: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  
  &:hover {
    background: var(--bg-secondary);
    border-color: var(--accent-color);
  }
}
</style>