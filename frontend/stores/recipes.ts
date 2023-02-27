import {defineStore} from "pinia";
import {jsonRequestOptions} from "~/utils/http";

export const useRecipeStore = defineStore('recipes', () => {
    const recipes = ref<Recipe[]>([]);
    const bookmarks = ref<String[]>([]); // id of bookmarked recipes

    function isBookmarked(recipe: Recipe): boolean {
        return bookmarks.value.includes(recipe.id);
    }

    function getRecipeById(id: string): Recipe | undefined {
        return recipes.value.find(r => r.id === id);
    }

    function getBookmarkedRecipes(): Recipe[] {
        return recipes.value.filter(r => isBookmarked(r));
    }

    async function bookmarkRecipe(recipe: Recipe, bookmark: boolean) {
        const {apiEndpoint} = useAppConfig();
        const authStore = useAuthStore();

        if (!authStore.user) {
            return;
        }

        const {error} = await useFetch(`${apiEndpoint}/bookmark`, {
            ...jsonRequestOptions,
            body: {
                recipe_id: recipe.id,
                bookmarked: bookmark
            },
            headers: {
                Authorization: `Bearer ${authStore.user.sessionToken}`
            }
        });

        if (error.value) {
            console.error("error bookmarking", error.value.data);
            return;
        }

        if (bookmark && !isBookmarked(recipe)) {
            bookmarks.value.push(recipe.id);
        }

        if (!bookmark) {
            bookmarks.value = bookmarks.value.filter(i => i !== recipe.id);
        }
    }

    async function commentRecipe(recipe: Recipe, comment: string) {
        const {apiEndpoint} = useAppConfig();
        const authStore = useAuthStore();

        if (!authStore.user) {
            return;
        }

        const {error} = await useFetch(`${apiEndpoint}/comment`, {
            ...jsonRequestOptions,
            body: {
                recipe_id: recipe.id,
                comment: comment
            },
            headers: {
                Authorization: `Bearer ${authStore.user.sessionToken}`
            }
        });

        if (error.value) {
            console.error("error commenting", error.value.data);
            return;
        }

        await initRecipes();
    }

    async function initRecipes() {
        const {apiEndpoint} = useAppConfig();

        const {data, error} = await useFetch(`${apiEndpoint}/recipes`, {
            ...jsonRequestOptions,
        });

        if (error.value) {
            console.error("error recipe fetching", error.value.data);
            return error.value.data["error"];
        }

        recipes.value = data.value.recipes;
    }

    async function initBookmarks() {
        const {apiEndpoint} = useAppConfig();
        const authStore = useAuthStore();

        const waitForAuth = new Promise<void>((resolve) => {
            const stopWatch = watch(authStore, (_) => {
                stopWatch();
                resolve();
            });
        });

        await waitForAuth;

        if (!authStore.user) {
            return;
        }

        const {data, error} = await useFetch(`${apiEndpoint}/bookmarks`, {
            ...jsonRequestOptions,
            headers: {
                Authorization: `Bearer ${authStore.user.sessionToken}`
            }
        });

        if (error.value) {
            console.error("error bookmarks fetching", error.value.data);
            return error.value.data["error"];
        }

        bookmarks.value = data.value.bookmarks;
    }

    initRecipes();
    initBookmarks();

    return {recipes, isBookmarked, getRecipeById, getBookmarkedRecipes, bookmarkRecipe, commentRecipe};
});

export interface Recipe {
    id: string,
    name: string,
    tags: string[],
    time_required: string,
    summary: string,
    description: string,
    image_key: string,
    ingredients: { amount: string, typ: string }[],
    comments: Comment[]
}

export interface Comment {
    author: string,
    comment: string,
    posted: number
}


if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useRecipeStore, import.meta.hot))
}
