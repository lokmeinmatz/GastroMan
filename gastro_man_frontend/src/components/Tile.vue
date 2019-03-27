<template>
  <div class="card">
    <div slot="title">
      <h2>{{tiletype.title}}</h2> 
    </div>
    <component :is="tiletype.componentName" v-if="componentExists"/>
    <b-message title="Warning" type="is-warning" v-else :closable="false">
            Component {{tiletype.componentName}} doesn't exist.
        </b-message>
  </div>
</template>

<script>
import AddOrder from '../tiles/AddOrder.vue'
import ManageWorkers from '../tiles/ManageWorkers.vue'

export default {
  name: 'Tile',
  components: {AddOrder, ManageWorkers},
  props: {
    tiletype: Object // {title: string, componentName: String}
  },
  computed: {
    componentExists() {
      return Object.keys(this.$options.components).includes(this.tiletype.componentName)
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.card {
  padding: 10px;
  height: min-content;
  margin: 10px;
}

h2 {
  font-size: 1.5rem;
}
</style>
