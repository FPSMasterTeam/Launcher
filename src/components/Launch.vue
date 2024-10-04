<template>
  <body>
  <container>
    <div class="user-box">
      <img class="user-logo" src="../assets/user-icon.svg" alt=""/>
      <label>欢迎您，SuperSkidder！</label>
      <a class="exit-label" @click="back()">退出</a>
    </div>

    <div class="runClient">
      <button
          :class="{'loading': isLoading}"
          class="gradient-button"
          @click="startGame"
      >
        <span v-if="!isLoading">启动游戏</span>
        <span v-else>启动中...</span>
        <span v-if="isLoading" class="progress"><br>下载资源中：{{ progress }}</span>
      </button>
    </div>
  </container>
  </body>
</template>

<style>
body {
  width: 100%;
}

.gradient-button {
  background: linear-gradient(135deg, #47abca, #334ed8);
  background-size: 200% 200%;
  color: white;
  border: none;
  padding: 15px 60px;
  font-size: 20px;
  font-weight: bold;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.5s ease;
  width: 300px;
  height: 76px;
}

.gradient-button:hover {
  background-position: 100% 0;
}

.gradient-button.loading {
  animation: gradientFlow 2s linear infinite;
}

@keyframes gradientFlow {
  0% {
    background-position: 0 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0 50%;
  }
}

.progress{
  font-size: 16px;
  color: #d8d8d8;
}

.user-logo {
  width: 20px;
  height: 20px;
  margin: 0 10px 0;
}

.runClient {
  margin-top: 450px;
}

.user-box {
  background: linear-gradient(135deg, #47abca, #334ed8);
  border-radius: 5px;
  box-shadow: 0 4px 4px rgba(4, 4, 4, 0.2);
  padding: 15px;
  position: absolute;
  margin: 5px;
  transition: transform 0.2s;
}


.image-container img {
  filter: brightness(50%);
  transition: filter 0.5s ease;
  width: 300px;
  cursor: pointer;
}

.image-container.selected img {
  filter: brightness(100%);
}

.image-container img:hover {
  filter: brightness(90%) drop-shadow(0 0 1em #ffffff46);
}

.exit-label {
  font-size: 14px;
  color: #9b9b9b;
}

.exit-label:hover {
  color: #fff;
}

</style>

<script setup lang="ts">
import {useRouter} from 'vue-router'
import {invoke} from "@tauri-apps/api/tauri";
import {ref} from "vue";

const isLoading = ref(false);
const progress = ref("");
const router = useRouter();

function back() {
  router.push({name: "login"})
}

function launch() {
  router.push({name: "launching"})
}

async function launchGame(version: String) {
  await invoke("launchgame", {version: version});
}

let interval: number | null = null;

const startGame = () => {
  if (isLoading.value) return;
  isLoading.value = true;
  progress.value = 0;

  interval = setInterval(() => {
    if (progress.value < 100) {
      progress.value += 1;
    } else {
      clearInterval(interval as number);
      isLoading.value = false;
      progress.value = ""
    }
  }, 50);
}
</script>