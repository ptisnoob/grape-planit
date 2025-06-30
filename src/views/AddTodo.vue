<template>
    <div class="add-todo-view">
        <form @submit.prevent="saveTodo" class="todo-form">
            <div class="form-group">
                <label for="title">事项名称</label>
                <input id="title" v-model="todo.title" type="text" required>
            </div>

            <div class="form-group">
                <label for="due_date">截止时间</label>
                <input id="due_date" v-model="todo.due_date" type="datetime-local" required>
            </div>

            <div class="form-group">
                <label>类型</label>
                <div class="radio-group">
                    <label v-for="(item, index) in levelOptions" :key="index">
                        <input v-model="todo.level" :value="index" type="radio" name="level">
                        <span>{{ item }}</span>
                    </label>
                </div>
            </div>

            <div class="form-group">
                <label>周期</label>
                <div class="radio-group">
                    <label v-for="item in cycleOptions" :key="item.value">
                        <input v-model="todo.cycle" :value="item.value" type="radio" name="cycle">
                        <span>{{ item.label }}</span>
                    </label>
                </div>
            </div>

            <div class="form-group">
                <label for="notes">备注</label>
                <textarea id="notes" v-model="todo.notes"></textarea>
            </div>

            <div class="form-actions">
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
    due_date: '',
    notes: '',
    level: 0, // 默认重要不紧急
    cycle: 'one', // 默认仅一次
});

const levelOptions = ['重要不紧急', '重要且紧急', '不重要不紧急', '不重要但紧急'];
const cycleOptions = [
    { label: '仅一次', value: 'one' },
    { label: '每日', value: 'day' },
    { label: '每周', value: 'week' },
    { label: '每月', value: 'month' },
    { label: '每年', value: 'year' },
];

const saveTodo = async () => {
    if (!todo.value.title || !todo.value.due_date) {
        alert('请填写事项名称和截止时间');
        return;
    }

    try {
        await invoke('add_todo', {
            params: {
                title: todo.value.title,
                due_date: todo.value.due_date,
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
    padding: 20px;
    background: var(--bg-primary);
    color: var(--text-primary);
    height: 100%;
    overflow-y: auto;
}

.todo-form {
    display: flex;
    flex-direction: column;
    gap: 20px;
    max-width: 500px;
    margin: 0 auto;
}

.form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

label {
    font-weight: bold;
    font-size: 14px;
}

input[type="text"],
input[type="datetime-local"],
textarea {
    width: 100%;
    padding: 10px;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 14px;
}

textarea {
    min-height: 80px;
    resize: vertical;
}

.radio-group {
    display: flex;
    flex-wrap: wrap;
    gap: 15px;
}

.radio-group label {
    display: flex;
    align-items: center;
    gap: 5px;
    cursor: pointer;
    font-weight: normal;
}

.form-actions {
    display: flex;
    justify-content: center;
    margin-top: 20px;
}

.save-btn {
    padding: 12px 30px;
    border: none;
    border-radius: 8px;
    background: var(--accent-color);
    color: white;
    font-size: 16px;
    font-weight: bold;
    cursor: pointer;
    transition: background 0.3s;
}

.save-btn:hover {
    background: var(--accent-color-hover);
}
</style>