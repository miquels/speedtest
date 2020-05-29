<template>
<div class="min-height-12vh min-height-8vh-sm-down">
<nav class="navbar navbar-expand-md navbar-dark nav-colors">
<div class="container maxwidth-800">

  <a class="navbar-brand" href="https://www.xs4all.nl/">
    <img src="https://cdn.xs4all.nl/content/_xs4all-themes/first-class/images/logos/xs4all-emblem-yellow.svg" height="25" alt="">
  </a>

  <ul class="navbar-nav mr-auto">
    <router-link tag="li" to="/" exact class="nav-item">
      <a class="nav-link">Speedtest</a>
    </router-link>
  </ul>

  <button type="button" class="navbar-toggler collapsed" v-collapse data-toggle="collapse" data-target="nav-collapse">
    <span class="navbar-toggler-icon"></span>
  </button>

  <div id="nav-collapse" class="navbar-collapse collapse">
    <ul class="navbar-nav d-md-none">
      <router-link tag="li" to="/" exact class="nav-item">
        <a class="nav-link">Speedtest</a>
      </router-link>
    </ul>
    <ul class="navbar-nav">
      <router-link tag="li" to="/info/" exact class="nav-item">
        <a class="nav-link">Info</a>
      </router-link>
      <router-link tag="li" to="/settings/" exact class="nav-item">
        <a class="nav-link">Settings</a>
      </router-link>
    </ul>
    <ul class="navbar-nav d-none d-md-block ml-auto">
      <li class="nav-item">
        <a href="https://github.com/miquels/speedtest" target="_self" class="nav-link">
          <img src="../assets/GitHub-Mark-Light-32px.png" height="32" alt="GitHub">
        </a>
      </li>
    </ul>
  </div>
</div>
</nav>
</div>
</template>

<script>
export default {
  name: 'NavBar'
}
</script>

