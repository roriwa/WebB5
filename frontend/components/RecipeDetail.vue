<template>
  <div class="max-w-[44rem]">
    <n-card>
      <template #cover>
        <div class="h-96">
          <img :src="get_recipe_image_url(props.recipe.image_key)" alt="preview of food"/>
        </div>
      </template>
      <template #header>
        <div class="relative">
          <n-h1>{{ props.recipe.name }}</n-h1>
          <RecipeBookmarkIcon :recipe="props.recipe"/>
          <div class="absolute top-10">
            <RecipeTags :tags="props.recipe.tags"/>
          </div>
        </div>
      </template>
      <template #header-extra>
        <RecipeCardHeaderExtra :recipe="props.recipe"/>
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
              <div class="ml-2">{{ ingredient.typ }}</div>
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
import {get_recipe_image_url} from "~/utils/http";

const props = defineProps<{
  recipe: Recipe
}>();

const recipeStore = useRecipeStore();
const authStore = useAuthStore();
</script>
