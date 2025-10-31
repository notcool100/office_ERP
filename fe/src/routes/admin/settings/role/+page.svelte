<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Settings, Shield, Plus } from 'lucide-svelte';
    import { onMount, onDestroy } from 'svelte';
    import {
        registerShortcut,
        unregisterShortcut,
    } from '$lib/stores/shortcutManager';
    import { goto } from '$app/navigation';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Roles',
        desc: 'Manage application roles',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'Roles', icon: Shield },
    ]);

    const roles = [
        { name: 'Admin', users: 2 },
        { name: 'Manager', users: 5 },
        { name: 'Staff', users: 10 },
    ];

    function addRole() {
        goto('/admin/settings/role/create');
    }

    onMount(() => {
        registerShortcut({
            key: 'a',
            action: 'Add Role',
            icon: Plus,
            handler: addRole,
        });
    });

    onDestroy(() => unregisterShortcut('a'));
</script>

<PageSection>
    <div class="text-right mb-2">
        <button class="btn btn-primary btn-sm" on:click={addRole}
            >Add Role</button>
    </div>
    <div class="overflow-x-auto">
        <table class="table">
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Users</th>
                </tr>
            </thead>
            <tbody>
                {#each roles as role}
                    <tr>
                        <td>{role.name}</td>
                        <td>{role.users}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
</PageSection>
