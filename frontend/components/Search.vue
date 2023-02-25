<template>
  <div class="w-full h-full flex flex-wrap justify-center">
    <div class="w-full text-center">
      <n-h1>Suche nach "{{ props.searchTerm }}"</n-h1>
    </div>
    <RecipeBox v-for="recipe in results" :recipe="recipe" v-if="results.length > 0"/>
    <n-empty size="huge" description="Keine Rezepte gefunden" v-else/>
  </div>
</template>

<script setup lang="ts">
import {ComputedRef} from "vue";
import {Recipe} from "~/stores/recipes";

const props = defineProps<{
  searchTerm: String
}>();

const cleanedSearchTerm: ComputedRef<String> = computed(() => props.searchTerm.startsWith("#") ? props.searchTerm.substring(1) : props.searchTerm);

const results: ComputedRef<Recipe[]> = computed(() => {
  return recipeStore.recipes.filter(r =>
      r.name.toLowerCase().includes(cleanedSearchTerm.value.toLowerCase())
      || r.tags.some(t => t.toLowerCase().includes(cleanedSearchTerm.value.toLowerCase()))
  );
});

const recipeStore = useRecipeStore();
</script>
