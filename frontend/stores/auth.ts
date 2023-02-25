export const useAuthStore = defineStore('user', () => {
    const user = ref<Auth | null>(null);

    function login() {
        user.value = {name: "Zargor"};
    }

    return {user, login}
});

export interface Auth {
    name: String
}

if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useAuthStore, import.meta.hot))
}
