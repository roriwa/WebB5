<template>
  <n-config-provider
      :theme="darkTheme"
      :inline-theme-disabled="true"
      :locale="deDE"
      :date-locale="dateDeDE"
      class="flex basis-full flex-grow"
  >
    <n-loading-bar-provider>
      <n-message-provider placement="bottom-right">
        <n-notification-provider>
          <n-dialog-provider>
            <nuxt-layout name="custom"/>
          </n-dialog-provider>
        </n-notification-provider>
      </n-message-provider>
    </n-loading-bar-provider>
  </n-config-provider>
</template>

<script setup>
import {darkTheme, dateDeDE, deDE} from "naive-ui"</script>
