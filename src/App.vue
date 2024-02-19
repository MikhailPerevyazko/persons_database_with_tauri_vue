<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import { ref } from "vue";
import { Search } from "@element-plus/icons-vue";

const input = ref("");

import { reactive } from "vue";

// do not use same name with ref
const form = reactive({
  name: "",
  surname: "",
  middleName: "",
  dateOfBirth: "",
  gender: "",
});

// const onSubmit = () => {
//   console.log("submit!");
// };
</script>

<template>
  <div class="border border-blue-500 m-10 items-center w-100">
    <div class="container mx-auto m-10 space-y-4">
      <div class="flex justify-center">
        <h1 class="text-xl">Find person by ID</h1>
      </div>
      <div class="flex justify-center space-x-4 w-auto">
        <el-input v-model="input" placeholder="Please input ID" />
        <el-button type="primary" :icon="Search">Find by ID</el-button>
        <el-button type="primary">Show all info</el-button>
      </div>
    </div>

    <div class="container mx-auto m-10 space-y-4 space-x-4">
      <div class="flex justify-center">
        <h2 class="text-x1">Add new person</h2>
      </div>
      <el-form :model="form" label-width="150px">
        <el-form-item label="Name">
          <el-input v-model="form.name" />
        </el-form-item>
        <div>
          <el-form-item label="Surname">
            <el-input v-model="form.surname" />
          </el-form-item>
        </div>
        <el-form-item label="Middle name">
          <el-input v-model="form.middleName" />
        </el-form-item>
        <el-form-item label="Date of birth">
          <el-input v-model="form.dateOfBirth" />
        </el-form-item>
        <el-form-item label="Gender">
          <el-input v-model="form.gender" />
        </el-form-item>
      </el-form>
      <div class="flex justify-center">
        <el-button type="primary">Add</el-button>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
