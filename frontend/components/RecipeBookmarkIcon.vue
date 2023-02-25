<template>
<span class="text-stone-400" :class="authStore.user ? 'cursor-pointer hover:text-stone-200' : ''" @click="bookmark">
              <IconCSS name="material-symbols:bookmark" size="1.4em" v-if="isBookmarked"/>
              <IconCSS name="material-symbols:bookmark-outline" size="1.4em" v-else/>
            </span>
</template>

<script setup lang="ts">
import {Recipe} from "~/stores/recipes";
import {ComputedRef} from "vue";

const authStore = useAuthStore();
const recipeStore = useRecipeStore();
const props = defineProps<{
  recipe: Recipe
}>();
const isBookmarked: ComputedRef<boolean> = computed(() => recipeStore.isBookmarked(props.recipe));

async function bookmark() {
  if (authStore.user)
    await recipeStore.bookmarkRecipe(props.recipe, !isBookmarked.value);
}
</script>
