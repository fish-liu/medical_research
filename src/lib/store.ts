import { writable } from 'svelte/store';

import type { Message4Chat } from './types/message.type';

// https://sveltebyexample.com/writable-stores/

// 聊天类型，1= 共情， 0= 低共情
export const chatType = writable<string>('')

export const chatMessages = writable<Array<Message4Chat>>([]);

export const defaultMessage = writable<Message4Chat>();