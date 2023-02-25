<template>
  <n-space size="small">
    <n-tag :bordered="false" size="small" v-for="tag in props.tags">
      <NuxtLink :to="`/tag/${tag.url_format}`">{{ tag.display }}</NuxtLink>
    </n-tag>
  </n-space>
</template>

<script setup lang="ts">
import Tag from "~/utils/Tag";

const props = defineProps<{
  tags: Tag[]
}>();
</script>
