<template>
  <div>
    <n-h1>Zuletzt besucht</n-h1>


    <n-h1>Alle Rezepte</n-h1>
    <div class="w-full h-full flex child:m-4 flex-wrap justify-center">
      <RecipeBox v-for="i in 10" :id="i.toString()" :recipe="recipeStore.recipes[0]"/>
    </div>
  </div>
</template>

<script setup lang="ts">

import RecipeBox from "~/components/RecipeBox.vue";


const recipeStore = useRecipeStore();
</script>
