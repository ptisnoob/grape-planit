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
import { getCurrentWindow } from '@tauri-apps/api/window';

const app = createApp(App);
const pinia = createPinia();

app.component('Icon', Icon);
app.use(pinia);
app.use(router);
app.mount("#app");

// 监听快速添加待办事件
listen('quick-add-todo', async () => {
  console.log('🚀 收到快速添加待办事件');
  
  // 检查当前窗口是否为主窗口
  const currentWindow = getCurrentWindow();
  const windowLabel = currentWindow.label;
  
  console.log('当前窗口标签:', windowLabel);
  
  // 只有在主窗口时才导航到添加待办页面
  if (windowLabel === 'main') {
    console.log('在主窗口中，导航到添加待办页面');
    router.push('/add');
  } else {
    console.log('不在主窗口中，忽略快捷键事件');
  }
});
