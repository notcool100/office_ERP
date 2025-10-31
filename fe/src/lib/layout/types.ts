export type BreadcrumbItem = {
    label: string;
    icon?: typeof import('lucide-svelte').Icon;
    path?: string;
    active?: boolean;
};

export type Shortcut = {
    key: string;
    action: string;
    icon: typeof import('lucide-svelte').Icon;
    handler?: (event: KeyboardEvent) => void;
    children?: Shortcut[];
};

export type MenuItem = {
    name: string;
    type: 'file' | 'folder';
    icon?: typeof import('lucide-svelte').Icon;
    shortcut?: string;
    expanded?: boolean;
    color: string;
    url?: string;
    children?: MenuItem[];
};

export type PageTitle = {
    title: string;
    desc: string;
};
