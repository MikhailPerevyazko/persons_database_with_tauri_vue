<script setup lang="ts">
import { onMounted } from "vue";
import { reactive } from "vue";

onMounted(() => {});

const form = reactive({
  name: "",
  surname: "",
  middleName: "",
  dateOfBirth: "",
  gender: "",
});
</script>

<template>
  <el-form :model="form" label-width="120px ">
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
</template>

<style scoped></style>
