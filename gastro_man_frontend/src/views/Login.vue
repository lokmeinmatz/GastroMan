<template>
  <div class="login">
    <div class="card">
      <div class="card-header">
        <b-notification :type="stateStyle" :closable="false">{{stateMsg}}</b-notification>
      </div>
      <form>
        <el-form-item label="Username" :required="true">
          <el-input v-model="form.username"></el-input>
        </el-form-item>
        <el-form-item label="Password" :required="true">
          <el-input v-model="form.pw" show-password></el-input>
        </el-form-item>
        <el-form-item label-width="0px">
          <el-button type="primary" id="login-button" @click="try_login()" :loading="loading">Login</el-button>
        </el-form-item>
      </form>
    </div>
  </div>
</template>
<script>
export default {
  name: 'Login-View',
  data() {
    return {
      form: {
        username: '',
        pw: ''
      },
      loading: false
    }
  },
  methods: {
    try_login() {

      if (this.form.username.length == 0 || this.form.pw.length == 0) return
      // eslint-disable-next-line
      console.log('Trying to login...')
      this.loading = true
      //this.$socket.try_login(this.username, this.pw)
      this.$store.dispatch('tryLoginAsync', {password: this.form.pw, username: this.form.username})
      .then(() => {
        this.$router.push('/')
        this.loading = false
      }).catch(() => {
        this.loading = false
        this.$message('Wrong credentials.')
        })
    }
  },

  computed: {
    stateStyle() {
      const rs = this.$socket.rawsock.readyState
      if (rs == 0) return 'is-warning'
      else if (rs == 1) return 'is-success'
      return 'is-danger'
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

.login {
  height: calc(100vh - 60px);
  background-color: rgb(230, 230, 230);
  width: 100vw;
  display: flex;
  align-items: center;
  justify-content: center;
}

.el-form {
  margin-top: 15px;
}

#login-button {
  width: 100%;
}
</style>
