<template>
  <div class="content">
    <el-row>
      <el-button icon="el-icon-circle-plus">
        Whole Table
      </el-button>
    </el-row>
    <el-row>
      <el-button icon="el-icon-circle-plus">
        Single Order
      </el-button>
    </el-row>
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
