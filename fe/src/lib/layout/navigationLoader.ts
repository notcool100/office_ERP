import { navigationService, type UserNavigationItem } from '$lib/services/navigation';
import { navigationStore, navigationLoading, navigationError } from '$lib/stores/navigation';
import type { MenuItem } from '$lib/layout/types';
import { get } from 'svelte/store';

// Icon mapping from backend to frontend (lucide-svelte components)
import {
    Calendar,
    CalendarHeart,
    CalendarMinus,
    ChartBarStacked,
    ChartColumn,
    ChartColumnDecreasing,
    ChartColumnIncreasing,
    ChartPie,
    CircleUser,
    ClipboardList,
    Clock,
    File,
    FileText,
    FileX,
    LayoutDashboard,
    List,
    Package,
    Plus,
    Receipt,
    Settings,
    Shield,
    ShoppingCart,
    SquareUser,
    UserCheck,
    UserCog,
    UserPlus,
    Users,
    Building2,
    Briefcase,
    Menu,
    ShieldCheck,
} from 'lucide-svelte';

const iconMap: Record<string, any> = {
    LayoutDashboard,
    UserCog,
    Settings,
    ShoppingCart,
    ChartColumn,
    Users,
    ChartPie,
    CircleUser,
    CalendarHeart,
    Shield,
    List,
    Plus,
    UserPlus,
    Package,
    Calendar,
    ClipboardList,
    FileText,
    Receipt,
    File,
    FileX,
    UserCheck,
    SquareUser,
    CalendarMinus,
    Clock,
    ChartColumnIncreasing,
    ChartColumnDecreasing,
    ChartBarStacked,
    Building2,
    Briefcase,
    Menu,
    ShieldCheck,
};

function mapNavigationToMenuItem(navItem: UserNavigationItem): MenuItem {
    const hasChildren = navItem.children && navItem.children.length > 0;

    const menuItem: MenuItem = {
        name: navItem.name,
        type: hasChildren ? 'folder' : 'file',
        icon: iconMap[navItem.icon || 'File'] || File,
        color: 'text-primary',
        url: hasChildren ? undefined : navItem.path,
        expanded: hasChildren,
    };

    if (hasChildren) {
        menuItem.children = navItem.children.map(mapNavigationToMenuItem);
    }

    return menuItem;
}

export async function loadUserNavigation(): Promise<MenuItem[]> {
    navigationLoading.set(true);
    navigationError.set(null);

    try {
        const navItems = await navigationService.getUserNavigation();
        navigationStore.set(navItems);

        // Convert to MenuItem format for the sidebar
        const menuItems = navItems
            .filter(item => item.can_read) // Only show items user can read
            .map(mapNavigationToMenuItem);

        return menuItems;
    } catch (error) {
        const errorMessage = error instanceof Error ? error.message : 'Failed to load navigation';
        navigationError.set(errorMessage);
        console.error('Failed to load navigation:', error);
        return [];
    } finally {
        navigationLoading.set(false);
    }
}

// Fallback menu for when not logged in or API fails
export const fallbackMenu: MenuItem[] = [
    {
        name: 'Dashboard',
        type: 'file',
        icon: LayoutDashboard,
        color: 'text-primary',
        url: '/admin/dashboard',
    },
];
