<template>
  <div>
    <template v-if="lastViewed.length > 0">
      <n-h1 class="text-center">Zuletzt besucht</n-h1>
      <div class="w-full h-full flex child:m-4 flex-wrap justify-center">
        <RecipeBox v-for="recipe in lastViewed" :recipe="recipe"/>
      </div>
    </template>

    <n-h1 class="text-center">Alle Rezepte</n-h1>
    <div class="w-full h-full flex child:m-4 flex-wrap justify-center">
      <RecipeBox v-for="recipe in recipeStore.recipes" :recipe="recipe"/>
    </div>
  </div>
</template>

<script setup lang="ts">

import RecipeBox from "~/components/RecipeBox.vue";
import {ComputedRef} from "vue";
import {Recipe} from "~/stores/recipes";


const recipeStore = useRecipeStore();
const lastViewedStore = useLastViewed();

const lastViewed: ComputedRef<Recipe[]> = computed(() => {
  const x = lastViewedStore.lastViewedIds.map(id => recipeStore.recipes.find(r => r.id === id));
  return x.filter(r => r);
});
</script>
