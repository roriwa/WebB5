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
          <n-form>
            <n-form-item path="name" label="Name">
              <n-input @keydown.enter.prevent/>
            </n-form-item>

            <n-form-item path="summary" label="Kurzbeschreibung">
              <n-input type="textarea" @keydown.enter.prevent/>
            </n-form-item>

            <n-form-item path="ingredients" label="Zutaten">
              <n-dynamic-input #="{ index, value }">
                <div class="flex">
                  <n-form-item :path="`ingredients[${index}].count`" :show-label="false" ignore-path-change>
                    <n-input placeholder="Anzahl" @keydown.enter.prevent/>
                  </n-form-item>
                  <n-form-item :path="`ingredients[${index}].type`" :show-label="false" class="ml-4" ignore-path-change>
                    <n-input placeholder="Typ" @keydown.enter.prevent/>
                  </n-form-item>
                </div>
              </n-dynamic-input>
            </n-form-item>

            <n-form-item path="description" label="Zubereitung">
              <n-input type="textarea" @keydown.enter.prevent/>
            </n-form-item>

            <n-form-item path="time_required" label="Benötigte Zeit">
              <n-input-number :step="5" :min="0" size="large" @keydown.enter.prevent>
                <template #prefix>
                  <IconCSS name="material-symbols:nest-clock-farsight-analog-outline"/>
                </template>
                <template #suffix>
                  min
                </template>
              </n-input-number>
            </n-form-item>

            <n-form-item path="tags" label="Tags">
              <n-dynamic-tags :max="4">
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
              <n-button type="primary" size="large">
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

const authStore = useAuthStore();

function onFileUpload(file: SettledFileInfo,
                      fileList: SettledFileInfo[],
                      event: ProgressEvent | Event | undefined) {
  console.log(file);
}

</script>
