import { createApp } from "vue";
import "./styles/tokens.css";
import DesktopLayer from "./components/DesktopLayer.vue";

// 桌面层禁用右键菜单（#7）
window.addEventListener("contextmenu", (e) => e.preventDefault());

createApp(DesktopLayer).mount("#desktop");
