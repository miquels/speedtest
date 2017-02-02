import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const state = {
  connType: 'Generic',
  units: 'Mbps',
  MBexcludeOverhead: true,
  connsUp: 8,
  connsDown: 8,
  l23overhead: [ 'ether' ],
  l12overhead: 'fiber',
  overhead: 1.01,
  config: {},
  ip: {
    default: null,
    ipv4: null,
    ipv6: null,
    info: '--'
  }
}

// mutations: synchronous changes
const mutations = {
  setConfig (state, config) {
    state.config = config
  },

  saveSettings (state, data) {
    for (let i in data) {
      if (typeof state[i] !== 'undefined') {
        state[i] = data[i]
      }
    }
  },

  setIP (state, {family, ip}) {
    state.ip[family] = ip
    let ips = []; // ASI
    [ state.ip.ipv4, state.ip.ipv6 ].filter(f => f !== null).forEach((ip) => {
      let n = ip
      if (n === state.ip.default) {
        n += ' (default)'
      }
      ips.push(n)
    })
    if (ips.length === 0) {
      ips.push(state.ip.default || '--')
    }
    state.ip.info = ips.join(', ')
  }
}

// actions: asynchronous changes.
const actions = {
}

export default new Vuex.Store({
  state,
  mutations,
  actions,
  strict: process.env.NODE_ENV !== 'production'
})
