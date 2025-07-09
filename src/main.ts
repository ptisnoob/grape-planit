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
  router.push('/add')
});
