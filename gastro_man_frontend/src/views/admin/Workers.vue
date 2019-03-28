<template>
  <div class="workers">
    <div class="page-options">
      <router-link to="/workers/add" class="button is-primary">
        <span>Add new Worker</span>
        <b-icon icon="plus-circle-outline"></b-icon>
      </router-link>
    </div>
    <b-table :data="$store.state.admin.allUsers">
      <template slot-scope="props">
        <b-table-column field="user_name" label="Username">{{ props.row.user_name }}</b-table-column>
        <b-table-column field="first_name" label="First Name">{{ props.row.first_name }}</b-table-column>
        <b-table-column field="last_name" label="Last Name">{{ props.row.last_name }}</b-table-column>
        <b-table-column field="permissions" label="Permissions">
          <b-field>
            <b-checkbox-button v-for="p in permissions" :key="p" :native-value="p" :value="user_permissions[props.row.user_name]" @input="updatePermission(props.row.user_name, $event)">
              {{ p }}
            </b-checkbox-button>
          </b-field>
        </b-table-column>
      </template>
    </b-table>
  </div>
</template>

<script>
import perm from '@/tiles/permissions.json'

export default {
  name: "workers",
  mounted() {
    this.$store.dispatch('getUserList')
  },
  data() {
    return {
      permissions: Object.keys(perm)
    }
  },
  methods: {
    updatePermission(user_name, p) {
      this.$store.dispatch('updateUserPermission', {user_name: user_name, permissions: p})
      .then(() => this.$store.dispatch('getUserList'))
    }
  },
  computed: {
    tiles() {
      return this.$store.getters.validTiles;
    },
    user_permissions: {
      get() {
        let o = {}
        for (let u of this.$store.state.admin.allUsers) {
          o[u.user_name] = u.permissions
        }
        return o
      }
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
