<template>
  <div id="app">
    <NavBar/>
    <router-view/>
    <settings/>
  </div>
</template>
<script>
import NavBar from './components/NavBar.vue'
import Settings from './components/Settings.vue'

export default {
  components: {NavBar, Settings},
  data() {
    return {
    }
  },
  computed: {
    subPath() {
      let parts = this.$route.path.split('/').filter(v => v.length > 0)
      let total = ''
      for (let p of parts) {
        total += '> ' + p.toUpperCase()
      }
      return total
    }
  }
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

body {
  margin: 0;
}

* {
  box-sizing: border-box;
}

button {
  border: none;
  background-color: rgba(255, 255, 255, 0.8);
  margin: 5px;
  padding: 5px;
  border-radius: 5px;
  cursor: pointer;
}

@media only screen and (max-width: 1200px) {
  .el-dialog {
    width: 70% !important;
  }
}

@media only screen and (max-width: 600px) {
  .el-dialog {
    width: 100% !important;
  }
}


</style>
