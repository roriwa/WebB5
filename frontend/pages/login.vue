<template>
  <div class="flex justify-center items-center w-full h-full">
    <div class="w-[40rem]">
      <n-card>
        <div>
          <n-h2>Login</n-h2>
          <n-form :rules="formRules" :model="loginFormModel" ref="loginForm">
            <n-form-item path="username" label="Username">
              <n-input @keydown.enter.prevent placeholder="Nutzername" :maxlength="16" show-count
                       :input-props="{autocomplete: 'username'}" v-model:value="loginFormModel.username"
                       :allow-input="noSpace"/>
            </n-form-item>

            <n-form-item path="password" label="Passwort">
              <n-input @keydown.enter.prevent placeholder="Passwort" type="password" show-password-on="mousedown"
                       minlength="8" :input-props="{autocomplete: 'current-password'}"
                       v-model:value="loginFormModel.password"
                       :allow-input="noSpace"/>
            </n-form-item>

            <div class="w-full flex justify-center">
              <n-button type="primary" size="large" attr-type="submit" @click="login(false)">
                Anmelden
              </n-button>
            </div>
          </n-form>

          <n-divider dashed/>
          <n-h2>Registrierung</n-h2>
          <n-form :rules="formRules" :model="registerFormModel" ref="registerForm">
            <n-form-item path="username" label="Username">
              <n-input @keydown.enter.prevent placeholder="Nutzername" :maxlength="16" show-count
                       :input-props="{autocomplete: 'username'}" v-model:value="registerFormModel.username"
                       :allow-input="noSpace"/>
            </n-form-item>

            <n-form-item path="password" label="Passwort">
              <n-input @keydown.enter.prevent placeholder="Passwort" type="password" show-password-on="mousedown"
                       minlength="8" :input-props="{autocomplete: 'new-password'}"
                       v-model:value="registerFormModel.password"
                       :allow-input="noSpace"/>
            </n-form-item>

            <div class="w-full flex justify-center">
              <n-button type="primary" size="large" attr-type="submit" @click="login(true)">
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
import {FormInst, FormItemRule, FormRules} from "naive-ui";
import {Ref} from "vue";
import {promiseTimeout} from "@vueuse/core";

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

const loginForm = ref<FormInst | null>(null)
const loginFormModel: Ref<{ username: string, password: string }> = ref({username: "", password: ""});

const registerForm = ref<FormInst | null>(null)
const registerFormModel: Ref<{ username: string, password: string }> = ref({username: "", password: ""});

function validatePasswordLength(_: FormItemRule, value: string | undefined): boolean {
  return (value !== undefined && value.length >= 8);
}

function noSpace(value: string): boolean {
  return !/ /g.test(value);
}

const authStore = useAuthStore();
const loadingBar = useLoadingBar();
const messaging = useMessage();
const router = useRouter();

async function login(register: boolean) {
  const promise = new Promise<boolean>((resolve) => {
    if (!register) {
      loginForm.value?.validate((error) => {
        resolve(!!error);
      });
    } else {
      registerForm.value?.validate((error) => {
        resolve(!!error);
      });
    }
  });

  if (await promise)
    return;

  loadingBar.start();

  const formValue = register ? registerFormModel.value : loginFormModel.value;

  const loginResult = await authStore.login(formValue.username, formValue.password, register);

  if (loginResult == null) {
    loadingBar.finish();
    messaging.success("Erfolgreich eingeloggt");
    await promiseTimeout(2000);
    router.push("/");
  } else {
    loadingBar.error();
    messaging.error(`Fehler: ${loginResult}. Bitte versuche es erneut`);
  }
}
</script>
