<template>
  <div class="ai-example">
    <h3>AI 助手示例</h3>
    
    <div class="chat-container">
      <div class="messages" ref="messagesRef">
        <div v-for="(message, index) in messages" :key="index" 
             :class="['message', message.role === 'user' ? 'user-message' : 'ai-message']">
          <div class="message-content">{{ message.content }}</div>
        </div>
        <div v-if="isLoading" class="message ai-message">
          <div class="message-content loading">
            <span class="dot"></span>
            <span class="dot"></span>
            <span class="dot"></span>
          </div>
        </div>
      </div>
      
      <div class="input-container">
        <textarea 
          v-model="userInput" 
          placeholder="输入你的问题..." 
          @keydown.enter.prevent="sendMessage"
          :disabled="isLoading"
          rows="2"
          autocomplete="off"
        ></textarea>
        <button @click="sendMessage" :disabled="isLoading || !userInput.trim()">
          发送
        </button>
      </div>
    </div>
    
    <div class="status" v-if="error">
      <div class="error">{{ error }}</div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, nextTick } from 'vue';
import aiService, { ChatMessage } from '../common/ai.ts';

const userInput = ref('');
const messages = ref<ChatMessage[]>([]);
const isLoading = ref(false);
const error = ref('');
const messagesRef = ref<HTMLElement | null>(null);

// 系统提示，定义AI助手的行为和知识范围
const systemPrompt = `你是一个友好的AI助手，专注于帮助用户管理任务和计划。
你的回答应该简洁、有帮助，并且与任务管理、时间规划相关。
当前应用是一个名为"Grape PlanIt"的任务管理工具。`;

// 发送消息
const sendMessage = async () => {
  const userMessage = userInput.value.trim();
  if (!userMessage || isLoading.value) return;
  
  // 添加用户消息到聊天
  messages.value.push({
    role: 'user',
    content: userMessage
  });
  
  // 清空输入框并滚动到底部
  userInput.value = '';
  await scrollToBottom();
  
  // 设置加载状态
  isLoading.value = true;
  error.value = '';
  
  try {
    // 准备消息数组，包含系统提示和历史消息
    const chatMessages: ChatMessage[] = [
      { role: 'system', content: systemPrompt },
      ...messages.value.slice(-10) // 只保留最近10条消息以避免超出token限制
    ];
    
    // 调用AI服务获取回复
    const response = await aiService.createChatCompletion({
      messages: chatMessages,
      temperature: 0.7, // 控制回复的创造性
    });
    
    // 添加AI回复到聊天
    const aiReply = response.choices[0]?.message?.content || '抱歉，我无法生成回复。';
    messages.value.push({
      role: 'assistant',
      content: aiReply
    });
  } catch (e) {
    console.error('AI请求失败:', e);
    error.value = e instanceof Error ? e.message : '请求失败，请稍后再试';
  } finally {
    isLoading.value = false;
    await scrollToBottom();
  }
};

// 滚动到聊天底部
const scrollToBottom = async () => {
  await nextTick();
  if (messagesRef.value) {
    messagesRef.value.scrollTop = messagesRef.value.scrollHeight;
  }
};

// 组件挂载时添加欢迎消息
onMounted(() => {
  messages.value.push({
    role: 'assistant',
    content: '你好！我是Grape PlanIt的AI助手。有什么可以帮到你的吗？'
  });
});
</script>

<style scoped>
.ai-example {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary);
  color: var(--text-primary);
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow);
}

h3 {
  margin: 0;
  padding: 12px 16px;
  font-size: 16px;
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.chat-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
}

.messages {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.message {
  max-width: 80%;
  padding: 8px 12px;
  border-radius: 12px;
  animation: fadeIn 0.3s ease;
}

.user-message {
  align-self: flex-end;
  background: var(--accent-color);
  color: white;
  border-bottom-right-radius: 4px;
}

.ai-message {
  align-self: flex-start;
  background: var(--bg-secondary);
  border-bottom-left-radius: 4px;
}

.message-content {
  font-size: 14px;
  line-height: 1.4;
  white-space: pre-wrap;
  word-break: break-word;
}

.input-container {
  display: flex;
  padding: 12px;
  border-top: 1px solid var(--border-color);
  background: var(--bg-secondary);
  gap: 8px;
}

textarea {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 14px;
  resize: none;
  outline: none;
  transition: border-color 0.3s;
}

textarea:focus {
  border-color: var(--accent-color);
}

button {
  padding: 8px 16px;
  background: var(--accent-color);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  transition: background 0.3s;
}

button:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.status {
  padding: 8px 12px;
  font-size: 12px;
}

.error {
  color: #e53935;
  background: rgba(229, 57, 53, 0.1);
  padding: 6px 10px;
  border-radius: 4px;
}

.loading {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 24px;
}

.dot {
  width: 6px;
  height: 6px;
  background: var(--text-secondary);
  border-radius: 50%;
  margin: 0 3px;
  animation: bounce 1.4s infinite ease-in-out both;
}

.dot:nth-child(1) {
  animation-delay: -0.32s;
}

.dot:nth-child(2) {
  animation-delay: -0.16s;
}

@keyframes bounce {
  0%, 80%, 100% { transform: scale(0); }
  40% { transform: scale(1); }
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>