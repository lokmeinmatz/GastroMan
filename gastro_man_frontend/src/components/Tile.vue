<template>
  <el-card>
    <div slot="header">
      <h2>{{tiletype.title}}</h2> 
    </div>
    <component :is="tiletype.componentName"/>
  </el-card>
</template>

<script>
import AddOrder from '../tiles/AddOrder.vue'
export default {
  name: 'Tile',
  components: {AddOrder},
  props: {
    size: Object,
    tiletype: Object // {title: string, componentName: String}
  },

  computed: {
    width() { return this.size.width * 100 },

    height() { return this.size.height * 120 },

    style() {
      return {
        'width': this.width + 'px',
        'height': this.height + 'px'
      }
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.el-card {
  display: block;
  height: min-content;
}

h2 {
  margin: 0;
}
</style>
