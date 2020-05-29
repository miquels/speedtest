import Vue from 'vue'
import VueRouter from 'vue-router'

import SpeedTest from '../views/SpeedTest'
import Settings from '../views/Settings'
import Info from '../views/Info'

Vue.use(VueRouter)

const routes = [
      { path: '/', component: SpeedTest },
      { path: '/settings/', component: Settings },
      { path: '/info/', component: Info },
      { path: '*', redirect: '/' }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  scrollBehavior: () => ({ y: 0 }),
  linkActiveClass: 'active',
  routes
})

export default router
