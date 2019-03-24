<template>
  <div class="login">
    <div class="panel">
      <div class="server-state" :class="stateStyle">
        <p>{{ stateMsg }}</p>
      </div>
      <p>Username</p>
      <input type="text" class="tb" v-model="username">
      <p>Password</p>
      <input type="text" class="tb" v-model="pw">
      <input type="button" value="Connect" id="connect" @click="try_login()">
    </div>
  </div>
</template>
<script>
export default {
  name: 'Login-View',
  data() {
    return {
      username: '',
      pw: ''
    }
  },
  methods: {
    try_login() {

      if (this.username.length == 0 || this.pw.length == 0) return
      // eslint-disable-next-line
      console.log('Trying to login...')
      this.$socket.try_login(this.username, this.pw)
      this.onLoggedIn(this.username, 'asdfgh12345')
    },

    onLoggedIn(username, sid) {
      this.$store.commit('setLoggedIn', {username: username, sid: sid})
      
      this.$router.push('/')
    }
  },

  computed: {
    stateStyle() {
      const rs = this.$socket.rawsock.readyState
      if (rs == 0) return 'connecting'
      else if (rs == 1) return 'connected'
      return 'not-connected'
    },
    stateMsg() {
      const rs = this.$socket.rawsock.readyState
      if (rs == 0) return 'Connecting to Server... ⏳'
      else if (rs == 1) return 'Connected to Server ✅'
      return 'Can\'t connect to server ☹️'
    }
  }
};
</script>

<style scoped>

.server-state {
  background-color: gray;
  grid-column: 1 / span 2;
  color: white;
  height: 30px;
  overflow: hidden;
  line-height: 30px;
}

.server-state.connecting {
  background-color: orange;
}

.server-state.not-connected {
  background-color: #da222e;
}

.server-state.connected {
  background-color: #64a718;
}

.server-state p {margin: 0;}

.login {
  height: calc(100vh - 60px);
  background-color: rgb(230, 230, 230);
  width: 100vw;
  display: flex;
  align-items: center;
  justify-content: center;
}

.panel {
  max-width: 700px;
  max-height: 500px;
  background-color: white;
  border-radius: 10px;
  overflow: hidden;
  display: grid;
  grid-template-columns: 1fr 2fr;
}

.panel input{
  margin: 10px;
  padding: 5px;
  font-size: 1rem;
}

.panel .tb {
  border: none;
  border-radius: 5px;
  background-color: #eaeaea;
}

.panel #connect {
  grid-column: 1 / span 2;
  border: none;
  border-radius: 5px;
  background-color: #CBFFEE;
  cursor: pointer;
}

.panel #connect:hover {
  background-color: rgb(119, 255, 210);
}
</style>
