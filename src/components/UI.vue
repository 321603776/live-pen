<script lang="ts" setup>
const palette = ["#7855d4", "#359ec9", "#38e2a8", "#86e238", "#e2cf38", "#e27f38", "#df2828", "#e2386d", "#d570c2", "#d6d6d6", "#7e7e7e"]
</script>

<template>
  <div class="absolute">
    <div class="flex gap-2">
      <div v-for="item in palette" :key="item" class="size-6 rounded-full" :style="`background-color: ${item};`"></div>
    </div>
    <div></div>
  </div>
</template>
<style scoped></style>