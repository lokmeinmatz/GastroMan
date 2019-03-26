<template>
  <b-modal
    title="Settings"
    :active.sync="showSettings">
    <div class="card">
      
      <img :src="userImg" id="user-profile-pic" style="justify-self: center;" alt="User profile">
      <p v-if="$store.getters.isLoggedIn">{{ $store.state.userdata.user_name }}</p>
      <p v-else>Not logged in</p>
      
      <b-switch v-model="setFullscreen" style="justify-self: center;"></b-switch>
      <p>Display page as fullscreen</p>

      <a class="button is-warning" @click="$store.commit('logOut')" v-if="$store.state.logged_in">Logout</a>

    </div>
    
 
  </b-modal>
</template>
<script>
import defaultImg from '@/assets/profile_placeholder.png'

export default {
  name: 'Settings',
  methods: {
  },
  computed: {
    userImg() {
      return defaultImg
    },
    showSettings: {
      get() {
        return this.$store.state.showSettings
      },
      set(val) {
        this.$store.commit('showSettings', val)
      }
    },
    setFullscreen: {
      get() {
        return this.$store.state.fullscreen
      },
      set(v) {
        this.$store.commit('setFullscreen', v)
      }
    }
  }
}
</script>

<style scoped>

#user-profile-pic {
  width: 50px;
  height: 50px;
  border-radius: 50%;
}

p {
  text-align: left;
  line-height: 50px;
}

.card {
  display: grid;
  grid-template-columns: 1fr 4fr;
  padding: 20px;
}

</style>
