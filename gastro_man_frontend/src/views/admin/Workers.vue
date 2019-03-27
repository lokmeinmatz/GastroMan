<template>
  <div class="workers">
    <div class="page-options">
      <router-link to="/workers/add" class="button is-primary">
        <span>Add new Worker</span>
        <b-icon icon="plus-circle-outline"></b-icon>
      </router-link>
    </div>
    <b-table :data="[]" :columns="columns"/>
  </div>
</template>

<script>
export default {
  name: "workers",
  mounted() {
    this.$store.dispatch('getUserList')
  },
  data() {
    return {
      columns: [
        {field: 'id', label: 'ID', width: '40', numeric: true},
        {field: 'username', label: 'Username'},
        {field: 'firstname', label: 'First name'},
        {field: 'secname', label: 'Last name'},
        {field: 'permissions', label: 'Roles'}
      ]
    }
  },
  computed: {
    tiles() {
      return this.$store.getters.validTiles;
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
