<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { onMount, onDestroy } from 'svelte';
    import {
        registerShortcut,
        unregisterShortcut,
    } from '$lib/stores/shortcutManager';
    import { Home, Settings, UserPlus, Plus } from 'lucide-svelte';
    import { goto } from '$app/navigation';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Contacts',
        desc: 'Manage your contacts',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'Contacts', icon: UserPlus },
    ]);

    const contacts = [
        { name: 'Alice', email: 'alice@example.com' },
        { name: 'Bob', email: 'bob@example.com' },
    ];

    function addContact() {
        goto('/admin/settings/contact/create');
    }

    onMount(() => {
        registerShortcut({
            key: 'a',
            action: 'Add Contact',
            icon: Plus,
            handler: addContact,
        });
    });

    onDestroy(() => unregisterShortcut('a'));
</script>

<PageSection>
    <div class="text-right mb-2">
        <button class="btn btn-primary btn-sm" on:click={addContact}
            >Add Contact</button>
    </div>
    <div class="overflow-x-auto">
        <table class="table">
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Email</th>
                </tr>
            </thead>
            <tbody>
                {#each contacts as contact}
                    <tr>
                        <td>{contact.name}</td>
                        <td>{contact.email}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
</PageSection>
