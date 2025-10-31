import { goto } from '$app/navigation';
import {
    ArrowLeft,
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
    EyeOff,
    File,
    FileText,
    FileX,
    Fullscreen,
    HelpCircle,
    LayoutDashboard,
    Link,
    Package,
    Receipt,
    Search,
    Settings,
    Shield,
    ShoppingCart,
    SquareUser,
    UserCheck,
    UserCog,
    UserPlus,
    Users,
} from 'lucide-svelte';
import type { Shortcut } from './types';

export const adminShortcuts: Shortcut[] = [
    {
        key: 'g',
        action: 'Go To',
        icon: Link,
        children: [
            {
                key: 'd',
                action: 'Dashboard',
                icon: LayoutDashboard,
                handler: () => goto('/admin/dashboard'),
            },
            {
                key: 'p',
                action: 'Profile',
                icon: UserCog,
                children: [
                    {
                        key: 'i',
                        action: 'Information',
                        icon: CircleUser,
                        handler: () => goto('/admin/profile/information'),
                    },
                    {
                        key: 's',
                        action: 'Schedule',
                        icon: CalendarHeart,
                        handler: () => goto('/admin/profile/schedule'),
                    },
                ],
            },
            {
                key: 's',
                action: 'Settings',
                icon: Settings,
                children: [
                    {
                        key: 'r',
                        action: 'Role',
                        icon: Shield,
                        children: [
                            {
                                key: 'l',
                                action: 'List',
                                icon: Shield,
                                handler: () => goto('/admin/settings/role'),
                            },
                            {
                                key: 'a',
                                action: 'Add',
                                icon: UserPlus,
                                handler: () =>
                                    goto('/admin/settings/role/create'),
                            },
                        ],
                    },
                    {
                        key: 'c',
                        action: 'Contact',
                        icon: UserPlus,
                        children: [
                            {
                                key: 'l',
                                action: 'List',
                                icon: UserPlus,
                                handler: () => goto('/admin/settings/contact'),
                            },
                            {
                                key: 'a',
                                action: 'Add Contact',
                                icon: UserPlus,
                                handler: () =>
                                    goto('/admin/settings/contact/create'),
                            },
                        ],
                    },
                    {
                        key: 'p',
                        action: 'Product',
                        icon: Package,
                        handler: () => goto('/admin/settings/product'),
                    },
                    {
                        key: 'a',
                        action: 'Calendar',
                        icon: Calendar,
                        handler: () => goto('/admin/settings/calendar'),
                    },
                ],
            },
            {
                key: 'u',
                action: 'Purchase',
                icon: ShoppingCart,
                children: [
                    {
                        key: 'o',
                        action: 'Purchase Order',
                        icon: ClipboardList,
                        handler: () => goto('/admin/purchase/order'),
                    },
                    {
                        key: 'b',
                        action: 'Bill',
                        icon: FileText,
                        handler: () => goto('/admin/purchase/bill'),
                    },
                    {
                        key: 'r',
                        action: 'Receipt Voucher',
                        icon: Receipt,
                        handler: () => goto('/admin/purchase/receipt'),
                    },
                ],
            },
            {
                key: 'a',
                action: 'Sales',
                icon: ChartColumn,
                children: [
                    {
                        key: 'q',
                        action: 'Quotation',
                        icon: File,
                        handler: () => goto('/admin/sales/quotation'),
                    },
                    {
                        key: 'i',
                        action: 'Invoice',
                        icon: FileX,
                        handler: () => goto('/admin/sales/invoice'),
                    },
                    {
                        key: 'p',
                        action: 'Payment Voucher',
                        icon: FileText,
                        handler: () => goto('/admin/sales/payment'),
                    },
                ],
            },
            {
                key: 'h',
                action: 'HR',
                icon: Users,
                children: [
                    {
                        key: 'e',
                        action: 'Employee',
                        icon: UserCheck,
                        handler: () => goto('/admin/hr/employee'),
                    },
                    {
                        key: 'n',
                        action: 'Intern',
                        icon: SquareUser,
                        handler: () => goto('/admin/hr/intern'),
                    },
                    {
                        key: 'l',
                        action: 'Leave',
                        icon: CalendarMinus,
                        handler: () => goto('/admin/hr/leave'),
                    },
                ],
            },
            {
                key: 'r',
                action: 'Report',
                icon: ChartPie,
                children: [
                    {
                        key: 'i',
                        action: 'Income',
                        icon: ChartColumnIncreasing,
                        handler: () => goto('/admin/report/income'),
                    },
                    {
                        key: 'e',
                        action: 'Expenditure',
                        icon: ChartColumnDecreasing,
                        handler: () => goto('/admin/report/expenditure'),
                    },
                    {
                        key: 's',
                        action: 'Sales',
                        icon: ChartBarStacked,
                        handler: () => goto('/admin/report/sales'),
                    },
                ],
            },
        ],
    },
    {
        key: 'F11',
        action: 'Full Screen',
        icon: Fullscreen,
        handler: () => {
            if (!document.fullscreenElement) {
                document.documentElement.requestFullscreen();
            } else {
                document.exitFullscreen();
            }
        },
    },
    {
        key: 'Z',
        action: 'Zen Mode',
        icon: EyeOff,
        handler: () => console.log('zen mode'),
    },
    {
        key: '/',
        action: 'Search',
        icon: Search,
        handler: () =>
            (
                document.querySelector(
                    'input[placeholder="Search..."]',
                ) as HTMLInputElement | null
            )?.focus(),
    },
    {
        key: 'Backspace',
        action: 'Go Back',
        icon: ArrowLeft,
        handler: () => history.back(),
    },
    {
        key: '?',
        action: 'Open Help Center',
        icon: HelpCircle,
        handler: () => window.open('https://example.com/help', '_blank'),
    },
];
