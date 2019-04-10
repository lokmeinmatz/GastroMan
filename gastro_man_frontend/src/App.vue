<template>
  <div id="app">
    <NavBar/>
    <transition name="main-rv" appear mode="out-in">
      <router-view class="rv"/>
    </transition>
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
  mounted() {
    this.$socket.addListenerConstant('permissionerror', (e) => this.$dialog.alert({
      title: 'Permission denied',
      message: e.msg,
      type: 'is-danger'
    }))
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
  background-color: rgb(230, 230, 230);
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

.main-rv-enter-active, .main-rv-leave-active {
  transition: transform 0.5s linear;
}

.main-rv-enter {
  transform: translateY(-100vh);
}

.main-rv-leave-to {
  transform: translateY(100vh);
}


.page-options {
  display: flex;
  padding: 10px;
}

.rv {
  min-height: calc(100vh - 52px);
}

</style>
