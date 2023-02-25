<template>
  <n-space size="small">
    <n-tag :bordered="false" size="small" v-for="tag in props.tags">
      <NuxtLink :to="`/search/%23${encodeURI(`${tag}`)}`">{{ tag }}</NuxtLink>
    </n-tag>
  </n-space>
</template>

<script setup lang="ts">
const props = defineProps<{
  tags: String[]
}>();
</script>
