<template>
  <div class="add-worker">
    <form class="section">
      <b-field label="Username" :message="validator.user_name" :type="(validator.user_name == '') ? '' : 'is-danger'">
        <b-input v-model="newUser.user_name"></b-input>
      </b-field>
      
      <b-field label="First name" :message="validator.first_name" :type="(validator.first_name == '') ? '' : 'is-danger'">
        <b-input v-model="newUser.first_name"></b-input>
      </b-field>
      <b-field label="Last name" :message="validator.last_name" :type="(validator.last_name == '') ? '' : 'is-danger'">
        <b-input v-model="newUser.last_name"></b-input>
      </b-field>
      <b-field label="Password" :message="validator.password" :type="(validator.password == '') ? '' : 'is-danger'">
        <b-input type="password" v-model="newUser.password" password-reveal></b-input>
      </b-field>
      <div class="permselect">
        <label class="label">Select permissions for worker.</label>
        <b-field position="is-centered">
          <b-checkbox-button v-for="p in allPermissions" v-model="newUser.permissions" :native-value="p" :key="p">{{p}}</b-checkbox-button>
        </b-field>
      </div>
      <div class="level">
        <div class="level-left"></div>
        <div class="level-right">
          <b-tooltip class="level-item" position="is-bottom" label="All field required!">
            <button class="button" :class="{'is-success': inputValid}"
            :disabled="!inputValid" @click.prevent="addUser">Add new User</button>
          </b-tooltip>
          <button class="level-item button is-danger" @click.prevent="goBack">Cancel</button>
        </div>
      </div>
    </form>
  </div>
</template>

<script>
import { User } from '@/types'
import permissions from '@/tiles/permissions.json'

export default {
  name: "addworker",
  data() {
    return {
      newUser: new User(),
      allPermissions: Object.keys(permissions)
    }
  },
  computed: {
    tiles() {
      return this.$store.getters.validTiles;
    },
    validator() {
      let obj = {
        user_name: '',
        first_name: '',
        last_name: '',
        password: '',
        permissions: ''
      }

      if (this.newUser.user_name.length > 0 && this.newUser.user_name.length < 4) obj.user_name = 'Username must be at least 4 characters long.'
      else if(this.newUser.user_name.length == 0) obj.user_name = 'Username must be provided.'
      
      if (this.newUser.first_name.length == 0) obj.first_name = 'First name must be provided.'
      if (this.newUser.last_name.length == 0) obj.last_name = 'First name must be provided.'

      if (this.newUser.password.length < 6) obj.password = 'Password must be at least 6 characters long.'

      if (this.newUser.permissions.length < 1) {
        this.newUser.permissions.push(this.allPermissions[0])}
      return obj
    },
    inputValid() {
      return this.newUser.user_name.length >= 4 && this.newUser.first_name.length > 0 && this.newUser.last_name.length > 0 && this.newUser.password.length >= 6 && this.newUser.permissions.length >= 1
    }
  },
  methods: {
    goBack() {
      window.history.length > 1
        ? this.$router.go(-1)
        : this.$router.push('/')
    },
    addUser() {
      this.triedToAdd = true
    }
  }
};
</script>
<style scoped>
.home {
  width: 100%;
  display: flex;
  background-color: rgb(230, 230, 230);
  min-height: calc(100vh - 60px);
  padding: 10px;
}
</style>
