<template>
  <div class="content">
    
    <router-link to="/workers/add" class="button">
      <span>Add new Worker</span>
      <b-icon icon="plus-circle-outline"></b-icon>
    </router-link>
  
    <router-link to="/workers" class="button is-primary">
      <span>View all workers</span>
    </router-link>
  </div>
</template>

<script>
export default {
  name: 'ManageWorkers',
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.content {
  height: 100px;
  display: flex;  
  flex-direction: column;
  align-items: center;
  justify-content: space-around;
}
</style>
