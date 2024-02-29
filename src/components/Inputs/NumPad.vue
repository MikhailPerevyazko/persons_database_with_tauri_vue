<script setup lang="ts">
import { ElButton } from "element-plus";
import { ref } from "vue";
const dialogVisible = ref(false);
</script>

<template>
  <div class="m-4">
    <p @click="dialogVisible = true">Press me!</p>
    <el-dialog v-model="dialogVisible">
      <textarea class="border border-amber-600" placeholder="Select..">
      </textarea>
      <el-button class="m-4" type="danger" circle>Clear</el-button>
      <div>
        <el-row class="py-1 justify-center">
          <el-button value="1" type="success">1</el-button>
          <el-button value="2" type="success">2</el-button>
          <el-button value="3" type="success">3</el-button>
        </el-row>
        <el-row class="py-1 justify-center">
          <el-button value="4" type="success">4</el-button>
          <el-button value="5" type="success">5</el-button>
          <el-button value="6" type="success">6</el-button>
        </el-row>
        <el-row class="py-1 justify-center">
          <el-button value="7" type="success">7</el-button>
          <el-button value="8" type="success">8</el-button>
          <el-button value="9" type="success">9</el-button>
        </el-row>
        <el-row class="py-1 justify-center">
          <el-button value="0" type="success">0</el-button>
        </el-row>
      </div>
    </el-dialog>
  </div>
</template>

<style scoped></style>
