<template>
  <el-dialog
    title="Settings"
    :visible.sync="showSettings"
    width="30%">
    <el-container direction="vertical">
      <el-row>
        <el-col :span="4">
          <img :src="userImg" id="user-profile-pic" alt="User profile">
        </el-col>
        <el-col :span="20">
          <p v-if="$store.getters.isLoggedIn">{{ $store.state.userdata.user_name }}</p>
          <p v-else>Not logged in</p>
        </el-col>
      </el-row>
      <el-row type="flex" align="middle">
        <el-col :span="4">
          <el-switch v-model="setFullscreen"></el-switch>
        </el-col>
        <el-col :span="20">
          <p>Display page as fullscreen</p>

        </el-col>
      </el-row>
      <el-row>
        <el-button type="warning" @click="$store.commit('logOut')" v-if="$store.state.logged_in">Logout</el-button>
      </el-row>
    </el-container>
    
 
  </el-dialog>
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
  padding-left: 5px;
  font-size: 1rem;
  text-align: left;
}

.logout {
  background-color: orange;
}


</style>
