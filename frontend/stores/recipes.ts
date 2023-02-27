import {defineStore} from "pinia";

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
        if (bookmark && !isBookmarked(recipe)) {
            bookmarks.value.push(recipe.id);
        }

        if (!bookmark) {
            bookmarks.value = bookmarks.value.filter(i => i !== recipe.id);
        }

        // Todo send to server
    }

    recipes.value = [
        {
            id: "1",
            name: "Sesam",
            tags: [
                "Einfach",
                "Omg",
                "Leeecker",
                "oh no"
            ],
            comments: [{
                author: "Zargor",
                posted: Date.now(),
                comment: "Wow mega gut!"
            },
                {
                    author: "Zargor",
                    posted: Date.now(),
                    comment: "Lorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem"
                }],
            ingredients: [
                {
                    amount: "2ml",
                    type: "Wasser"
                },
                {
                    amount: "20kg",
                    type: "Löwenfleisch"
                },
                {
                    amount: "3x",
                    type: "Merkel"
                }
            ],
            summary: "Lorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum DoloremLorem Ipsum Dolorem",
            timeRequired: "32h",
            description: "Die Milch mit Zucker, Vanillezucker und einer Prise Salz in einem weiten Topf zum Kochen bringen. Den Weizengrieß unter Rühren mit einem Schneebesen einrieseln lassen und nochmals aufkochen lassen. Dann den Topf vom Herd nehmen und den Grieß zugedeckt 5 Minuten ziehen lassen.\n" +
                "\n" +
                "In der Zwischenzeit das Eigelb vom Eiweiß trennen. Das Eiweiß zu steifem Schnee schlagen. Das Eigelb in den Grießbrei rühren. Die Butter ebenfalls in den Grießbrei geben und so lange rühren, bis die Butter geschmolzen ist. Zum Schluss den Eischnee vorsichtig unter den fertigen Grießbrei heben.\n" +
                "\n" +
                "Dazu schmeckt Kompott nach Wahl, Zimt und Zucker oder braune Butter.\n" +
                "\n" +
                "So hat ihn meine Omi gemacht und ich liebe diesen luftig lockeren Grießbrei.",
            imageUrl: "https://img.chefkoch-cdn.de/rezepte/914031196710118/bilder/967316/crop-960x640/griessbrei-von-grossmutter.jpg"
        }
    ]

    return {recipes, isBookmarked, getRecipeById, getBookmarkedRecipes, bookmarkRecipe, rateRecipe};
});

export interface Recipe {
    id: String,
    name: String,
    tags: String[],
    timeRequired: String,
    summary: String,
    description: String,
    imageUrl: String,
    ingredients: { amount: string, type: string }[],
    comments: Comment[]
}

export interface Comment {
    author: String,
    comment: String,
    posted: number
}


if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useRecipeStore, import.meta.hot))
}
