<script lang="ts">
    import '../../app.css';
    import { onMount, tick } from 'svelte';
    import Sidebar from '../../components/layout/Sidebar.svelte';
    import StatusBar from '../../components/layout/StatusBar.svelte';
    import { userStore } from '$lib/stores/user';
    import { goto } from '$app/navigation';
    import { messagingService, type Channel } from '$lib/services/messaging';
    import { userService } from '$lib/services/user-service';
    import type { User } from '$lib/types/user';
    import { Hash, MessageCircle, Plus, Users, X } from 'lucide-svelte';

    const { children } = $props();

    let allChannels: Channel[] = $state([]);
    let users: User[] = $state([]);
    let loading = $state(true);

    // Modals state
    let showChannelModal = $state(false);
    let showDmModal = $state(false);

    // Forms
    let newChannelName = $state('');
    let newChannelDesc = $state('');
    let newChannelPrivate = $state(false);
    let selectedUserId = $state('');
    let currentUserId = $derived($userStore.user?.id);

    let channels = $derived(
        allChannels.filter(
            (c) => !c.is_private || (c.is_private && !c.name.startsWith('DM-')),
        ),
    );
    let dms = $derived(
        allChannels.filter((c) => c.is_private && c.name.startsWith('DM-')),
    );

    onMount(async () => {
        await userStore.init();
        if (!$userStore.isAuthenticated) {
            goto('/login');
            return;
        }

        try {
            const [fetchedChannels, fetchedUsers] = await Promise.all([
                messagingService.listChannels(),
                userService.getAll(),
            ]);
            allChannels = fetchedChannels;
            users = fetchedUsers.filter((u) => u.id !== currentUserId);
        } catch (error) {
            console.error('Failed to load messaging data:', error);
        } finally {
            loading = false;
        }
    });

    async function handleCreateChannel(e: Event) {
        e.preventDefault();
        try {
            const channel = await messagingService.createChannel({
                name: newChannelName,
                description: newChannelDesc,
                is_private: newChannelPrivate,
            });
            allChannels = [...allChannels, channel];
            showChannelModal = false;
            newChannelName = '';
            newChannelDesc = '';
            newChannelPrivate = false;
            goto(`/messaging/channels/${channel.id}`);
        } catch (error) {
            console.error('Failed to create channel:', error);
        }
    }

    async function handleCreateDm(e: Event) {
        e.preventDefault();
        if (!selectedUserId) return;

        try {
            // Check if DM already exists (simplistic check)
            const otherUser = users.find((u) => u.id === selectedUserId);
            if (!otherUser) return;

            // Name format for DMs: DM-user1id-user2id (just a convention, backend could handle it better)
            const dmName = `DM-${Math.random().toString(36).substring(7)}`;

            const channel = await messagingService.createChannel({
                name: dmName,
                description: `Direct message with ${otherUser.userName}`,
                is_private: true,
                members: [selectedUserId],
            });
            allChannels = [...allChannels, channel];
            showDmModal = false;
            selectedUserId = '';
            goto(`/messaging/dm/${channel.id}`);
        } catch (error) {
            console.error('Failed to create DM:', error);
        }
    }

    function getDmName(channel: Channel) {
        // Try to extract the other user's name from description or logic
        // E.g. description is "Direct message with Alice"
        if (channel.description?.startsWith('Direct message with ')) {
            return channel.description.replace('Direct message with ', '');
        }
        return 'Unknown User';
    }
</script>

<div
    class="h-screen bg-base-300 text-base-content flex flex-col font-mono text-sm">
    <div class="flex flex-1 overflow-hidden">
        <Sidebar />

        <!-- Messaging Sidebar -->
        <div class="w-64 bg-base-100 border-r border-base-300 flex flex-col">
            <div
                class="p-4 border-b border-base-300 flex items-center justify-between">
                <h2 class="font-bold text-lg">Messaging</h2>
                <button
                    class="btn btn-ghost btn-xs"
                    onclick={() => (showChannelModal = true)}>
                    <Plus class="w-4 h-4" />
                </button>
            </div>

            <div class="flex-1 overflow-y-auto">
                <div class="p-4">
                    <div
                        class="flex items-center justify-between mb-2 text-xs uppercase tracking-wider text-base-content/50 font-bold">
                        <span>Channels</span>
                    </div>
                    {#if loading}
                        <div class="flex justify-center p-4">
                            <span class="loading loading-spinner loading-xs"
                            ></span>
                        </div>
                    {:else}
                        <div class="space-y-1">
                            {#each channels as channel}
                                <a
                                    href="/messaging/channels/{channel.id}"
                                    class="flex items-center px-2 py-1.5 rounded hover:bg-base-200 transition-colors group">
                                    <Hash
                                        class="w-4 h-4 mr-2 text-base-content/50" />
                                    <span class="truncate">{channel.name}</span>
                                </a>
                            {/each}
                        </div>
                    {/if}

                    <div
                        class="flex items-center justify-between mt-6 mb-2 text-xs uppercase tracking-wider text-base-content/50 font-bold">
                        <span>Direct Messages</span>
                        <button
                            class="btn btn-ghost btn-xs p-0 min-h-0 h-4 w-4"
                            onclick={() => (showDmModal = true)}>
                            <Plus class="w-3 h-3" />
                        </button>
                    </div>
                    <div class="space-y-1">
                        {#if loading}
                            <div class="flex justify-center p-4">
                                <span class="loading loading-spinner loading-xs"
                                ></span>
                            </div>
                        {:else if dms.length === 0}
                            <div
                                class="text-xs text-base-content/30 px-2 italic">
                                No recent DMs
                            </div>
                        {:else}
                            {#each dms as dm}
                                <a
                                    href="/messaging/dm/{dm.id}"
                                    class="flex items-center px-2 py-1.5 rounded hover:bg-base-200 transition-colors group">
                                    <div
                                        class="w-4 h-4 rounded-full bg-primary/20 text-primary flex items-center justify-center text-[8px] font-bold mr-2">
                                        {getDmName(dm)[0].toUpperCase()}
                                    </div>
                                    <span class="truncate"
                                        >{getDmName(dm)}</span>
                                </a>
                            {/each}
                        {/if}
                    </div>
                </div>
            </div>
        </div>

        <!-- Main Chat Area -->
        <div class="flex-1 bg-base-200 flex flex-col overflow-hidden">
            {@render children()}
        </div>
    </div>
    <StatusBar />
</div>

<!-- Create Channel Modal -->
{#if showChannelModal}
    <div
        class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
        <div
            class="bg-base-100 rounded-xl w-full max-w-md shadow-xl border border-base-300">
            <div
                class="flex justify-between items-center p-4 border-b border-base-300">
                <h3 class="font-bold text-lg">Create Channel</h3>
                <button
                    class="btn btn-ghost btn-sm btn-square"
                    onclick={() => (showChannelModal = false)}>
                    <X class="w-4 h-4" />
                </button>
            </div>
            <form onsubmit={handleCreateChannel} class="p-4 space-y-4">
                <div class="form-control">
                    <label class="label" for="channel-name-input"
                        ><span class="label-text">Channel Name</span></label>
                    <input
                        id="channel-name-input"
                        type="text"
                        bind:value={newChannelName}
                        placeholder="e.g. general"
                        class="input input-bordered w-full"
                        required />
                </div>
                <div class="form-control">
                    <label class="label" for="channel-desc-input"
                        ><span class="label-text">Description (Optional)</span
                        ></label>
                    <input
                        id="channel-desc-input"
                        type="text"
                        bind:value={newChannelDesc}
                        placeholder="What's this channel about?"
                        class="input input-bordered w-full" />
                </div>
                <div class="form-control">
                    <label
                        class="label cursor-pointer flex justify-start space-x-3">
                        <input
                            type="checkbox"
                            bind:checked={newChannelPrivate}
                            class="checkbox checkbox-primary" />
                        <span class="label-text">Make Private</span>
                    </label>
                </div>
                <div class="flex justify-end pt-2">
                    <button
                        type="button"
                        class="btn btn-ghost mr-2"
                        onclick={() => (showChannelModal = false)}
                        >Cancel</button>
                    <button
                        type="submit"
                        class="btn btn-primary"
                        disabled={!newChannelName.trim()}>Create</button>
                </div>
            </form>
        </div>
    </div>
{/if}

<!-- Create DM Modal -->
{#if showDmModal}
    <div
        class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
        <div
            class="bg-base-100 rounded-xl w-full max-w-md shadow-xl border border-base-300">
            <div
                class="flex justify-between items-center p-4 border-b border-base-300">
                <h3 class="font-bold text-lg">New Direct Message</h3>
                <button
                    class="btn btn-ghost btn-sm btn-square"
                    onclick={() => (showDmModal = false)}>
                    <X class="w-4 h-4" />
                </button>
            </div>
            <form onsubmit={handleCreateDm} class="p-4 space-y-4">
                <div class="form-control">
                    <label class="label" for="dm-user-select"
                        ><span class="label-text">Select User</span></label>
                    <select
                        id="dm-user-select"
                        bind:value={selectedUserId}
                        class="select select-bordered w-full"
                        required>
                        <option value="" disabled selected
                            >Select a person</option>
                        {#each users as user}
                            <option value={user.id}>{user.userName}</option>
                        {/each}
                    </select>
                </div>
                <div class="flex justify-end pt-2">
                    <button
                        type="button"
                        class="btn btn-ghost mr-2"
                        onclick={() => (showDmModal = false)}>Cancel</button>
                    <button
                        type="submit"
                        class="btn btn-primary"
                        disabled={!selectedUserId}>Start DM</button>
                </div>
            </form>
        </div>
    </div>
{/if}
