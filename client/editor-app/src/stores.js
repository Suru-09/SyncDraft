import { writable } from "svelte/store";
import { persisted } from 'svelte-persisted-store'

export const loggedIn = persisted('loggedIn', false);
export const username = writable("");

export const loggedUser = persisted('loggedUser', {
    "firstName": "",
    "lastName": "",
    "username": "",
    "password": ""
});

export const userDocuments = writable([{
    "_id": "",
    "doc_name": "",
    "doc_owner": "",
    "body": "",
}]);

export const isAnyDocEdited = writable(false);
export const currentEditingDocument = writable({
    "_id": "",
    "doc_name": "",
    "doc_owner": "",
    "body": "",
});