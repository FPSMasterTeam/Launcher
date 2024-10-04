<template>
    <body>
    <container>
        <div v-if="loggingIn" class="loading">
            <div class="load">
                <h2 data-text="加载中...">加载中...</h2>
            </div>
        </div>
        <div v-else class="loginform">
            <img class="logo" src="../assets/logo.svg" />
            <div class="row">
                <div class="row">
                    <input id="username" placeholder="用户名" />
                    <input id="password" placeholder="密码" />
                    <input id="password" placeholder="重复密码" />
                    <div class="buttons">
                        <a href="/">返回登录</a>
                        <button @click="login()" type="submit">注册</button>
                    </div>
                </div>
            </div>
            <p class="copyright">© 2021 - 2023 FPSMaster</p>
        </div>
    </container>
    </body>
</template>


<style scoped>
.loading {
    position: fixed;
    display: flex;
    top: 0px;
    width: 100%;
    height: 100%;
    left: 0px;
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
    font-size: 5vw;
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
    -webkit-text-stroke: #3b3b3b;
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
    bottom: 0px;
    color: #9b9b9b;
}

.logo:hover {
    filter: drop-shadow(0 0 2em #ffffff52);
}

</style>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
const router = useRouter();

const loggingIn = ref(false);

function login() {
    loggingIn.value = !loggingIn.value;

    setTimeout(() => {
        router.push({ name: "launch" })
    }, 2000);
}

onMounted(() => {

})

</script>