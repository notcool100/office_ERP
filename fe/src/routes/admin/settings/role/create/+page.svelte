<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { onMount, onDestroy } from 'svelte';
    import { Home, Settings, Shield } from 'lucide-svelte';
    import {
        registerShortcut,
        unregisterShortcut,
    } from '$lib/stores/shortcutManager';
    import { goto } from '$app/navigation';
    import PageSection from '../../../../../components/PageSection.svelte';
    import { menuList } from '$lib/layout/menu';
    import type { MenuItem } from '$lib/layout/types';

    pageTitle.set({
        title: 'Add Role',
        desc: 'Create a new role and assign permissions',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'Add Role', icon: Shield },
    ]);

    function goBack() {
        goto('/admin/settings/role');
    }

    onMount(() => {
        registerShortcut({
            key: '\\',
            action: 'Back to List',
            icon: Shield,
            handler: goBack,
        });
    });

    onDestroy(() => unregisterShortcut('\\'));

    interface PermissionItem {
        name: string;
    }

    function flatten(list: MenuItem[], out: PermissionItem[] = []) {
        for (const item of list) {
            if (item.type === 'file') {
                out.push({ name: item.name });
            }
            if (item.children) {
                flatten(item.children, out);
            }
        }
        return out;
    }

    const allPermissions: PermissionItem[] = flatten(menuList);

    let name = '';
    let description = '';
    let search = '';
    let selected: string[] = [];

    $: filtered = allPermissions.filter((p) =>
        p.name.toLowerCase().includes(search.toLowerCase()),
    );

    function togglePermission(p: string, checked: boolean) {
        if (checked) {
            if (!selected.includes(p)) selected = [...selected, p];
        } else {
            selected = selected.filter((s) => s !== p);
        }
    }

    function handleSubmit() {
        console.log({ name, description, permissions: selected });
    }
</script>

<PageSection>
    <div class="space-y-4 max-w-md">
        <div>
            <label class="text-sm block mb-1" for="name">Role Name</label>
            <input
                id="name"
                type="text"
                bind:value={name}
                placeholder="Enter role name"
                class="input input-bordered w-full bg-base-100 border-base-300 text-sm" />
        </div>

        <div>
            <label class="text-sm block mb-1" for="description"
                >Role Description</label>
            <textarea
                id="description"
                rows="3"
                bind:value={description}
                placeholder="Describe this role"
                class="textarea textarea-bordered w-full bg-base-100 border-base-300 text-sm"
            ></textarea>
        </div>

        <div>
            <label class="text-sm block mb-1" for="search">Permissions</label>
            <input
                id="search"
                type="text"
                bind:value={search}
                placeholder="Search permissions"
                class="input input-bordered w-full mb-2 bg-base-100 border-base-300 text-sm" />
            <div class="max-h-40 overflow-y-auto space-y-1">
                {#each filtered as perm}
                    <label class="flex items-center space-x-2">
                        <input
                            type="checkbox"
                            class="checkbox checkbox-sm"
                            checked={selected.includes(perm.name)}
                            on:change={(e) =>
                                togglePermission(
                                    perm.name,
                                    (e.target as HTMLInputElement).checked,
                                )} />
                        <span>{perm.name}</span>
                    </label>
                {/each}
            </div>
        </div>

        <button class="btn btn-primary btn-sm" on:click={handleSubmit}
            >Save Role</button>
    </div>
</PageSection>
