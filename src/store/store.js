import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const state = {
  host: 'webdev.langeraar.net:4000',
  numParallelUp: 8,
  numParallelDown: 8
}

// mutations: synchronous changes
const mutations = {
  numParallelUp (state, num) {
    state.numParallelUp = num
  },

  numParallelDown (state, num) {
    state.numParallelDown = num
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
