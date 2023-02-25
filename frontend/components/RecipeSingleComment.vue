<template>
  <div class="w-full">
    <div>
      {{ props.comment.author }} <span class="text-stone-400">am</span>
      <n-time :to="props.comment.posted" format="dd.MM.yyyy hh:mm"/>
    </div>
    <div class="ml-4">
      {{ props.comment.comment }}
    </div>
  </div>
</template>

<script setup lang="ts">
import {Comment} from "~/stores/recipes";

const props = defineProps<{
  comment: Comment
}>();
</script>
