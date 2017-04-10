import 'babel-polyfill'
import 'whatwg-fetch'

import Vue from 'vue'
import App from './App'
import './style/style.scss'

import Collapse from 'directives/Collapse'
import Dropdown from 'directives/Dropdown'
Vue.directive('collapse', Collapse)
Vue.directive('dropdown', Dropdown)

import Router from 'vue-router'
import store from './store'
import Vuex from 'vuex'

Vue.use(Vuex)
Vue.use(Router)

import SpeedTest from './views/SpeedTest'
import Settings from './views/Settings'
import Info from './views/Info'

var router = new Router({
  mode: 'history',
  scrollBehavior: () => ({ y: 0 }),
  linkActiveClass: 'active',
  routes: [
      { path: '/', component: SpeedTest },
      { path: '/settings/', component: Settings },
      { path: '/info/', component: Info },
      { path: '*', redirect: '/' }
  ]
})

// read config, then start app.
var cfgurl = window.location.pathname.replace(/[^/]*$/, 'static/config.json')
window.fetch(cfgurl, {
  redirect: 'follow'
}).then((resp) => {
  if (!resp.ok) {
    throw new RangeError(`${cfgurl}: unexpected HTTP code: ${resp.status}`)
  }
  return resp.json()
}).then((config) => {
  store.commit('setConfig', config)

  /* eslint-disable no-new */
  new Vue({
    el: '#app',
    store,
    router,
    template: '<App/>',
    components: { App }
  })
})

