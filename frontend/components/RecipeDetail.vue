<template>
  <div class="max-w-[44rem]">
    <n-card>
      <template #cover>
        <img :src="props.recipe.imageUrl" alt="preview of food"/>
      </template>
      <template #header>
        <div class="relative">
          <n-h1>{{ props.recipe.name }}</n-h1>
          <RecipeBookmarkIcon :bookmarked="recipeStore.isBookmarked(props.recipe)"/>
          <div class="absolute top-10">
            <RecipeTags :tags="props.recipe.tags"/>
          </div>
        </div>
      </template>
      <template #header-extra>
        <RecipeCardHeaderExtra :recipe="props.recipe" rateable/>
      </template>

      <div>
        <n-h2>Kurzbeschreibung</n-h2>
        {{ props.recipe.summary }}

        <n-divider/>

        <n-h2>Zutaten</n-h2>

        <n-list bordered>
          <n-list-item v-for="ingredient in props.recipe.ingredients">
            <div class="flex">
              <div class="w-12">{{ ingredient.amount }}</div>
              <n-divider vertical/>
              <div class="ml-2">{{ ingredient.type }}</div>
            </div>
          </n-list-item>
        </n-list>

        <n-divider/>

        <n-h2>Zubereitung</n-h2>
        {{ props.recipe.description }}

        <n-divider/>

        <n-h2>Kommentare</n-h2>
        <RecipeComments :recipe="recipe"/>
      </div>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import {Recipe} from "~/stores/recipes";
import RecipeComments from "~/components/RecipeComments.vue";

const props = defineProps<{
  recipe: Recipe
}>();

const recipeStore = useRecipeStore();
const isBookmarked = computed(() => recipeStore.isBookmarked(props.recipe));
</script>
