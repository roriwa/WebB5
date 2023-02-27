<template>
  <div class="flex justify-center items-center w-full h-full">
    <div class="w-[40rem]">
      <n-card v-if="!authStore.user">
        <n-alert type="error">
          Du musst eingeloggt sein, um neue Rezepte zu erstellen!
        </n-alert>
      </n-card>
      <n-card v-else>
        <template #cover>
          <n-upload :default-upload="false" accept="image/png,image/jpeg" :max="1" @change="onFileUpload">
            <n-upload-dragger>
              <div style="margin-bottom: 12px">
                <IconCSS name="material-symbols:archive-outline" size="4em" class="text-neutral-500"/>
              </div>
              <n-text style="font-size: 16px">
                Klicke oder ziehe eine Datei, um diese zu uploaden
              </n-text>
              <n-p depth="3" style="margin: 8px 0 0 0">
                Es dürfen keine unangemessenen Bilder hochgeladen werden.
                Ebenfalls sind Bilder, auf denen Personen zu sehen sind, nicht erlaubt.
              </n-p>
            </n-upload-dragger>
          </n-upload>
        </template>

        <div>
          <n-h2>Erstelle ein neues Rezept</n-h2>
          <n-form ref="createForm" :model="createFormModel">
            <n-form-item path="name" label="Name">
              <n-input @keydown.enter.prevent v-model:value="createFormModel.name"/>
            </n-form-item>

            <n-form-item path="summary" label="Kurzbeschreibung">
              <n-input type="textarea" @keydown.enter.prevent v-model:value="createFormModel.summary"/>
            </n-form-item>

            <n-form-item path="ingredients" label="Zutaten">
              <n-dynamic-input #="{ index, value }" v-model:value="createFormModel.ingredients"
                               :on-create="() => {return {amount: '', typ: ''}}">
                <div class="flex">
                  <n-form-item :path="`ingredients[${index}].count`" :show-label="false" ignore-path-change>
                    <n-input placeholder="Anzahl" @keydown.enter.prevent
                             v-model:value="createFormModel.ingredients[index].amount"/>
                  </n-form-item>
                  <n-form-item :path="`ingredients[${index}].type`" :show-label="false" class="ml-4" ignore-path-change>
                    <n-input placeholder="Typ" @keydown.enter.prevent
                             v-model:value="createFormModel.ingredients[index].typ"/>
                  </n-form-item>
                </div>
              </n-dynamic-input>
            </n-form-item>

            <n-form-item path="description" label="Zubereitung">
              <n-input type="textarea" @keydown.enter.prevent v-model:value="createFormModel.description"/>
            </n-form-item>

            <n-form-item path="time_required" label="Benötigte Zeit">
              <n-input-number :step="5" :min="0" size="large" @keydown.enter.prevent
                              :on-update-value="(num) => createFormModel.time_required = `${num}min`">
                <template #prefix>
                  <IconCSS name="material-symbols:nest-clock-farsight-analog-outline"/>
                </template>
                <template #suffix>
                  min
                </template>
              </n-input-number>
            </n-form-item>

            <n-form-item path="tags" label="Tags">
              <n-dynamic-tags :max="4" v-model:value="createFormModel.tags">
                <template #trigger="{ activate, disabled }">
                  <n-button
                      size="small"
                      type="primary"
                      dashed
                      :disabled="disabled"
                      @click="activate()"
                  >
                    <template #icon>
                      <n-icon>
                        <IconCSS name="material-symbols:add"/>
                      </n-icon>
                    </template>
                    New Tag
                  </n-button>
                </template>
              </n-dynamic-tags>
            </n-form-item>
            <div class="flex justify-center">
              <n-button type="primary" size="large" attr-type="submit" @click="upload">
                Eintragen
              </n-button>
            </div>
          </n-form>
        </div>
      </n-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import {SettledFileInfo} from "naive-ui/es/upload/src/interface";
import {uploadImage, uploadRecipe, UploadRecipe} from "~/utils/recipe_upload";
import {Ref} from "vue";
import {FormInst} from "naive-ui";

const authStore = useAuthStore();
const recipeStore = useRecipeStore();
const router = useRouter();
const loadingBar = useLoadingBar();
const messaging = useMessage();

const image: Ref<SettledFileInfo | undefined> = ref();
const createForm = ref<FormInst | null>(null);
const createFormModel: Ref<UploadRecipe> = ref({
  name: "",
  summary: "",
  description: "",
  tags: [],
  time_required: "",
  ingredients: []
});

function onFileUpload(file: SettledFileInfo) {
  image.value = file;
}

async function upload() {
  if (!image.value) {
    messaging.error("Bitte lade ein Bild hoch");
    return;
  }

  const promise = new Promise<boolean>((resolve) => {
    createForm.value?.validate((error) => {
      resolve(!!error);
    });
  });

  await promise;

  loadingBar.start();

  const fileKey = await uploadRecipeImage();
  if (fileKey == undefined) {
    messaging.error("Das Bild konnte nicht hochgeladen werden");
    loadingBar.error();
    return;
  }

  const recipe = await uploadRecipe(createFormModel.value, fileKey);
  if (recipe == null) {
    messaging.error("Ein Fehler trat auf");
    loadingBar.error();
    return;
  }

  recipeStore.recipes.push(recipe);

  loadingBar.finish();
  messaging.success("Rezept hochgeladen!");

  await useTimeout(500);
  router.push(`/recipe/${recipe.id}`);

  console.info("Rezept wurde erstellt", recipe);
}

function uploadRecipeImage(): Promise<string | undefined> {
  // ignore error here.
  // idk why the fuck this is as it is, but apparently getting the field "file" from SettledFile is only possible with an extra ".file" ???????????????????
  return uploadImage(image.value!.file!.file);
}

</script>
