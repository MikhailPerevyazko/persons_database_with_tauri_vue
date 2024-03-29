import { createApp } from "vue";
import "./assets/styles.css";
import App from "./App.vue";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";

createApp(App).use(ElementPlus).mount("#app");
