<template>
  <div class="card">
    <div slot="title">
      <h2>{{tiletype.title}}</h2> 
    </div>
    <component :is="tiletype.componentName"/>
  </div>
</template>

<script>
import AddOrder from '../tiles/AddOrder.vue'
export default {
  name: 'Tile',
  components: {AddOrder},
  props: {
    tiletype: Object // {title: string, componentName: String}
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.card {
  padding: 10px;
  height: min-content;
}
</style>
