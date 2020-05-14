//import 'whatwg-fetch'

import Vue from 'vue'
import App from './App'
import './style/style.scss'

import Collapse from './directives/Collapse'
Vue.directive('collapse', Collapse)

import Dropdown from './directives/Dropdown'
Vue.directive('dropdown', Dropdown)

import router from './router'
import store from './store'

import Vuex from 'vuex'
Vue.use(Vuex)

// read config, then start app.
var cfgurl = window.location.pathname.replace(/[^/]*$/, './config.json')
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
    render: h => h(App)
  }).$mount('#app')
})

