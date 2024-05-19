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

export const userDocuments = persisted('userDocuments', [{
    "_id": "",
    "doc_name": "",
    "doc_owner": "",
    "body": "",
}]);

export const usersList = writable([""]);

export const isAnyDocEdited = persisted('isAnyDocEdited',false);
export const connectedToSession = persisted('connectedToSession',false);
export const isSessionStarted = persisted('isSessionStarted',false);

export const currentEditingDocument = persisted('currentEditingDocument', {
    "_id": "",
    "doc_name": "",
    "doc_owner": "",
    "body": "",
});