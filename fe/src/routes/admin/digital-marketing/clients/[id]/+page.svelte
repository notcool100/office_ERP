<script lang="ts">
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Megaphone, Briefcase, FileText, Info, ArrowLeft, Mail, Phone, Globe } from 'lucide-svelte';
    import PageSection from '../../../../../components/PageSection.svelte';
    import DocumentBrowser from '$lib/components/documents/DocumentBrowser.svelte';
    import { clientService, getClientStatusBadge, getClientStatusLabel, type Client } from '$lib/services/client-management';

    const clientId = $page.params.id;

    let client: Client | null = null;
    let loading = true;
    let errorMessage = '';
    let activeTab: 'details' | 'documents' = 'details';

    async function load() {
        loading = true;
        errorMessage = '';
        try {
            client = await clientService.getClient(clientId);
            pageTitle.set({ title: client.name, desc: 'Client details' });
            breadcrumb.set([
                { label: 'Home', icon: Home },
                { label: 'Digital Marketing', icon: Megaphone },
                { label: 'Clients', icon: Briefcase },
                { label: client.name, icon: Info },
            ]);
        } catch (err) {
            console.error('Failed to load client:', err);
            errorMessage = 'Failed to load client';
        } finally {
            loading = false;
        }
    }

    onMount(load);
</script>

<PageSection>
    <div class="flex items-center justify-between mb-4">
        <a href="/admin/digital-marketing/clients" class="btn btn-ghost btn-sm gap-1">
            <ArrowLeft size={16} />
            Back to Clients
        </a>
    </div>

    {#if loading}
        <div class="flex justify-center py-8">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    {:else if errorMessage}
        <div class="alert alert-error">{errorMessage}</div>
    {:else if client}
        <div class="flex items-center gap-3 mb-6">
            <div class="avatar placeholder">
                <div class="bg-primary text-primary-content rounded-full w-14">
                    <span class="text-xl">{client.name[0]}</span>
                </div>
            </div>
            <div>
                <h2 class="text-xl font-bold">{client.name}</h2>
                <p class="text-sm opacity-70">{client.industry || 'N/A'}</p>
            </div>
            <span class={`badge ml-auto ${getClientStatusBadge(client.status)}`}>
                {getClientStatusLabel(client.status)}
            </span>
        </div>

        <div role="tablist" class="tabs tabs-boxed mb-6 w-fit">
            <button
                role="tab"
                class="tab gap-1"
                class:tab-active={activeTab === 'details'}
                on:click={() => (activeTab = 'details')}>
                <Info size={16} />
                Details
            </button>
            <button
                role="tab"
                class="tab gap-1"
                class:tab-active={activeTab === 'documents'}
                on:click={() => (activeTab = 'documents')}>
                <FileText size={16} />
                Documents
            </button>
        </div>

        {#if activeTab === 'details'}
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 max-w-2xl">
                <div>
                    <div class="text-xs uppercase opacity-60">Contact Person</div>
                    <div>{client.contact_person || 'N/A'}</div>
                </div>
                <div>
                    <div class="text-xs uppercase opacity-60 flex items-center gap-1"><Mail size={12}/> Email</div>
                    <div>{client.email || 'N/A'}</div>
                </div>
                <div>
                    <div class="text-xs uppercase opacity-60 flex items-center gap-1"><Phone size={12}/> Phone</div>
                    <div>{client.phone || 'N/A'}</div>
                </div>
                <div>
                    <div class="text-xs uppercase opacity-60 flex items-center gap-1"><Globe size={12}/> Website</div>
                    <div>{client.website || 'N/A'}</div>
                </div>
                <div class="sm:col-span-2">
                    <div class="text-xs uppercase opacity-60">Address</div>
                    <div>{client.address || 'N/A'}</div>
                </div>
                <div class="sm:col-span-2">
                    <div class="text-xs uppercase opacity-60">Notes</div>
                    <div>{client.notes || 'N/A'}</div>
                </div>
            </div>
        {:else}
            <DocumentBrowser
                category="client"
                ownerId={client.id}
                navPath="/admin/documents/client"
                rootLabel="Client Documents" />
        {/if}
    {/if}
</PageSection>
