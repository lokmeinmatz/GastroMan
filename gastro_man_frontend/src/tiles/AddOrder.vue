<template>
  <div class="content">
    
    <a class="button">
      <span>Whole Table</span>
      <b-icon icon="plus-circle-outline"></b-icon>
    </a>
  
    <a class="button">
      <span>Single Order</span>
      <b-icon icon="plus-circle-outline"/>
    </a>

  </div>
</template>

<script>
export default {
  name: 'AddOrder',
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
