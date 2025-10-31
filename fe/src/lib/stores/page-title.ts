import type { PageTitle } from '$lib/layout/types';
import { writable } from 'svelte/store';

export const pageTitle = writable<PageTitle>({
    title: 'Home',
    desc: 'This is your Home Page',
});
