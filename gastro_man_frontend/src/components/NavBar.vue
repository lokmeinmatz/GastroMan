<template>
  <div id="nav">
    <router-link to="/">HOME</router-link>
    <p>{{subPath}}</p>
    <button class="options" @click="showOptions = !showOptions">⚙️</button>
    <options-dropdown v-show="showOptions"/>
  </div>
</template>
<script>
import OptionsDropdown from './OptionsDropdown.vue'

export default {
  name: 'NavBar',
  components: {OptionsDropdown},

  data() {
    return {
      showOptions: false
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
#nav {
  position: fixed;
  height: 60px;
  width: 100vw;
  top: 0;
  background-color: aquamarine;
  display: grid;
  grid-template-columns: 200px auto 200px;
  align-items: center;
  padding: 0px 10px;
  justify-content: space-between;
  overflow: hidden;
}

#nav > a, #nav > p {
  text-decoration: none;
  font-size: 2rem;
  padding-left: 6px;
  color: #2a2a2a;
  margin: 0;
}

#nav a {
  padding: 3px 6px;
  border-radius: 5px;
  background-color: rgba(255, 255, 255, 0.6);
}

#nav .options {
  font-size: 1.5rem;
  justify-self: right;
}

#nav a:hover {
  background-color: rgba(255, 255, 255, 0.8);
}
@media only screen and (max-width: 600px) {
  #nav a, #nav p {
  font-size: 1.5rem;
}
}
</style>
