<script setup lang="ts">
import { ElButton } from "element-plus";
import { ref } from "vue";
const dialogVisible = ref(false);
</script>

<template>
  <p
    type="text"
    id="result"
    value=""
    @click="dialogVisible = true"
    class="border-2 border-amber-500 items-center m-10 h-7"
  ></p>
  <div class="m-4">
    <el-dialog v-model="dialogVisible">
      <form action="" method="post">
        <el-row class="py-1">
          <input
            type="txt"
            id="text"
            value=""
            placeholder="Select.."
            class="m-4"
          />
          <el-button
            class="m-4"
            type="success"
            id="recordButton"
            onclick="document.getElementById('recordButton').addEventListener('click', function() {
              var inputValue = document.getElementById('text').value;
              document.getElementById('result').innerHTML = inputValue;
            });"
            >Record</el-button
          >
          <el-button
            class="m-4"
            id="clearButton"
            type="danger"
            onclick="document.getElementsByID('#text #result').value = ''"
            >Clear</el-button
          >
        </el-row>
        <el-row class="py-1 justify-center">
          <el-button
            onclick="document.getElementById('text').value+='1'; return false;"
            value="1"
          >
            1
          </el-button>
          <el-button
            onclick="document.getElementById('text').value+='2'; return false;"
            value="2"
          >
            2
          </el-button>
          <el-button
            onclick="document.getElementById('text').value+='3'; return false;"
            value="3"
          >
            3
          </el-button>
        </el-row>
        <el-row class="py-1 justify-center">
          <el-button
            onclick="document.getElementById('text').value+='4'; return false;"
            value="4"
          >
            4
          </el-button>
          <el-button
            onclick="document.getElementById('text').value+='5'; return false;"
            value="5"
          >
            5
          </el-button>
          <el-button
            onclick="document.getElementById('text').value+='6'; return false;"
            value="6"
          >
            6
          </el-button>
        </el-row>
        <el-row class="py-1 justify-center">
          <el-button
            onclick="document.getElementById('text').value+='7'; return false;"
            value="7"
          >
            7
          </el-button>
          <el-button
            onclick="document.getElementById('text').value+='8'; return false;"
            value="8"
          >
            8
          </el-button>
          <el-button
            onclick="document.getElementById('text').value+='9'; return false;"
            value="9"
          >
            9
          </el-button>
        </el-row>
        <el-row class="py-1 justify-center">
          <el-button
            onclick="document.getElementById('text').value+='0'; return false;"
            value="0"
          >
            0
          </el-button>
        </el-row>
      </form>
    </el-dialog>
  </div>
</template>

<style scoped></style>
