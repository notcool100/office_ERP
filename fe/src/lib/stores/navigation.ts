import { writable } from 'svelte/store';
import type { UserNavigationItem } from '$lib/services/navigation';

export const navigationStore = writable<UserNavigationItem[]>([]);
export const navigationLoading = writable<boolean>(false);
export const navigationError = writable<string | null>(null);

// Permission helpers
export function canCreate(path: string, nav: UserNavigationItem[]): boolean {
    return hasPermission(path, nav, 'can_create');
}

export function canRead(path: string, nav: UserNavigationItem[]): boolean {
    return hasPermission(path, nav, 'can_read');
}

export function canUpdate(path: string, nav: UserNavigationItem[]): boolean {
    return hasPermission(path, nav, 'can_update');
}

export function canDelete(path: string, nav: UserNavigationItem[]): boolean {
    return hasPermission(path, nav, 'can_delete');
}

function hasPermission(
    path: string,
    nav: UserNavigationItem[],
    permission: keyof Pick<UserNavigationItem, 'can_create' | 'can_read' | 'can_update' | 'can_delete'>
): boolean {
    for (const item of nav) {
        if (item.path === path) {
            return item[permission];
        }
        if (item.children && item.children.length > 0) {
            const childPermission = hasPermission(path, item.children, permission);
            if (childPermission) return true;
        }
    }
    return false;
}
