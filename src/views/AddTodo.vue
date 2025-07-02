<template>
    <div class="add-todo-view">
        <form @submit.prevent="saveTodo" class="todo-form">
            <div class="form-group">
                <input id="title" v-model="todo.title" type="text" placeholder="事项名称" required class="title-input">
            </div>

            <div class="form-group time-row">
                <input id="start_time" v-model="todo.start_time" type="datetime-local" required>
                <input id="end_time" v-model="todo.end_time" type="datetime-local" placeholder="结束时间（可选）">
            </div>

            <div class="form-group select-row">
                <select v-model="todo.level" required>
                    <option v-for="(item, index) in levelOptions" :key="index" :value="index">
                        {{ item }}
                    </option>
                </select>
                <select v-model="todo.cycle" required>
                    <option v-for="item in cycleOptions" :key="item.value" :value="item.value">
                        {{ item.label }}
                    </option>
                </select>
            </div>

            <div class="form-group">
                <textarea id="notes" v-model="todo.notes" placeholder="备注"></textarea>
            </div>

            <div class="form-actions">
                <button type="button" class="cancel-btn" @click="goBack">取消</button>
                <button type="submit" class="save-btn">保存</button>
            </div>
        </form>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';

const router = useRouter();

const todo = ref({
    title: '',
    start_time: '',
    end_time: '',
    notes: '',
    level: 0, // 默认重要不紧急
    cycle: 'one', // 默认仅一次
});

// 获取当前时间的ISO字符串（用于datetime-local输入框的默认值）
const getCurrentDateTimeLocal = () => {
    const now = new Date();
    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, '0');
    const day = String(now.getDate()).padStart(2, '0');
    const hours = String(now.getHours()).padStart(2, '0');
    const minutes = String(now.getMinutes()).padStart(2, '0');
    return `${year}-${month}-${day}T${hours}:${minutes}`;
};

// 将datetime-local字符串转换为时间戳（秒）
const dateTimeLocalToTimestamp = (dateTimeLocal: string): number => {
    return Math.floor(new Date(dateTimeLocal).getTime() / 1000);
};

// 设置默认开始时间为当前时间
todo.value.start_time = getCurrentDateTimeLocal();

const levelOptions = ['重要不紧急', '重要且紧急', '不重要不紧急', '不重要但紧急'];
const cycleOptions = [
    { label: '仅一次', value: 'one' },
    { label: '每日', value: 'day' },
    { label: '每周', value: 'week' },
    { label: '每月', value: 'month' },
    { label: '每年', value: 'year' },
];

const goBack = () => {
    router.back();
};

const saveTodo = async () => {
    if (!todo.value.title || !todo.value.start_time) {
        alert('请填写事项名称和开始时间');
        return;
    }

    try {
        // 转换时间为时间戳
        const startTimeTimestamp = dateTimeLocalToTimestamp(todo.value.start_time);
        const endTimeTimestamp = todo.value.end_time ? dateTimeLocalToTimestamp(todo.value.end_time) : null;

        // 验证结束时间不能早于开始时间
        if (endTimeTimestamp && endTimeTimestamp <= startTimeTimestamp) {
            alert('结束时间不能早于或等于开始时间');
            return;
        }

        await invoke('add_todo', {
            params: {
                title: todo.value.title,
                startTime: startTimeTimestamp,
                endTime: endTimeTimestamp,
                notes: todo.value.notes || null,
                level: todo.value.level,
                cycle: todo.value.cycle
            }
        });
        // 保存成功后可以跳转到列表页或显示成功信息
        router.push('/list');
    } catch (error) {
        console.error('Failed to save todo:', error);
        alert('保存失败，请查看控制台');
    }
};
</script>

<style scoped>
.add-todo-view {
    padding: 12px;
    background: var(--bg-primary);
    color: var(--text-primary);
    height: 100%;
    overflow: hidden;
    padding-top: 40px;
}

.todo-form {
    display: flex;
    flex-direction: column;
    gap: 8px;
    height: 100%;
}

.form-group {
    display: flex;
    flex-direction: column;
}

.time-row {
    flex-direction: row;
    gap: 8px;
}

.select-row {
    flex-direction: row;
    gap: 8px;
}

input[type="text"],
input[type="datetime-local"],
select,
textarea {
    padding: 6px 8px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 12px;
    flex: 1;
}

.title-input {
    height: 32px;
}

textarea {
    min-height: 40px;
    max-height: 40px;
    resize: none;
    flex: 1;
}

select {
    cursor: pointer;
}

.form-actions {
    display: flex;
    justify-content: center;
    gap: 12px;
    margin-top: auto;
}

.save-btn,
.cancel-btn {
    padding: 6px 16px;
    border: none;
    border-radius: 4px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s;
    min-width: 60px;
}

.save-btn {
    background: var(--accent-color);
    color: white;
}

.save-btn:hover {
    background: var(--accent-color-hover);
}

.cancel-btn {
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
}

.cancel-btn:hover {
    background: var(--bg-tertiary);
}
</style>