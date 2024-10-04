<template>
    <body>
    <container>
        <div v-if="loggingIn" class="loading">
            <div class="load">
                <h2 data-text="正在登录...">正在登录...</h2>
            </div>
        </div>
        <div v-else class="loginform">
            <img class="logo" src="../assets/logo.svg"  alt="logo"/>
            <div class="row">
                <div class="row">
                    <input id="username" placeholder="用户名" />
                    <input id="password" placeholder="密码" />
                    <div class="buttons">
                        <a @click="register()">注册</a>
                        <button @click="login()" type="submit">登录</button>
                    </div>
                </div>
            </div>
            <p class="copyright">© 2021 - 2024 FPSMaster</p>
        </div>
    </container>
    </body>
</template>


<style scoped>
body{
  width: 100%;
}

.logo:hover {
  filter: drop-shadow(0 0 1em #ffffff46);
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.15s;
}


input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #ffffff;
  background-color: #111111;
  transition: 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

input {
  width: 260px;
}

button {
  cursor: pointer;
  width: 130px;
  transition: 0.3s;
  color: #ffffff;
  background-color: #0f0f0f69;
}

button:hover {
  border-color: #717FFE;
  filter: drop-shadow(0 0 2em #717ffe62);
  background-color: #717FFE;
}

input:hover {
  filter: drop-shadow(0 0 2em #ffffff23);
  border-color: #5c5c5c;
}

input,
button {
  outline: none;
}

.row {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 20px;
}

.loading {
    position: fixed;
    display: flex;
    top: 0;
    width: 100%;
    height: 100%;
    left: 0;
}

.loading .load {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    min-height: 100vh;
}

.loading .load h2 {
    position: relative;
    font-size: 30px;
    color: #0f0f0f00;
    -webkit-text-stroke: 0.2vw #3d3d3d;
}

.loading .load h2::before {
    content: attr(data-text);
    position: absolute;
    top: 0;
    left: 0;
    width: 0;
    height: 100%;
    text-wrap: nowrap;
    color: #717FFE;
    border-right: 2px solid #717FFE;
    overflow: hidden;
    animation: animate 3s cubic-bezier(0.075, 0.22, 0.165, 1) infinite;
}

@keyframes animate {
    0% {
        opacity: 10%;
        width: 0;
    }

    50% {
        opacity: 100%;
        width: 100%;
    }

    100% {
        opacity: 10%;
        width: 0;
    }
}


.buttons {
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
    align-items: center;
    width: 100%;
    gap: 20px;

}

.buttons a {
    color: #ffffff;
    text-decoration: none;
    transition: 0.2s;
}

.buttons a:hover {
    color: #717FFE;
    text-decoration: none;
}


.copyright {
    position: fixed;
    left: 20px;
    bottom: 0;
    color: #9b9b9b;
}

.logo:hover {
    filter: drop-shadow(0 0 2em #ffffff52);
}

</style>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { shell } from '@tauri-apps/api';

const router = useRouter();

const loggingIn = ref(false);

function register(){
  shell.open("https://fpsmaster.top/register")
}

function login() {
    loggingIn.value = !loggingIn.value;

    setTimeout(() => {
        router.push({ name: "launch" })
    }, 2000);
}

onMounted(() => {

})

</script>