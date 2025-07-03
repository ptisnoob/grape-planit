<template>
    <div class="add-todo-view">
        <!-- AI辅助输入阶段 -->
        <div v-if="!showForm" class="ai-input-stage">
            <div class="ai-input-container">
                <h2 class="ai-title">AI智能填写</h2>
                <form @submit.prevent="handleAISubmit" class="ai-form">
                    <textarea v-model="aiInput"
                        placeholder="例如：明天下午3点开会讨论项目进度，会议室在A座201，需要准备PPT； 后天是妈妈生日，记得准备礼物：7号到14号是xx生理期，记得别惹她生气；"
                        class="ai-textarea" :disabled="isLoading" rows="4" required></textarea>
                    <div class="ai-actions">
                        <button type="button" class="skip-btn" @click="skipAI" :disabled="isLoading">手动填写</button>
                        <button type="submit" class="ai-submit-btn" :disabled="isLoading || !aiInput.trim()">
                            <span v-if="!isLoading">智能填写</span>
                            <span v-else class="loading-text">
                                <span class="loading-spinner"></span>
                                AI分析中...
                            </span>
                        </button>
                    </div>
                </form>
            </div>
        </div>

        <!-- 表单填写阶段 -->
        <div v-else class="form-stage" :class="{ 'animate-in': showForm }">
            <form @submit.prevent="saveTodo" class="todo-form">
                <div class="form-group">
                    <input id="title" v-model="todo.title" type="text" placeholder="事项名称" required class="title-input"
                        autocomplete="off">
                </div>

                <div class="form-group time-row">
                    <input id="start_time" v-model="todo.start_time" type="datetime-local" required autocomplete="off">
                    <input id="end_time" v-model="todo.end_time" type="datetime-local" placeholder="结束时间（可选）"
                        autocomplete="off">
                </div>

                <div class="form-group select-row">
                    <div class="select-group">
                        <div class="select-with-tip">
                            <select v-model="todo.level" required autocomplete="off">
                                <option v-for="(item, index) in levelOptions" :key="index" :value="index">
                                    {{ item }}
                                </option>
                            </select>
                            <span class="select-tip" :title="levelOptions[todo.level]">优先级</span>
                        </div>
                    </div>
                    <div class="select-group">
                        <div class="select-with-tip">
                            <select v-model="todo.cycle" required autocomplete="off">
                                <option v-for="item in cycleOptions" :key="item.value" :value="item.value">
                                    {{ item.label }}
                                </option>
                            </select>
                            <span class="select-tip"
                                :title="cycleOptions.find(item => item.value === todo.cycle)?.label">重复周期</span>
                        </div>
                    </div>
                </div>

                <div class="form-group">
                    <textarea id="notes" v-model="todo.notes" placeholder="备注（选填）" autocomplete="off"></textarea>
                </div>

                <div class="form-actions">
                    <button type="button" class="cancel-btn" @click="goBack">取消</button>
                    <button type="button" class="back-to-ai-btn" @click="backToAI">重新描述</button>
                    <button type="submit" class="save-btn">保存</button>
                </div>
            </form>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
import { TodoVo } from '@/model/todo';
import defaultAIService from "@/common/ai"
const router = useRouter();

// AI相关状态
const aiInput = ref('')
const isLoading = ref(false)
const showForm = ref(false)

const todo = ref<TodoVo>({
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

// AI处理函数
const handleAISubmit = async () => {
    if (!aiInput.value.trim() || isLoading.value) return

    isLoading.value = true
    try {
        const response = await defaultAIService.extTask(aiInput.value)
        console.log("response", response)
        if (response) {
            todo.value = response
            todo.value.level = 0
        } else {
            todo.value.notes = aiInput.value
        }
        showForm.value = true
    } catch (error) {
        console.error('AI处理失败:', error)
        // 失败时也显示表单，让用户手动填写
        todo.value.notes = aiInput.value
        showForm.value = true
    } finally {
        isLoading.value = false
    }
}

// 跳过AI助手
const skipAI = () => {
    showForm.value = true
}

// 返回AI输入阶段
const backToAI = () => {
    showForm.value = false
    // 重置表单数据
    todo.value = {
        title: '',
        start_time: getCurrentDateTimeLocal(),
        end_time: '',
        notes: '',
        level: 0,
        cycle: 'one'
    }
}

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

.select-group {
    display: flex;
    flex-direction: column;
    flex: 1;
}

.select-with-tip {
    position: relative;
    display: flex;
    align-items: center;
}

.select-tip {
    position: absolute;
    right: 18px;
    font-size: 10px;
    color: var(--text-secondary);
    background: var(--bg-primary);
    padding: 2px 6px;
    border-radius: 3px;
    pointer-events: none;
    font-weight: 500;
    border: 1px solid var(--border-color);
    cursor: help;
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

/* AI相关样式 */
.ai-input-stage {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 20px;
}

.ai-input-container {
    max-width: 500px;
    width: 100%;
    text-align: center;
}

.ai-title {
    font-size: 24px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 8px;
}

.ai-subtitle {
    font-size: 14px;
    color: var(--text-secondary);
    margin-bottom: 24px;
    line-height: 1.5;
}

.ai-form {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.ai-textarea {
    width: 100%;
    padding: 12px;
    border: 2px solid var(--border-color);
    border-radius: 8px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 14px;
    line-height: 1.5;
    resize: vertical;
    min-height: 100px;
    transition: border-color 0.3s;
}

.ai-textarea:focus {
    outline: none;
    border-color: var(--accent-color);
}

.ai-textarea:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.ai-actions {
    display: flex;
    gap: 10px;
    justify-content: center;
    flex-wrap: nowrap;
    margin-top: 10px;
}

.skip-btn,
.ai-submit-btn,
.back-to-ai-btn {
    padding: 8px 16px;
    border: none;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    min-width: 80px;
    position: relative;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    white-space: nowrap;
}

.skip-btn {
    background: linear-gradient(135deg, var(--bg-secondary) 0%, var(--bg-tertiary) 100%);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
}

.skip-btn:hover:not(:disabled) {
    background: linear-gradient(135deg, var(--bg-tertiary) 0%, var(--bg-secondary) 100%);
    color: var(--text-primary);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.skip-btn:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
}

.ai-submit-btn {
    background: linear-gradient(135deg, var(--accent-color) 0%, #667eea 100%);
    color: white;
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.3);
}

.ai-submit-btn:hover:not(:disabled) {
    background: linear-gradient(135deg, #667eea 0%, var(--accent-color) 100%);
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
}

.ai-submit-btn:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.ai-submit-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

/* 按钮波纹效果 */
.skip-btn::before,
.ai-submit-btn::before {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    width: 0;
    height: 0;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.2);
    transform: translate(-50%, -50%);
    transition: width 0.6s, height 0.6s;
}

.skip-btn:active::before,
.ai-submit-btn:active:not(:disabled)::before {
    width: 300px;
    height: 300px;
}

.back-to-ai-btn {
    background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
    color: white;
    border: none;
    box-shadow: 0 4px 15px rgba(240, 147, 251, 0.3);
}

.back-to-ai-btn:hover {
    background: linear-gradient(135deg, #f5576c 0%, #f093fb 100%);
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(240, 147, 251, 0.4);
}

.back-to-ai-btn:active {
    transform: translateY(0);
    box-shadow: 0 4px 12px rgba(240, 147, 251, 0.3);
}

/* 为back-to-ai-btn添加波纹效果 */
.back-to-ai-btn::before {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    width: 0;
    height: 0;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.2);
    transform: translate(-50%, -50%);
    transition: width 0.6s, height 0.6s;
}

.back-to-ai-btn:active::before {
    width: 300px;
    height: 300px;
}

.loading-text {
    display: flex;
    align-items: center;
    gap: 8px;
}

.loading-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top: 2px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    0% {
        transform: rotate(0deg);
    }

    100% {
        transform: rotate(360deg);
    }
}

/* 表单阶段动画 */
.form-stage {
    opacity: 0;
    transform: translateY(20px);
    transition: all 0.4s ease-out;
}

.form-stage.animate-in {
    opacity: 1;
    transform: translateY(0);
}

/* 响应式设计 */
@media (max-width: 600px) {
    .ai-input-container {
        padding: 0 10px;
    }

    .ai-title {
        font-size: 18px;
    }

    .ai-actions {
        gap: 8px;
    }

    .skip-btn,
    .ai-submit-btn {
        min-width: 70px;
        padding: 6px 12px;
        font-size: 11px;
    }
}
</style>