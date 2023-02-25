<template>
  <div>
    <n-space vertical>
      <n-empty size="medium" description="Das Rezept wurde noch nicht kommentiert"
               v-if="props.recipe.comments.length === 0"/>
      <RecipeSingleComment v-for="comment in props.recipe.comments" :comment="comment" :key="comment" v-else/>
    </n-space>

    <n-h3>Kommentiere selber!</n-h3>

    <n-alert title="Du musst eingeloggt sein, um zu kommentieren" type="error" v-if="!authStore.user"/>
    <div v-else>
      <n-space vertical>
        <n-input placeholder="Dein Kommentar" type="textarea"/>
        <n-button type="primary">Kommentieren</n-button>
      </n-space>
    </div>
  </div>
</template>

<script setup lang="ts">
import {Recipe} from "~/stores/recipes";
import RecipeSingleComment from "~/components/RecipeSingleComment.vue";

const props = defineProps<{
  recipe: Recipe
}>();

const authStore = useAuthStore();
</script>
