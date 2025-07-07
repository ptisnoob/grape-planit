import { createApp } from "vue";
import { createPinia } from 'pinia';
import App from "./App.vue";
import router from "./router";
import Icon from "./components/Icon.vue";
import "./assets/css/base.css";
import "./assets/css/themes.css";
import "./assets/css/variable.scss"
import "animate.css";
import { listen } from '@tauri-apps/api/event';
import { useFocusTimer } from '@/composables/useTimer';

const app = createApp(App);
const pinia = createPinia();

app.component('Icon', Icon);
app.use(pinia);
app.use(router);
app.mount("#app");

// 监听快速添加待办事件
listen('quick-add-todo', () => {
  console.log('🚀 收到快速添加待办事件');
  const { delayedFocus } = useFocusTimer();
  
  // 导航到添加待办页面
  router.push('/add').then(() => {
    // 智能聚焦策略：根据页面状态选择合适的输入框
    // 首先尝试AI输入框（AI输入阶段）
    delayedFocus('.ai-textarea', 200);
    
    // 然后尝试标题输入框（表单阶段）
    delayedFocus('.title-input, #title, input[placeholder*="事项名称"]', 250);
  }).catch(err => {
    console.error('❌ 导航到添加待办页面失败:', err);
  });
});
