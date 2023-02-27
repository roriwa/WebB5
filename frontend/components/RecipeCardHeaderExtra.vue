<template>
  <div class="relative">
    <div
        class="flex items-center text-stone-400 right-0 child:flex child:items-center child:justify-center">
      <div>
        <IconCSS name="material-symbols:nest-clock-farsight-analog-outline" size="1.2em"/>
        {{ props.recipe.timeRequired }}
      </div>
      <div class="w-1"></div>
      <div>
        <IconCSS name="material-symbols:mode-comment-outline" size="1.2em"/>
        {{ props.recipe.comments.length }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {Recipe} from "~/stores/recipes";

const props = defineProps<{
  recipe: Recipe
}>();
</script>
