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

      <b-field>
        <b-checkbox-button v-for="permission in validPermissions" v-model="storePermissions" :native-value="permission" :key="permission">{{permission}}</b-checkbox-button>
      </b-field>

      <a class="button is-warning" @click="$store.commit('logOut')" v-if="$store.state.userdata">Logout</a>

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
    validPermissions() {
      if (this.$store.state.userdata)
        return  this.$store.state.userdata.permissions
      return []
    },
    storePermissions: {
      get() {
        return this.$store.state.filteredPermissions
      },
      set(v) {
        v = v.filter(e => typeof e === 'string' || e instanceof String)

        this.$store.commit('setFilteredPermissions', v)
      }
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
