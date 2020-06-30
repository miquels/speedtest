import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

let isIE = navigator.userAgent.match(/Trident|MSIE/)
let conns = isIE ? 6 : 8

const state = {
  connType: 'Generic',
  units: 'Mbps',
  MBexcludeOverhead: true,
  connsUp: conns,
  connsDown: conns,
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
  setConfig (state, cfg) {
    // get values from config or from defaults.
    let proto = cfg.protocol || window.location.protocol
    if (proto && !proto.endsWith(':')) {
      proto = proto + ':'
    }
    let ws_proto = proto.startsWith('http:') ? 'ws:' : 'wss:'
    let port = cfg.apiport || window.location.port || ''
    if (port) {
      port = ':' + port
    }
    if (!cfg.apihost || !cfg.apihost.default) {
      cfg.apihost = { default: window.location.hostname }
    }

    // now build up config.api values.
    let config = { api: {} }
    config.api.default = proto + '//' + cfg.apihost.default + port
    config.api.ws = ws_proto + '//' + cfg.apihost.default + port
    if (cfg.apihost.ipv4) {
      config.api.ipv4 = proto + '//' + cfg.apihost.ipv4 + port
    }
    if (cfg.apihost.ipv6) {
      config.api.ipv6 = proto + '//' + cfg.apihost.ipv6 + port
    }

    console.log('config:', config)
    state.config = config
  },

  saveSettings (state, data) {
    for (let i in data) {
      if (typeof state[i] !== 'undefined') {
        if (isIE && (i === 'connsUp' || i === 'connsDown') && +data[i] > 6) {
          data[i] = 6
        }
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
