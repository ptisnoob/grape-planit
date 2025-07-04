<template>
    <div class="settings-container settings-page theme-transition">
        <!-- 左侧导航菜单 -->
        <div class="settings-sidebar">
            <div class="sidebar-content">
                <div class="menu-items">
                    <div v-for="item in menuItems" :key="item.key"
                        :class="['menu-item', { active: activeMenu === item.key }]" @click="setActiveMenu(item.key)">
                        {{ item.label }}
                    </div>
                </div>

                <!-- 版本信息 -->
                <div class="version-info">
                    v1.2.3
                </div>
            </div>
        </div>

        <!-- 右侧内容区域 -->
        <div class="settings-content">
            <div class="content-area">
                <!-- 根据激活菜单显示对应组件 -->
                <GeneralSettings v-if="activeMenu === 'general'" />
                <TimeSettings v-else-if="activeMenu === 'time'" />
                <TodoSettings v-else-if="activeMenu === 'todo'" />
                <AISettings v-else-if="activeMenu === 'ai'" />
                
                <!-- 其他菜单的空内容 -->
                <div v-else class="empty-content">
                    <p>{{ currentMenuLabel }}内容将在这里显示...</p>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { useTheme } from '@/composables/useTheme';
import GeneralSettings from './components/GeneralSettings.vue';
import TimeSettings from './components/TimeSettings.vue';
import AISettings from './components/AISettings.vue';
import TodoSettings from './components/TodoSettings.vue';

const { initTheme } = useTheme();

// 菜单项配置
const menuItems = ref([
    { key: 'general', label: '通用' },
    { key: 'time', label: '时间' },
    { key: 'todo', label: '待办' },
    { key: 'weather', label: '天气' },
    { key: 'ai', label: 'AI' },
    { key: 'shortcuts', label: '快捷键' }
]);

// 当前激活的菜单
const activeMenu = ref('general');

// 设置激活菜单
const setActiveMenu = (key: string) => {
    activeMenu.value = key;
};

// 当前菜单标签
const currentMenuLabel = computed(() => {
    const item = menuItems.value.find(item => item.key === activeMenu.value);
    return item ? item.label : '设置';
});

onMounted(() => {
    initTheme();
});
</script>

<style lang="scss" scoped>
.settings-container {
    display: flex;
    width: 100%;
    height: 100vh;
    background: var(--bg-primary);
    color: var(--text-primary);
    overflow: hidden;
    transition: all var(--transition-normal);
}

.settings-sidebar {
    width: 110px;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    transition: all var(--transition-normal);

    .sidebar-content {
        display: flex;
        flex-direction: column;
        height: 100%;
        padding: 10px 0;
    }

    .menu-items {
        flex: 1;
        padding: 0 10px;
    }

    .menu-item {
        padding: 8px 6px;
        margin: 2px 0;
        border-radius: 8px;
        text-align: center;
        cursor: pointer;
        font-size: 14px;
        color: var(--text-secondary);
        transition: all var(--transition-fast);

        &:hover {
            background: var(--bg-primary);
            color: var(--text-primary);
            transform: translateY(-1px);
        }

        &.active {
            background: var(--accent-color);
            color: white;
            box-shadow: 0 2px 8px var(--shadow);
        }
    }

    .version-info {
        padding: 16px;
        text-align: center;
        font-size: 12px;
        color: var(--text-secondary);
        border-top: 1px solid var(--border-color);
        opacity: 0.7;
    }
}

.settings-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: var(--bg-primary);
    transition: all var(--transition-normal);

    .content-area {
        flex: 1;
        padding: 20px;
        overflow-y: auto;
        
        /* 自定义滚动条样式 */
        &::-webkit-scrollbar {
            width: 8px;
        }
        
        &::-webkit-scrollbar-track {
            background: var(--bg-secondary);
            border-radius: 4px;
        }
        
        &::-webkit-scrollbar-thumb {
            background: var(--border-color);
            border-radius: 4px;
            transition: background var(--transition-fast);
        }
        
        &::-webkit-scrollbar-thumb:hover {
            background: var(--accent-color);
        }
        
        /* Firefox 滚动条样式 */
        scrollbar-width: thin;
        scrollbar-color: var(--border-color) var(--bg-secondary);
    }

    .empty-content {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100%;

        p {
            font-size: 16px;
            color: var(--text-secondary);
            text-align: center;
            opacity: 0.8;
        }
    }
}


</style>