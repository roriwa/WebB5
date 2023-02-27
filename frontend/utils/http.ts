import {UseFetchOptions} from "#app/composables/fetch";

export const jsonRequestOptions: UseFetchOptions<any> = {
    method: "post",
    headers: {
        "content-type": "application/json"
    },
};
