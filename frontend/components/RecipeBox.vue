<template>
  <div class="w-96">
    <n-card>
      <template #cover>
        <NuxtLink :to="recipeLink">
          <img
              src="https://image.stern.de/30390974/t/tC/v3/w1440/r1.7778/-/sesamkringel.jpg"/>
        </NuxtLink>
      </template>
      <template #header>
        <NuxtLink :to="recipeLink">{{ props.name }}</NuxtLink>
      </template>
      <template #header-extra>
        <div class="relative">
          <n-rate readonly :value="props.stars"/>

          <div
              class="flex items-center text-stone-400 absolute right-0 child:flex child:items-center child:justify-center">
            <div>
              <IconCSS name="material-symbols:nest-clock-farsight-analog-outline" size="1.2em"/>
              {{ props.timeRequired }}
            </div>
            <div class="w-1"></div>
            <div>
              <IconCSS name="material-symbols:mode-comment-outline" size="1.2em"/>
              {{ props.comment_count }}
            </div>
          </div>
        </div>
      </template>
      Lorem Ipsum Lorem Ipsum Lorem Ipsum Lorem Ipsum Lorem Ipsum Lorem Ipsum Lorem Ipsum Lorem Ipsum Lorem Ipsum
      Lorem
      Ipsum Lorem Ipsum Lorem Ipsum Lorem Ipsum Lorem Ipsum Lorem Ipsum
      Lorem Ipsum
      Lorem Ipsum
      Lorem Ipsum Lorem Ipsum Lorem Ipsum Lorem Ipsum Lorem Ipsum

      Lorem Ipsum


      Lorem Ipsum

      {{ props.description }}


      <template #footer>
        <div class="flex">
          <div>
            <RecipeTags :tags="props.tags"/>
          </div>
          <div class="ml-auto">
            <span class="cursor-pointer text-stone-400 hover:text-stone-200">
              <IconCSS name="material-symbols:bookmark-outline" size="1.4em" v-if="!props.bookmarked"/>
              <IconCSS name="material-symbols:bookmark" size="1.4em" v-if="props.bookmarked"/>
            </span>
          </div>
        </div>
      </template>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import {Tag} from "~/utils/Tag";

const props = defineProps<{
  id: String,
  name: String,
  tags: Tag[],
  stars: number,
  comment_count: number,
  bookmarked: boolean,
  timeRequired: String,
  description: String,
}>();

const recipeLink = computed(() => `/recipe/${props.id}`);
</script>
