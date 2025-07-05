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

const app = createApp(App);
const pinia = createPinia();

app.component('Icon', Icon);
app.use(pinia);
app.use(router);
app.mount("#app");

// 监听快速添加待办事件
listen('quick-add-todo', () => {
  console.log('🚀 收到快速添加待办事件');
  // 导航到添加待办页面
  router.push('/add').then(() => {
    // 等待页面加载完成后聚焦到输入框
    setTimeout(() => {
      // 尝试聚焦到标题输入框
      const titleInput = document.querySelector('.title-input, #title, input[placeholder*="事项名称"]') as HTMLInputElement;
      if (titleInput) {
        titleInput.focus();
        titleInput.select();
        console.log('✅ 聚焦到标题输入框');
      } else {
        // 如果在AI输入阶段，聚焦到AI输入框
        const aiInput = document.querySelector('.ai-textarea, textarea[placeholder*="描述"]') as HTMLTextAreaElement;
        if (aiInput) {
          aiInput.focus();
          aiInput.select();
          console.log('✅ 聚焦到AI输入框');
        } else {
          console.log('❌ 未找到可聚焦的输入框');
        }
      }
    }, 200);
  }).catch(err => {
    console.error('❌ 导航到添加待办页面失败:', err);
  });
});
