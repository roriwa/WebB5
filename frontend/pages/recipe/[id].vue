<template>
  <div class="w-full h-full flex flex-col justify-center items-center">
    <RecipeDetail :recipe="recipe" v-if="recipe"/>
    <n-empty description="Rezept nicht gefunden" size="huge" v-else/>
  </div>
</template>

<script setup lang="ts">
import {ComputedRef} from "vue";
import {Recipe} from "~/stores/recipes";

const route = useRoute();
const recipeStore = useRecipeStore();
const lastViewed = useLastViewed();

const recipe: ComputedRef<Recipe | undefined> = computed(() => recipeStore.getRecipeById(route.params.id as string));

onMounted(() => {
  if (recipe.value)
    lastViewed.addLastViewed(recipe.value);
});
</script>
