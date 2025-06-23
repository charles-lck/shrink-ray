import { createApp } from "vue";
import { createPinia } from 'pinia';
import App from "./App.vue";
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import "./assets/css/main.css";
import router from './router';
import i18n from './assets/js/i18n'
const pinia = createPinia();

pinia.use(piniaPluginPersistedstate);
createApp(App).use(router).use(pinia).use(i18n).mount("#app");
