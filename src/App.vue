<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import { onMounted } from "vue";

const router = useRouter();
const isActive =(path: string) => {
  return router.currentRoute.value.path === path;
};
// const greetMsg = ref("");
// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }
// 导航函数
onMounted(() => {
  // 如果当前在根路径，导航到 fundamental
  if (router.currentRoute.value.path === '/') {
    router.push('/Fundamental');
  }
});
const navigateTo = (path: string) => {
  router.push(path);
};
</script>

<template>
  <main class="container">

    <aside class="sidebar">
      <a href="https://github.com/SillyBeee" target="_blank">
        <img class=" CV logo" src="./assets/CV_logo.png" alt="ToolKit logo"  />
      </a>

      <h1 class="title">CVToolKit</h1>

      <div
          class="col 1"
          :class="{ 'active': isActive('/Fundamental') }"
          @click="navigateTo('/Fundamental')"
      >
        <img class="icon" src="./assets/pic.svg" alt="icon img" />
        <div>基础图像处理</div>
      </div>

      <div
          class="col"
          :class="{ 'active': isActive('/JudgePara') }"
          @click="navigateTo('/JudgePara')"
      >
        <img class="icon" src="./assets/para.svg" alt="icon img" />
        <div>HSV实时调参</div>
      </div>
    </aside>

    <div class="content-area">
      <keep-alive>
        <router-view v-if="isActive('/Fundamental')" />
      </keep-alive>
      
    </div>

  </main>
</template>

<style>
@keyframes siz {

  from {
    opacity: 0;
    transform: translateY(0); /* 确保初始位置不变 */
  }
  to {
    opacity: 1;
    transform: translateY(0); /* 确保最终位置不变 */
  }


}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #ffffff;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}
.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  
}
.title {
  overflow: hidden;

  animation: siz 3s steps(60) forwards;

  width: 100%;
  height: 28px;
  font-size: 2.3em;
  font-weight: lighter;
}
.icon{
  height: auto;
  width: 1.57em;

}
.col {
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 1.2s;
  left: 0;
  width: 100%; /* 占据父容器的全部宽度 */
  border: none; /* 移除默认边框 */
  background-color: transparent ; /* 背景透明（或根据需要调整颜色）*/
  text-align: center;
  /* //margin: 15px 0; */
  margin: 0;
  padding: 7px 0;
  gap: 8px;
  font-size: inherit; /* 字体大小继承自父级元素 */
  cursor: pointer; /* 鼠标悬停时显示手型指针 */
}
.col.active {
  background-color: #d5d2d2; /* 比普通悬停状态更深的颜色 */
  font-weight: bold;
  filter: drop-shadow(0 0 10px #c7c3c3);
}
.col:hover{
  background-color: #e2dede;
  filter: drop-shadow(0 0 20px #c7c3c3);

}
.logo {
  height: auto;

  width: 10em;
  will-change: filter;
  transition: 0.75s;

}

.CV.logo:hover{
  filter: drop-shadow(0 0 2em #24c8db);
}

.sidebar {
  width: 200px; /* 设置侧边栏宽度 */
  height: 100vh; /* 让侧边栏高度与视口相同 */
  position: fixed; /* 固定定位，使其保持在屏幕同一位置 */
  left: 0; /* 左对齐 */
  top: 0; /* 顶部对齐 */
  background-color: #f4f4f4; /* 背景颜色 */

  box-shadow: 2px 0 5px rgba(0,0,0,0.1); /* 添加轻微阴影效果 */
}

</style>