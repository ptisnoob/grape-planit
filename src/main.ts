import { createApp } from "vue";
import { createPinia } from 'pinia';
import App from "./App.vue";
import router from "./router";
import Icon from "./components/Icon.vue";
import "./assets/css/base.css";
import "./assets/css/themes.css";
import "./assets/css/variable.scss"
import "animate.css";

const app = createApp(App);
const pinia = createPinia();

app.component('Icon', Icon);
app.use(pinia);
app.use(router);
app.mount("#app");
