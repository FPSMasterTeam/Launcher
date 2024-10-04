// Composables
import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    name: "login",
    path: '/',
    component: () => import('../components/Login.vue'),
  },
  {
    name: "register",
    path: '/register',
    component: () => import('../components/Register.vue'),
  },
  {
    name: "launch",
    path: '/launch',
    component: () => import('../components/Launch.vue'),
  },
  {
    path: '/404',
    component: () => import('../components/NotFound.vue'),
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
