<template>
<div id="app">
  <NavBar/>
  <div class="container main maxwidth-800">
    <transition name="fade" mode="out-in">
      <router-view class="view"></router-view>
    </transition>
  </div>
</div>
</template>

<script>
import NavBar from './components/NavBar.vue'

export default {
  name: 'app',
  components: {
    NavBar,
  },
  mounted () {
    // get dual-stack / v4 / v6 addresses.
    let s = this.$store
    let c = this.$store.state.config; // ASI
    [ 'default', 'ipv4', 'ipv6' ].forEach((family) => {
      this.getIP(c.api[family]).then((r) => {
        s.commit('setIP', { family: family, ip: r.remoteip })
      }).catch(() => {})
    })
  },
  methods: {
    getIP (baseUrl) {
      let url = baseUrl + '/speedtest/ip'
      return window.fetch(url, {
        redirect: 'follow'
      }).then((resp) => {
        if (!resp.ok) {
          throw new RangeError(`${url}: unexpected HTTP code ${resp.status}`)
        }
        //console.log(resp)
        return resp.json()
      })
    }
  }
}
</script>

<style lang="scss">
@import "./style/bootstrap-post.scss";
@import "./style/style.scss";
html, body {
  margin: 0px;
  padding: 0px;
  height: 100vh;
}
#app {
  min-height: 100%;
}
.nav-colors {
  color: white;
  background: #191919;
}
.fade-enter-active, .fade-leave-active {
  transition: all .2s ease-in-out;
}
.fade-enter, .fade-leave-active {
  opacity: 0;
}
.slide-enter {
  transform: translateX(-100%);
}
.slide-enter-active {
  transition: all .3s ease;
}
.slide-leave-active {
  transition: all .3s ease;
  transform: translateX(100%);
}
</style>
