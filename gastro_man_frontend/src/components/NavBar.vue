<template>
  <el-menu mode="horizontal" id="nav" :router="true">
    <el-menu-item route="/">HOME</el-menu-item>
    <el-menu-item class="options" @click="showSettings = !showSettings">SETTINGS</el-menu-item>
  </el-menu>
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
#nav {
  position: fixed;
  top: 0px;
}
</style>
