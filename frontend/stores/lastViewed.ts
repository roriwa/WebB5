import {defineStore} from "pinia";
import {Ref} from "vue";
import {Recipe} from "~/stores/recipes";

export const useLastViewed = defineStore('lastViewed', () => {
    const lastViewedIds: Ref<string[]> = ref([]);
    const lastViewedCookie = useCookie("lastViewed", {default: () => ""});

    function addLastViewed(recipe: Recipe) {
        if (lastViewedIds.value.includes(recipe.id))
            return;

        lastViewedIds.value.push(recipe.id);

        if (lastViewedIds.value.length > 3)
            lastViewedIds.value.shift();

        lastViewedCookie.value = lastViewedIds.value.join(";");
    }

    function initLastViewed() {
        lastViewedIds.value = lastViewedCookie.value?.split(";") ?? [];
    }

    initLastViewed();

    return {lastViewedIds, addLastViewed};
});

if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useLastViewed, import.meta.hot))
}
