import { writable } from "svelte/store";
import { persisted } from 'svelte-persisted-store'

export const loggedIn = persisted('loggedIn', false);
export const username = writable("");
export const loggedUser = persisted('loggedUser', {
    "firstName": "",
    "lastName": "",
    "email": "",
    "password": ""
})