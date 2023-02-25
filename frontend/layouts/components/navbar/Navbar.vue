<template>
  <div>
    <div class="mx-8 lg:mx-32 py-4 flex lg:items-center child:flex-1">
      <NuxtLink to="/">
        <div class="flex items-center">

          <Icon name="twemoji:beer-mug" size="2.5rem"/>
          <h1 class="font-extrabold text-2xl ml-1 text-white">Food Finder</h1>
        </div>
      </NuxtLink>
      <div class="hidden lg:block">
        <NavbarSearch/>

      </div>
      <div class="flex flex-col lg:flex-row">
        <button class="lg:hidden ml-auto" @click="mobileShowMenu = !mobileShowMenu">
          <Icon name="ion:menu" size="3em"/>
        </button>
        <div class="hidden lg:flex ml-auto items-center justify-center">
          <NavbarCreate class="mr-8"/>
          <NavbarUser/>
        </div>
      </div>
    </div>
    <div class="lg:hidden flex flex-col items-center child:mb-2" :class="mobileShowMenu ? '' : 'hidden'">
      <NavbarCreate/>
      <NavbarUser/>
      <NavbarSearch/>
    </div>
  </div>
</template>

<script setup>

import NavbarUser from "~/layouts/components/navbar/NavbarUser.vue";
import NavbarSearch from "~/layouts/components/navbar/NavbarSearch.vue";
import NavbarCreate from "~/layouts/components/navbar/NavbarCreate.vue";

const mobileShowMenu = ref(false);
</script>
