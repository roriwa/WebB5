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
            {{ username }}
          </div>
        </div>
      </n-dropdown>
    </template>
  </div>
</template>

<script setup lang="ts">
const loggedInDropdownOptions = [
  {
    label: 'Gespeichert',
    key: 'bookmarked',
    props: {
      onClick: () => {
        router.push("/bookmarked");
      }
    }
  },
  {
    label: 'Logout',
    key: 'logout',
    props: {
      class: "bg-red-500",
      onClick: logout
    }
  }
];

const authStore = useAuthStore();
const router = useRouter();
const username = computed(() => authStore.user?.name ?? "");
const loadingBar = useLoadingBar();

async function logout() {
  loadingBar.start();
  await authStore.logout();
  loadingBar.finish();
}

</script>
