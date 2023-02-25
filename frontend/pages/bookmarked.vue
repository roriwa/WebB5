<template>
  <div class="w-full h-full flex flex-wrap justify-center">
    <div class="w-full text-center">
      <n-h1>Deine gespeicherten Rezepte</n-h1>
    </div>

    <RecipeBox v-for="recipe in bookmarkedRecipes" :recipe="recipe" v-if="bookmarkedRecipes.length > 0"/>
    <n-empty size="huge" description="Keine Rezepte gefunden" v-else/>
  </div>
</template>

<script setup lang="ts">
const recipeStore = useRecipeStore();
const bookmarkedRecipes = computed(() => recipeStore.getBookmarkedRecipes());
</script>
