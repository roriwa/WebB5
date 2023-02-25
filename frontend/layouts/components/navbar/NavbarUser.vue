<template>
  <div>
    <template v-if="authStore.user === null">
      <NuxtLink to="/login">Login</NuxtLink>
    </template>
    <template v-else>
      <n-dropdown trigger="click" :options="loggedInDropdownOptions">
        <div class="flex items-center cursor-pointer">
          <div class="rounded-full p-2 bg-black">
            <Icon name="ph:user-fill" size="2em"/>
          </div>
          <div class="ml-2 font-bold">
            {{ authStore.user.name }}
          </div>
        </div>
      </n-dropdown>
    </template>
  </div>
</template>

<script setup lang="ts">
import {useAuthStore} from "~/stores/auth";

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

const authStore = useAuthStore();

</script>
