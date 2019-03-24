<template>
  <div class="tile" :style="{width: width, height: height}">
    {{ title }}
  </div>
</template>

<script>
export default {
  name: 'Tile',
  props: {
    size: Object,
    title: String
  },

  computed: {
    width() { return this.size.width * 100 },

    height() { return this.size.height * 120 }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.tile {

}
</style>
