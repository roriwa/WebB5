<template>
<span class="cursor-pointer text-stone-400 hover:text-stone-200">
              <IconCSS name="material-symbols:bookmark" size="1.4em" v-if="props.bookmarked"/>
              <IconCSS name="material-symbols:bookmark-outline" size="1.4em" v-else/>
            </span>
</template>

<script setup lang="ts">
const props = defineProps<{
  bookmarked: boolean
}>();
</script>
