<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { onMount, onDestroy } from 'svelte';
    import {
        registerShortcut,
        unregisterShortcut,
    } from '$lib/stores/shortcutManager';
    import { Home, Settings, UserPlus } from 'lucide-svelte';
    import { goto } from '$app/navigation';
    import PageSection from '../../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Add Contact',
        desc: 'Create a new contact',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'Add Contact', icon: UserPlus },
    ]);

    let name = '';
    let email = '';

    function goBack() {
        goto('/admin/settings/contact');
    }

    function handleSubmit() {
        console.log({ name, email });
    }

    onMount(() => {
        registerShortcut({
            key: '\\',
            action: 'Back to List',
            icon: UserPlus,
            handler: goBack,
        });
    });

    onDestroy(() => unregisterShortcut('\\'));
</script>

<PageSection>
    <div class="space-y-4 max-w-md">
        <div>
            <label class="text-sm block mb-1" for="name">Name</label>
            <input
                id="name"
                type="text"
                bind:value={name}
                placeholder="John Doe"
                class="input input-bordered w-full bg-base-100 border-base-300 text-sm" />
        </div>

        <div>
            <label class="text-sm block mb-1" for="email">Email</label>
            <input
                id="email"
                type="email"
                bind:value={email}
                placeholder="john@example.com"
                class="input input-bordered w-full bg-base-100 border-base-300 text-sm" />
        </div>

        <button class="btn btn-primary btn-sm" on:click={handleSubmit}
            >Save Contact</button>
    </div>
</PageSection>
