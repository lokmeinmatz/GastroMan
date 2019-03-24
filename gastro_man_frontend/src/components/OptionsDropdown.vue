<template>
  <div id="options-dropdown">
    <img :src="userImg" id="user-profile-pic" alt="User profile">
    <p v-if="$store.getters.isLoggedIn">{{ $store.state.userdata.user_name }}</p>
    <p v-else>Not logged in</p>
    <button style="grid-column: 1 / span 2" @click="toggleFullscreen()">Toggle Fullscreen</button>
    <button class="logout" @click="$store.commit('logOut')" style="grid-column: 1 / span 2" v-if="$store.state.logged_in">Logout</button>
  </div>
</template>
<script>
import defaultImg from '@/assets/profile_placeholder.png'

export default {
  name: 'OptionsDropdown',
  methods: {
    toggleFullscreen() {
      this.$store.commit('toogleFullscreen')
    }
  },
  computed: {
    userImg() {
      return defaultImg
    }
  }
}
</script>

<style scoped>
#options-dropdown {
  max-width: 600px;
  min-width: 50px;
  min-height: 80px;
  position: fixed;
  background-color: rgb(240, 240, 240);
  right: 5px;
  top: 65px;
  
  z-index: 5;
  display: grid;
  padding: 5px;
  grid-template-columns: 1fr 4fr;
  -webkit-box-shadow: 0px 5px 5px 0px rgba(0,0,0,0.5);
  -moz-box-shadow: 0px 5px 5px 0px rgba(0,0,0,0.5);
  box-shadow: 0px 5px 5px 0px rgba(0,0,0,0.5);
  border-radius: 5px;
}

#options-dropdown::before {
  background-color: rgb(240, 240, 240);
  content: '';
  display: block;
  position: absolute;
  top: -8px;
  right: 22px;
  width: 20px;
  height: 20px;
  transform: rotate(45deg);
  z-index: -5;

}

#user-profile-pic {
  width: 50px;
  height: 50px;
  border-radius: 50%;
}

p {
  font-size: 1rem;
}

.logout {
  background-color: orange;
}

@media only screen and (max-width: 600px) {
  #options-dropdown {
    width: 90vw;
    margin: auto 5vw;
  }

  #options-dropdown::before {
 
  top: -8px;
  right: 5px;
}
}
</style>
