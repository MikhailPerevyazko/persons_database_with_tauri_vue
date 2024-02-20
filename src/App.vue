<script setup lang="ts">
import { ref } from "vue";
import ShowInfoButton from "./components/Buttons/ShowInfoButton.vue";
import FindByIdButton from "./components/Buttons/FindByIdButton.vue";
import AddInfoButton from "./components/Buttons/AddInfoButton.vue";
import DeleteButton from "./components/Buttons/DeleteButton.vue";
import InputDataLine from "./components/Inputs/InputDataLine.vue";

const input = ref("");
</script>

<template>
  <div class="m-10 items-center">
    <h1 class="flex justify-center text-x1 mb-5">Find person by ID</h1>
    <div class="flex justify-center space-x-4 w-auto">
      <el-input v-model="input" placeholder="Please input ID:" />
      <FindByIdButton></FindByIdButton>
      <ShowInfoButton></ShowInfoButton>
      <DeleteButton></DeleteButton>
    </div>

    <div class="container mx-auto m-10 space-y-4 space-x-4">
      <h2 class="text-x2 mb-5 flex justify-center">Add new person</h2>
      <InputDataLine bg-color="black"></InputDataLine>
      <AddInfoButton></AddInfoButton>
    </div>
  </div>
</template>
