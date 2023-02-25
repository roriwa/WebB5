<template>
  <div class="flex justify-center items-center w-full h-full">
    <div class="w-[40rem]">
      <n-card>
        <div>
          <n-h2>Login</n-h2>
          <n-form :rules="formRules">
            <n-form-item path="username" label="Username">
              <n-input @keydown.enter.prevent placeholder="Nutzername" :maxlength="16" show-count
                       :input-props="{autocomplete: 'username'}"/>
            </n-form-item>

            <n-form-item path="password" label="Passwort">
              <n-input @keydown.enter.prevent placeholder="Passwort" type="password" show-password-on="mousedown"
                       minlength="8" :input-props="{autocomplete: 'current-password'}"/>
            </n-form-item>

            <div class="w-full flex justify-center">
              <n-button type="primary" size="large" attr-type="submit">
                Anmelden
              </n-button>
            </div>
          </n-form>

          <n-divider dashed/>
          <n-h2>Registrierung</n-h2>
          <n-form :rules="formRules">
            <n-form-item path="username" label="Username">
              <n-input @keydown.enter.prevent placeholder="Nutzername" :maxlength="16" show-count
                       :input-props="{autocomplete: 'username'}"/>
            </n-form-item>

            <n-form-item path="password" label="Passwort">
              <n-input @keydown.enter.prevent placeholder="Passwort" type="password" show-password-on="mousedown"
                       minlength="8" :input-props="{autocomplete: 'new-password'}"/>
            </n-form-item>

            <div class="w-full flex justify-center">
              <n-button type="primary" size="large" attr-type="submit">
                Registrieren
              </n-button>
            </div>
          </n-form>
        </div>
      </n-card>
    </div>
  </div>
</template>

<script setup lang="ts">

import {FormItemRule, FormRules} from "naive-ui";

const formRules: FormRules = {
  username: [
    {
      required: true,
      message: 'Nutzername wird benötigt'
    }
  ],
  password: [
    {
      required: true,
      message: 'Passwort wird benötigt'
    },
    {
      validator: validatePasswordLength,
      message: 'Passwort benötigt mindestens 8 Zeichen',
      trigger: ['input']
    }
  ]
}

function validatePasswordLength(_: FormItemRule, value: string | undefined): boolean {
  return (value !== undefined && value.length >= 8);
}
</script>
