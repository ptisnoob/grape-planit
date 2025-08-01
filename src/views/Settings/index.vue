<template>
    <div class="settings-container settings-page theme-transition">
        <!-- 左侧导航菜单 -->
        <div class="settings-sidebar">
            <div class="sidebar-content">
                <div class="menu-items">
                    <div v-for="item in menuItems" :key="item.value"
                        :class="['menu-item', { active: activeMenu === item.value }]"
                        @click="setActiveMenu(item.value)">
                        {{ item.label }}
                    </div>
                </div>

                <!-- 版本信息 -->
                <div class="version-info" @click="checkForUpdates" :class="{ 'checking': isCheckingUpdate }">
                    {{ appVersion }}
                    <span v-if="isCheckingUpdate" class="update-status">检查中...</span>
                    <span v-else-if="updateAvailable" class="update-status available">有新版本</span>
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
                <WeatherSettings v-else-if="activeMenu === 'weather'" />
                <HolidaySync v-else-if="activeMenu === 'holiday'" />
                <AISettings v-else-if="activeMenu === 'ai'" />
                <ShortcutSettings v-else-if="activeMenu === 'shortcuts'" />

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
import { getVersion } from '@tauri-apps/api/app';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import GeneralSettings from './components/GeneralSettings.vue';
import TimeSettings from './components/TimeSettings.vue';
import AISettings from './components/AISettings.vue';
import TodoSettings from './components/TodoSettings.vue';
import ShortcutSettings from './components/ShortcutSettings.vue';
import WeatherSettings from './components/WeatherSettings.vue';
import HolidaySync from './components/HolidaySync.vue';
import { SelOption } from "@/model/public"


const { initTheme } = useTheme();

// 应用版本号
const appVersion = ref('v1.0.0');
// 更新检查状态
const isCheckingUpdate = ref(false);
const updateAvailable = ref(false);

// 菜单项配置
const menuItems = ref<SelOption[]>([
    { value: 'general', label: '通用' },
    { value: 'time', label: '时间' },
    { value: 'todo', label: '待办' },
    { value: 'weather', label: '天气' },
    { value: 'holiday', label: '节假日' },
    { value: 'ai', label: 'AI' },
    { value: 'shortcuts', label: '快捷键' }
]);

// 当前激活的菜单
const activeMenu = ref('general');

// 设置激活菜单
const setActiveMenu = (key: string) => {
    activeMenu.value = key;
};

// 当前菜单标签
const currentMenuLabel = computed(() => {
    const item = menuItems.value.find(i => i.value === activeMenu.value);
    return item ? item.label : '设置';
});

// 检查更新
const checkForUpdates = async () => {
    if (isCheckingUpdate.value) return;
    
    isCheckingUpdate.value = true;
    updateAvailable.value = false;
    
    try {
        const update = await check();
        if (update?.available) {
            updateAvailable.value = true;
            const shouldUpdate = confirm(`发现新版本 ${update.version}！\n\n更新内容：\n${update.body || '修复问题和性能优化'}\n\n是否立即更新？`);
            
            if (shouldUpdate) {
                console.log('开始下载更新...');
                await update.downloadAndInstall();
                console.log('更新下载完成，准备重启应用...');
                await relaunch();
            }
        } else {
            alert('当前已是最新版本！');
        }
    } catch (error) {
        console.error('检查更新失败:', error);
        alert('检查更新失败，请稍后重试。');
    } finally {
        isCheckingUpdate.value = false;
    }
};

onMounted(async () => {
    initTheme();
    
    // 获取应用版本号
    try {
        const version = await getVersion();
        appVersion.value = `v${version}`;
    } catch (error) {
        console.warn('Failed to get app version:', error);
        // 保持默认版本号
    }
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
        cursor: pointer;
        transition: all var(--transition-fast);
        border-radius: 8px;
        margin: 8px;
        position: relative;

        &:hover {
            opacity: 1;
            background: var(--bg-primary);
            transform: translateY(-1px);
        }

        &.checking {
            opacity: 1;
            background: var(--accent-color);
            color: white;
        }

        .update-status {
            display: block;
            font-size: 10px;
            margin-top: 4px;
            
            &.available {
                color: #ff6b6b;
                font-weight: bold;
            }
        }
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