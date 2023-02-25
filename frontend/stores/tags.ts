import {Ref} from "vue";

export const useTagStore = defineStore('tags', () => {
    const tags: Ref<String[]> = ref([])

    return {tags};
});

if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useTagStore, import.meta.hot))
}
