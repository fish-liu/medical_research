import { writable } from 'svelte/store';

import type { ChatMessage } from './types/message.type';

// https://sveltebyexample.com/writable-stores/

export const chatMessages = writable<Array<ChatMessage>>([]);

export const defaultMessage = writable<ChatMessage>();