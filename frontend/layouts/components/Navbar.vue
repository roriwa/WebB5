<template>
  <div class="mx-24 py-4 flex items-center child:flex-1 flex-col lg:flex-row child:my-1">
    <NuxtLink to="/">
      <div class="flex items-center">

        <Icon name="twemoji:beer-mug" size="2.5rem"/>
        <h1 class="font-extrabold text-2xl ml-1 text-white">Food Recipes</h1>
      </div>
    </NuxtLink>
    <div>
      <n-input
          placeholder="Suche"
          round
          clearable
          attr-size=25
      >
        <template #prefix>
          <Icon name="material-symbols:search" size="1.5em"/>
        </template>
      </n-input>
    </div>
    <div class="flex">
      <n-dropdown trigger="click" :options="loggedInDropdownOptions">
        <div class="ml-auto flex items-center cursor-pointer">
          <div class="rounded-full p-2 bg-black">
            <Icon name="ph:user-fill" size="2em"/>
          </div>
          <div class="ml-2 font-bold">
            Zargor
          </div>
        </div>
      </n-dropdown>
    </div>
  </div>
</template>

<script setup>
const loggedInDropdownOptions = [
  {
    label: 'Favoriten',
    key: 'favs',
    props: {
      onClick: () => {

      }
    }
  },
  {
    label: 'Logout',
    key: 'logout',
    props: {
      class: "bg-red-500"
    }
  }
];
</script>
