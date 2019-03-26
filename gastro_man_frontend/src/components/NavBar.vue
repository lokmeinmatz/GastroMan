<template>
  <nav class="navbar is-fixed-top">
    <div class="navbar-brand">
      <a
        role="button"
        class="navbar-burger"
        :class="{'is-active': showOnMobile}"
        aria-label="menu"
        aria-expanded="false"
        data-target="navbarBasicExample"
        @click="showOnMobile = !showOnMobile"
      >
        <span aria-hidden="true"></span>
        <span aria-hidden="true"></span>
        <span aria-hidden="true"></span>
      </a>
    </div>
    <div class="navbar-menu" :class="{'is-active': showOnMobile}">
      <div class="navbar-start">
        <router-link class="navbar-item" to="/">HOME</router-link>
        <a class="navbar-item" @click="showSettings = !showSettings">SETTINGS</a>
      </div>
    </div>
  </nav>
</template>
<script>
export default {
  name: "NavBar",
  data() {
    return {
      showOnMobile: false
    };
  },
  computed: {
    showSettings: {
      get() {
        return this.$store.state.showSettings;
      },
      set(val) {
        this.$store.commit("showSettings", val);
      }
    }
  }
};
</script>

<style>
.navbar {
  position: fixed !important;
  top: 0px;
}
</style>
