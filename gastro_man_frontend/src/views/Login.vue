<template>
  <div class="login">
    <div class="card">
      <div class="card-header">
        <b-notification class="con-state" :type="stateStyle()" :closable="false">{{stateMsg()}}</b-notification>
      </div>
      <form class="section" @submit.prevent="try_login()">
        <b-field label="Username">
          <b-input v-model="form.username"></b-input>
        </b-field>
        <b-field label="Password">
          <b-input v-model="form.pw" type="password" password-reveal></b-input>
        </b-field>
        <b-field label-width="0px">
          <button
            class="button is-primary"
            :class="{'is-loading': loading}"
            type="primary">Login</button>
        </b-field>
      </form>
    </div>
  </div>
</template>
<script>

import Cookie from 'js-cookie'

export default {
  name: "Login-View",
  mounted() {
    //check if user was logged in before
    let sid = Cookie.get('sid')
    let uname = Cookie.get("user")
    if (sid != undefined && sid != 'undefined' && uname != undefined && uname != 'undefined' && sid.length > 5 && uname.length > 1) {
      this.try_login(uname, sid)
    }
  },

  data() {
    return {
      form: {
        username: "",
        pw: ""
      },
      loading: false
    };
  },
  methods: {
    try_login(uname, sid) {
      if (uname == undefined && sid == undefined && (this.form.username.length == 0 || this.form.pw.length == 0)) return;
      // eslint-disable-next-line
      console.log("Trying to login...");
      this.loading = true;
      //this.$socket.try_login(this.username, this.pw)
      this.$store
        .dispatch("tryLoginAsync", (sid == undefined || uname == undefined) ? {
          password: this.form.pw,
          username: this.form.username
        } : {
          username: uname,
          sid: sid
        })
        .then(() => {
          
          this.$router.push("/");
          this.loading = false;
        })
        .catch(() => {
          
          this.loading = false;
          this.$toast.open({
            message: "Wrong credentials!",
            type: "is-error"
          });
        });
    },

    stateStyle() {
      const rs = this.$socket.rawsock.readyState;
      if (rs == 0) return "is-warning";
      else if (rs == 1) return "is-success";
      return "is-danger";
    },
    stateMsg() {
      const rs = this.$socket.rawsock.readyState;
      if (rs == 0) return "Connecting to Server... ⏳";
      else if (rs == 1) return "Connected to Server ✅";
      return "Can't connect to server ☹️";
    }
  }
};
</script>

<style scoped>
.login {
  height: calc(100vh - 60px);
  background-color: rgb(44, 178, 255);
  width: 100vw;
  display: flex;
  align-items: center;
  justify-content: center;
}

.el-form {
  margin-top: 15px;
}

.con-state {
  width: 100%;
}
</style>
