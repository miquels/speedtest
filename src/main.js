//import 'whatwg-fetch'

//import './style/style.scss'
//import '../node_modules/bootstrap/scss/bootstrap.scss'

import Vue from 'vue'
import App from './App'

import Collapse from './directives/Collapse'
Vue.directive('collapse', Collapse)

import Dropdown from './directives/Dropdown'
Vue.directive('dropdown', Dropdown)

import router from './router'
import store from './store'

import Vuex from 'vuex'
Vue.use(Vuex)

// read config, then start app.
var base = window.location.pathname.replace(/(\/settings\/?|\/info\/?|\/[^/]+)$/, '/');
var cfgurl = base + 'config.json';
window.fetch(cfgurl, {
  redirect: 'follow'
}).then((resp) => {
  if (!resp.ok) {
    throw new RangeError(`${cfgurl}: unexpected HTTP code: ${resp.status}`)
  }
  return resp.text()
}).then((text) => {
  let json = text.replace(/\/\/.*\n/gm, '')
  let config = JSON.parse(json)
  store.commit('setConfig', config)

  /* eslint-disable no-new */
  new Vue({
    el: '#app',
    store,
    router,
    render: h => h(App)
  }).$mount('#app')
})

