<template>
  <div class="w-96">
    <n-card>
      <template #cover>
        <NuxtLink :to="recipeLink">
          <img :src="props.recipe.imageUrl" alt="preview of food"/>
        </NuxtLink>
      </template>
      <template #header>
        <NuxtLink :to="recipeLink">{{ props.recipe.name }}</NuxtLink>
      </template>
      <template #header-extra>
        <RecipeCardHeaderExtra :recipe="props.recipe"/>
      </template>
      <div class="h-32">
        <n-ellipsis :line-clamp="5">
          {{ props.recipe.summary }}
        </n-ellipsis>
      </div>
      <template #footer>
        <div class="flex">
          <div class="w-full">
            <RecipeTags :tags="props.recipe.tags"/>
          </div>
          <div class="ml-auto">
            <RecipeBookmarkIcon :recipe="props.recipe"/>
          </div>
        </div>
      </template>
    </n-card>
  </div>
</template>

<script setup lang="ts">

import {Recipe} from "~/stores/recipes";
import RecipeCardHeaderExtra from "~/components/RecipeCardHeaderExtra.vue";
import RecipeBookmarkIcon from "~/components/RecipeBookmarkIcon.vue";

const props = defineProps<{
  recipe: Recipe
}>();

const recipeStore = useRecipeStore();

const recipeLink = computed(() => `/recipe/${props.recipe.id}`);
</script>
