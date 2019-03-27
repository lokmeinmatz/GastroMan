<template>
  <div class="home">
    
    <Tile v-for="tile in tiles" :key="tile.title" :tiletype="tile"/>
    
  </div>
</template>

<script>
// @ is an alias to /src
import Tile from '@/components/Tile.vue'


export default {
  name: 'home',
  components: {
    Tile
  },
  computed: {
    tiles() {
      return this.$store.getters.validTiles
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
