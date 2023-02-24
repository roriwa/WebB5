<template>
  <div class="bg-black w-screen h-screen text-white">
    <n-layout content-style="display: flex; flex-direction: column; width: 100vw; height: 100vh;">
      <n-layout-header>
        <Navbar/>
      </n-layout-header>
      <n-layout-content>
        <!--suppress JSUnusedGlobalSymbols -->
        <NuxtPage
            :transition="{
                        name: 'page',
                        mode: 'out-in',
                        onEnterCancelled: () => load.error(),
                        onLeaveCancelled: () => load.error(),
                        onBeforeEnter: () => load.finish(),
                        onBeforeLeave: () => load.start(),
                    }"
        />
      </n-layout-content>
      <n-layout-footer>
        <Footer/>
      </n-layout-footer>
    </n-layout>
  </div>
</template>

<script setup>
import Navbar from "~/layouts/components/Navbar.vue";
import Footer from "~/layouts/components/Footer.vue";

const load = useLoadingBar()

</script>


<style>
.page-enter-from {
  opacity: 0;
}

.page-enter-active {
  transition: all 0.3s ease-out;
}

.page-leave-to {
  opacity: 0;
}

.page-leave-active {
  transition: all 0.3s ease-in;
}
</style>
