import {jsonRequestOptions} from "~/utils/http";

export const useAuthStore = defineStore('user', () => {
    const user = ref<Auth | null>(null);


    const sessionCookie = useCookie("session");

    // returns the error or null
    async function login(name: string, password: string, register: boolean): Promise<string | null> {
        const {apiEndpoint} = useAppConfig();
        const {data, error} = await useFetch(`${apiEndpoint}/${register ? 'register' : 'login'}`, {
            ...jsonRequestOptions,
            body: {
                username: name,
                password
            },

        });

        if (error.value) {
            console.error("error login:", error.value.data);
            return error.value.data["error"];
        }

        sessionCookie.value = data.value.session_token;
        user.value = {name: data.value.username, sessionToken: data.value.session_token};

        return null;
    }

    async function logout() {
        if (!user.value)
            return;

        const {apiEndpoint} = useAppConfig();

        await useFetch(`${apiEndpoint}/logout`, {
            ...jsonRequestOptions,
            headers: {
                Authorization: `Bearer ${user.value.sessionToken}`
            }
        });

        user.value = null;
    }

    async function loginBySessionCookie() {
        const {apiEndpoint} = useAppConfig();
        if (!sessionCookie.value)
            return;

        const {data, error} = await useFetch(`${apiEndpoint}/whois`, {
            ...jsonRequestOptions,
            headers: {
                Authorization: `Bearer ${sessionCookie.value}`
            }
        });

        if (error.value)
            return;


        user.value = {name: data.value.username, sessionToken: sessionCookie.value};
    }


    loginBySessionCookie();

    return {user, login, logout}
});

export interface Auth {
    name: String,
    sessionToken: String
}

if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useAuthStore, import.meta.hot))
}
