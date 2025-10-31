import type { BreadcrumbItem } from '$lib/layout/types';
import { writable } from 'svelte/store';

export const breadcrumb = writable<BreadcrumbItem[]>([]);
