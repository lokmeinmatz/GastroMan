<template>
  <nav class="navbar">
    <div class="navbar-menu">
      <div class="navbar-start">
        <router-link class="navbar-item" to="/">HOME</router-link>
        <a class="navbar-item" @click="showSettings = !showSettings">SETTINGS</a>
      </div>
    </div>
  </nav>
</template>
<script>


export default {
  name: 'NavBar',
  data() {
    return {
    }
  },
  computed: {
    showSettings: {
      get() {
        return this.$store.state.showSettings
      },
      set(val) {
        this.$store.commit('showSettings', val)
      }
    }
  }
}
</script>

<style>
.navbar {
  position: fixed !important;
  top: 0px;
}
</style>
