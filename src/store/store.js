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
  config: {}
}

// mutations: synchronous changes
const mutations = {
  numParallelUp (state, num) {
    state.numParallelUp = num
  },

  numParallelDown (state, num) {
    state.numParallelDown = num
  },

  saveSettings (state, data) {
    for (let i in data) {
      if (typeof state[i] !== 'undefined') {
        state[i] = data[i]
      }
    }
  }
}

// actions: asynchronous changes.
const actions = {
  numParallelUp (context, num) {
    context.commit('numParallelUp', num)
  },
  numParallelDown (context, num) {
    context.commit('numParallelDown', num)
  }
}

export default new Vuex.Store({
  state,
  mutations,
  actions,
  strict: process.env.NODE_ENV !== 'production'
})
