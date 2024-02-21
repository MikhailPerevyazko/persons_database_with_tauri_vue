<script setup lang="ts">
import { ref } from "vue";
import { ArrowDown } from "@element-plus/icons-vue";

import ShowInfoButton from "./components/Buttons/ShowInfoButton.vue";
import FindByIdButton from "./components/Buttons/FindByIdButton.vue";
import AddInfoButton from "./components/Buttons/AddInfoButton.vue";
import DeleteButton from "./components/Buttons/DeleteButton.vue";
import InputDataLine from "./components/Inputs/InputDataLine.vue";
import SelectAFileButton from "./components/Buttons/SelectAFileButton.vue";

const input = ref("");
</script>

<template>
  <div class="m-10 items-center w-auto border">
    <h1 class="flex justify-center text-x1 mb-5">Find person by ID</h1>
    <div class="flex justify-center space-x-4 w-auto">
      <SelectAFileButton></SelectAFileButton>
      <el-input v-model="input" placeholder="Please input ID:" />
      <FindByIdButton></FindByIdButton>
      <DeleteButton></DeleteButton>
    </div>

    <div id="add" class="flex justify-center container m-5">
      <el-dropdown>
        <el-button class="mr-5" type="primary">
          Add person<el-icon class="el-icon--right"><arrow-down /></el-icon>
        </el-button>
        <template #dropdown class="w-screen">
          <el-dropdown-menu>
            <InputDataLine bg-color="black"></InputDataLine>
            <AddInfoButton></AddInfoButton>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
      <ShowInfoButton></ShowInfoButton>
    </div>
  </div>
</template>
