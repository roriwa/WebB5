<template>
  <div class="w-full h-full flex child:m-4 flex-wrap justify-center">
    <RecipeBox v-for="i in 10" :id="i.toString()" name="Sesam" :bookmarked="false" :stars="4" :comment_count="42"
               time-required="32min" :tags="[
        {
          display: 'Einfach',
          url_format: 'einfach'
        },
        {
          display: 'Sehr Lecker',
          url_format: 'sehr_lecker'
        },
        {
          display: 'Schnell',
          url_format: 'schnell'
        }
    ]" description="dwwd"/>
  </div>
</template>

<script setup>

import RecipeBox from "~/components/RecipeBox.vue";</script>
