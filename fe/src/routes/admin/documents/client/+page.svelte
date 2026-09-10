<script lang="ts">
    import { onMount } from 'svelte';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, FileText, Briefcase, ArrowLeft, Search } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import DocumentBrowser from '$lib/components/documents/DocumentBrowser.svelte';
    import { clientService, type Client } from '$lib/services/client-management';

    pageTitle.set({
        title: 'Client Documents',
        desc: 'TOR, contracts, and other per-client documents',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Documents', icon: FileText },
        { label: 'Client Documents', icon: Briefcase },
    ]);

    let clients: Client[] = [];
    let loading = true;
    let searchText = '';
    let selectedClient: Client | null = null;

    async function loadClients() {
        loading = true;
        try {
            clients = await clientService.listClients();
        } catch (err) {
            console.error('Failed to load clients:', err);
        } finally {
            loading = false;
        }
    }

    $: filteredClients = clients.filter((c) =>
        c.name.toLowerCase().includes(searchText.toLowerCase()),
    );

    onMount(loadClients);
</script>

<PageSection title="Client Documents">
    {#if selectedClient}
        <div class="flex items-center justify-between mb-4">
            <button class="btn btn-ghost btn-sm gap-1" on:click={() => (selectedClient = null)}>
                <ArrowLeft size={16} />
                All Clients
            </button>
            <h3 class="font-semibold">{selectedClient.name}</h3>
        </div>
        <DocumentBrowser
            category="client"
            ownerId={selectedClient.id}
            navPath="/admin/documents/client"
            rootLabel={selectedClient.name} />
    {:else}
        <div class="flex gap-2 mb-4 max-w-md">
            <input
                type="text"
                placeholder="Search clients..."
                bind:value={searchText}
                class="input input-bordered flex-1" />
            <div class="btn btn-square btn-ghost no-animation">
                <Search size={20} />
            </div>
        </div>

        {#if loading}
            <div class="flex justify-center py-8">
                <span class="loading loading-spinner loading-lg"></span>
            </div>
        {:else if filteredClients.length === 0}
            <div class="text-center py-12 opacity-60">No clients found</div>
        {:else}
            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3">
                {#each filteredClients as client}
                    <button
                        class="card bg-base-200 hover:bg-base-300 transition-colors"
                        on:click={() => (selectedClient = client)}>
                        <div class="card-body items-center text-center p-4 gap-1">
                            <Briefcase size={32} class="text-primary" />
                            <span class="text-sm font-medium truncate w-full" title={client.name}>
                                {client.name}
                            </span>
                        </div>
                    </button>
                {/each}
            </div>
        {/if}
    {/if}
</PageSection>
