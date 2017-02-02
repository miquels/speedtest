<template>
<div id="app">

<div class="min-height-12vh">
<nav class="navbar navbar-dark nav-colors">
  <div class="container maxwidth-800">
  <a class="navbar-brand" href="https://www.xs4all.nl/">
    <img src="https://cdn.xs4all.nl/content/_xs4all-themes/first-class/images/logos/xs4all-emblem-yellow.svg" height="25" alt="">
  </a>
  <button class="navbar-toggler hidden-md-up float-xs-right" v-collapse type="submit"></button>
  <a class="navbar-brand hidden-sm-down float-xs-right" href="https://github.com/miquels/speedtest">
    <img src="./assets/GitHub-Mark-Light-32px.png" height="32" alt="">
  </a>
  <ul class="nav navbar-nav">
    <router-link tag="li" to="/" exact class="nav-item">
      <a class="nav-link">Speedtest</a>
    </router-link>
  </ul>
  <div class="collapse navbar-toggleable-sm">
    <div class="hidden-md-up">&nbsp;<p>&nbsp;</p></div>
    <ul class="nav navbar-nav">
      <router-link tag="li" to="/" exact class="nav-item">
        <a class="nav-link hidden-md-up">Speedtest</a>
      </router-link>
      <router-link tag="li" to="/info/" class="nav-item">
        <a class="nav-link">Info</a>
      </router-link>
      <router-link tag="li" to="/settings/" class="nav-item">
        <a class="nav-link">Settings</a>
      </router-link>
    </ul>
  </div>
  </div>
</nav>
</div>

<div class="container main maxwidth-800">
  <transition name="fade" mode="out-in">
    <router-view class="view"></router-view>
  </transition>
</div>

</div>
</template>

<script>
export default {
  name: 'app',
  mounted () {
    // get dual-stack / v4 / v6 addresses.
    let s = this.$store
    let c = this.$store.state.config; // ASI
    [ 'default', 'ipv4', 'ipv6' ].forEach((family) => {
      this.getIP(c.apihost[family], c.apiport).then((r) => {
        s.commit('setIP', { family: family, ip: r.remoteip })
      }).catch(() => {})
    })
  },
  methods: {
    getIP (host, port) {
      let url = `http://${host}:${port}/speedtest/ip`
      return window.fetch(url, {
        redirect: 'follow'
      }).then((resp) => {
        if (!resp.ok) {
          throw new RangeError(`${url}: unexpected HTTP code ${resp.status}`)
        }
        console.log(resp)
        return resp.json()
      })
    }
  }
}
</script>

<style lang="scss">
html, body {
  margin: 0px;
  padding: 0px;
  height: 100vh;
}
body {
  background: #ffffff;
}
#app {
  min-height: 100%;
}
/*
.main {
  position: relative;
  overflow-x: hidden;
  height: 78vh;
}
.view {
  position: absolute;
  top: 0px;
  left: 0px;
  right: 0px;
  bottom: 0px;
}
*/
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
