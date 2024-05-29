import {writable} from "svelte/store"
export const questions = writable([])
export const selected_context = writable(-1)
export const form_metadata = writable([])
// either form_metadata or form_question
export const editor_context = writable("form_question")