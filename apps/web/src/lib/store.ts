import { writable } from "svelte/store";

const createWritableStore = (key, startValue) => {
    const { subscribe, set } = writable<any>(startValue);

    return {
        subscribe,
        set,
        get: () => {
            const json = localStorage.getItem(key);
            if (json) {
                set(JSON.parse(json));
            }

            subscribe((current) => {
                localStorage.setItem(key, JSON.stringify(current));
            });

            return json;
        },
        useLocalStorage: () => {
            const json = localStorage.getItem(key);
            if (json) {
                set(JSON.parse(json));
            }

            subscribe((current) => {
                localStorage.setItem(key, JSON.stringify(current));
            });
        },
    };
};

export const user = createWritableStore("user", {});
