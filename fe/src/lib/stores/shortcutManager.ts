import type { Shortcut } from '$lib/layout/types';
import { writable } from 'svelte/store';

let rootShortcuts: Shortcut[] = [];

let stack: Shortcut[] = [];

const stackStore = writable<Shortcut[]>(stack);

let activeShortcuts: Shortcut[] = [];

const shortcutStore = writable<Shortcut[]>(activeShortcuts);
shortcutStore.subscribe((value) => {
    activeShortcuts = value;
});
stackStore.subscribe((value) => {
    stack = value;
});

export const registeredShortcuts = {
    subscribe: shortcutStore.subscribe,
};

export const shortcutStack = {
    subscribe: stackStore.subscribe,
};

function refresh() {
    shortcutStore.set(stack.length === 0 ? rootShortcuts : activeShortcuts);
    stackStore.set(stack);
}

export function registerShortcut(sc: Shortcut) {
    rootShortcuts = rootShortcuts.filter((s) => s.key !== sc.key);
    rootShortcuts = [...rootShortcuts, sc];
    if (stack.length === 0) {
        activeShortcuts = rootShortcuts;
        shortcutStore.set(activeShortcuts);
    }
}

export function unregisterShortcut(key: string) {
    rootShortcuts = rootShortcuts.filter((s) => s.key !== key);
    refresh();
}

function resetStack() {
    stack = [];
    activeShortcuts = rootShortcuts;
    shortcutStore.set(activeShortcuts);
    stackStore.set(stack);
}

let listenerAdded = false;

function handler(event: KeyboardEvent) {
    const target = event.target as HTMLElement | null;
    if (target) {
        const tag = target.tagName;
        if (
            tag === 'INPUT' ||
            tag === 'TEXTAREA' ||
            tag === 'SELECT' ||
            target.isContentEditable
        ) {
            return;
        }
    }

    const key = event.key;

    if (key === 'Escape' && stack.length > 0) {
        resetStack();
        event.preventDefault();
        return;
    }

    if (key === 'Backspace' && stack.length > 0) {
        stack.pop();
        if (stack.length === 0) {
            activeShortcuts = rootShortcuts;
        } else {
            activeShortcuts = stack[stack.length - 1].children ?? rootShortcuts;
        }
        shortcutStore.set(activeShortcuts);
        stackStore.set(stack);
        event.preventDefault();
        return;
    }

    const list = stack.length === 0 ? rootShortcuts : activeShortcuts;
    const found = list.find((s) => s.key === key);

    if (!found) return;

    event.preventDefault();

    if (found.children) {
        stack.push(found);
        activeShortcuts = found.children;
        shortcutStore.set(activeShortcuts);
        stackStore.set(stack);
    } else {
        found.handler?.(event);
        resetStack();
    }
}

export function initShortcutListener() {
    if (listenerAdded) return;
    listenerAdded = true;
    window.addEventListener('keydown', handler);
}
