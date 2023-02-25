<template>
  <n-dropdown trigger="click" :options="loggedInDropdownOptions">
    <div class="flex items-center cursor-pointer">
      <div class="rounded-full p-2 bg-black">
        <Icon name="ph:user-fill" size="2em"/>
      </div>
      <div class="ml-2 font-bold">
        Zargor
      </div>
    </div>
  </n-dropdown>
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
