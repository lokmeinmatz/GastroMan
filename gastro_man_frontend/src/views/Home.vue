<template>
  <div class="home">
    
    <Tile v-for="tile in tiles" :key="tile.title" :tiletype="tile"/>
    
  </div>
</template>

<script>
// @ is an alias to /src
import Tile from '@/components/Tile.vue'

class TileProp {
  constructor(title, compName) {
    this.title = title,
    this.componentName = compName
  }
}

export default {
  name: 'home',
  components: {
    Tile
  },
  data() {
    return {
      tiles: [new TileProp('New Order', 'AddOrder')]
    }
  }
}
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
