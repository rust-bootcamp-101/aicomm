import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import { createPinia } from 'pinia';


import { useAuthStore, initPlatformInfo } from './stores/authStore';

// import './assets/styles.css'; // Import any global styles
import './tailwind.css';

const app = createApp(App);

app.use(createPinia());
app.use(router);

await initPlatformInfo()
const authStore = useAuthStore()
authStore.appStart()
app.mount('#app');
