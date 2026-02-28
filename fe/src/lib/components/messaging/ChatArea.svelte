<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { page } from '$app/stores';
    import {
        messagingService,
        type Message,
        type Channel,
    } from '$lib/services/messaging';
    import { userStore } from '$lib/stores/user';
    import { userService } from '$lib/services/user-service';
    import type { User } from '$lib/types/user';
    import { PUBLIC_API_URL } from '$env/static/public';
    import {
        Send,
        Hash,
        Info,
        Search,
        Bell,
        Star,
        UserPlus,
        X,
        MessageSquare,
    } from 'lucide-svelte';
    import { fade } from 'svelte/transition';

    let channelId = $state($page.params.id);
    let channel: Channel | null = $state(null);
    let messages: Message[] = $state([]);
    let newMessage = $state('');
    let loading = $state(true);
    let socket: WebSocket | null = null;
    let messageContainer: HTMLDivElement | null = $state(null);

    // Add member state
    let users: User[] = $state([]);
    let showAddMemberModal = $state(false);
    let selectedUserId = $state('');
    let addingMember = $state(false);

    // Settings state
    let showSettingsModal = $state(false);
    let editingChannelName = $state('');
    let editingChannelDesc = $state('');
    let updatingChannel = $state(false);

    // Member list state
    let channelMembers: User[] = $state([]);
    let loadingMembers = $state(false);

    // Mentions state
    let showMentions = $state(false);
    let mentionSearch = $state('');
    let filteredMentions = $derived.by(() => {
        let all = [
            ...channelMembers,
            {
                id: 'everyone',
                userName: 'everyone',
                user_name: 'everyone',
            } as any,
        ];
        if (!mentionSearch) return all;
        const lower = mentionSearch.toLowerCase();
        return all.filter((u) =>
            (u.userName || u.user_name).toLowerCase().includes(lower),
        );
    });
    let mentionSelectedIndex = $state(0);

    let textareaRef: HTMLTextAreaElement | null = $state(null);

    // Re-load when channel changes
    $effect(() => {
        if ($page.params.id !== channelId) {
            channelId = $page.params.id;
            channel = null;
            loadChannel();
            loadMessages();
            connectWs();
        }
    });

    async function loadChannel() {
        try {
            const [fetchedChannel, fetchedUsers] = await Promise.all([
                messagingService.getChannel(channelId),
                userService.getAll(),
            ]);
            channel = fetchedChannel;
            users = fetchedUsers.filter((u) => u.id !== $userStore.user?.id);
            // Also preload channel members for mentions
            loadMembers();
        } catch (error) {
            console.error('Failed to load channel data:', error);
        }
    }

    async function loadMessages() {
        loading = true;
        try {
            messages = await messagingService.listMessages(channelId);
            messages = messages.reverse(); // Newest at bottom
            scrollToBottom();
        } catch (error) {
            console.error('Failed to load messages:', error);
        } finally {
            loading = false;
        }
    }

    function connectWs() {
        if (socket) socket.close();

        const wsUrl =
            PUBLIC_API_URL.replace('http', 'ws') + `/ws/messaging/${channelId}`;
        socket = new WebSocket(wsUrl);

        socket.onmessage = (event) => {
            const data = JSON.parse(event.data);
            if (data.message_type === 'new_message') {
                const msg: Message = data.payload;
                // Avoid duplicates if we just sent it
                if (!messages.find((m) => m.id === msg.id)) {
                    messages = [...messages, msg];
                    scrollToBottom();
                }
            }
        };

        socket.onclose = () => {
            console.log('WS connection closed, retrying...');
            setTimeout(connectWs, 3000); // Note: Simple reconnect logic
        };
    }

    async function sendMessage(e?: Event) {
        if (e) e.preventDefault();
        if (!newMessage.trim()) return;

        const content = newMessage;
        newMessage = '';

        try {
            await messagingService.sendMessage(channelId, content);
            // Optimistic update or WS will handle it. WS is safer for ordering.
        } catch (error) {
            console.error('Failed to send message:', error);
            newMessage = content; // restore
        }
    }

    function scrollToBottom() {
        setTimeout(() => {
            if (messageContainer) {
                messageContainer.scrollTop = messageContainer.scrollHeight;
            }
        }, 100);
    }

    function getChannelDisplayName(c: Channel | null) {
        if (!c) return 'Loading...';
        if (c.is_private && c.name.startsWith('DM-')) {
            if (c.description?.startsWith('Direct message with ')) {
                return c.description.replace('Direct message with ', '');
            }
            return 'Direct Message';
        }
        return c.name;
    }

    function formatMessageContent(content: string) {
        if (!content) return '';
        // Escape HTML
        let safeContent = content.replace(/</g, '&lt;').replace(/>/g, '&gt;');
        // Replace @username with highlighted span
        return safeContent.replace(
            /@(\w+)/g,
            '<span class="text-primary font-bold hover:underline cursor-pointer bg-primary/10 px-1 rounded">@$1</span>',
        );
    }

    async function handleAddMember(e: Event) {
        e.preventDefault();
        if (!selectedUserId) return;

        addingMember = true;
        try {
            await messagingService.addMember(channelId, selectedUserId);
            showAddMemberModal = false;
            selectedUserId = '';
            // Optional: Show toast notification
        } catch (error) {
            console.error('Failed to add member:', error);
        } finally {
            addingMember = false;
        }
    }

    async function openSettingsModal() {
        if (!channel || channel.name.startsWith('DM-')) return;
        editingChannelName = channel.name;
        editingChannelDesc = channel.description || '';
        showSettingsModal = true;
        loadMembers();
    }

    async function loadMembers() {
        loadingMembers = true;
        try {
            channelMembers =
                await messagingService.getChannelMembers(channelId);
        } catch (error) {
            console.error('Failed to load members:', error);
        } finally {
            loadingMembers = false;
        }
    }

    async function handleUpdateChannel(e: Event) {
        e.preventDefault();
        if (!editingChannelName.trim()) return;

        updatingChannel = true;
        try {
            const updated = await messagingService.updateChannel(channelId, {
                name: editingChannelName,
                description: editingChannelDesc,
            });
            channel = updated;
        } catch (error) {
            console.error('Failed to update channel:', error);
        } finally {
            updatingChannel = false;
        }
    }
    async function handleRemoveMember(userId: string) {
        if (!confirm('Are you sure you want to remove this member?')) return;
        try {
            await messagingService.removeMember(channelId, userId);
            channelMembers = channelMembers.filter((m) => m.id !== userId);
        } catch (error) {
            console.error('Failed to remove member:', error);
        }
    }

    function handleTextareaInput(e: Event) {
        const target = e.target as HTMLTextAreaElement;
        const text = target.value;
        const cursor = target.selectionStart;

        // Find if we are typing a mention
        // Regex looks for @ followed by letters/numbers/underscores up to the cursor
        const textBeforeCursor = text.substring(0, cursor);
        const match = textBeforeCursor.match(/@(\w*)$/);

        if (match) {
            showMentions = true;
            mentionSearch = match[1];
            mentionSelectedIndex = 0; // Reset selection index
        } else {
            showMentions = false;
        }
    }

    function insertMention(user: User) {
        if (!textareaRef) return;
        const name = (user as any).user_name || user.userName;
        const text = newMessage;
        const cursor = textareaRef.selectionStart;

        const textBeforeCursor = text.substring(0, cursor);
        const textAfterCursor = text.substring(cursor);

        // Find the @ that triggered this
        const match = textBeforeCursor.match(/@(\w*)$/);
        if (match) {
            const prefix = textBeforeCursor.substring(0, match.index);
            newMessage = prefix + '@' + name + ' ' + textAfterCursor;

            showMentions = false;
            // Best effort focus back
            setTimeout(() => {
                const newCursor = prefix.length + name.length + 2;
                textareaRef?.setSelectionRange(newCursor, newCursor);
                textareaRef?.focus();
            }, 0);
        }
    }

    onMount(() => {
        loadChannel();
        loadMessages();
        connectWs();
    });

    onDestroy(() => {
        if (socket) socket.close();
    });
</script>

<!-- Channel Header -->
<div
    class="h-16 border-b border-base-300 bg-base-100 flex items-center justify-between px-6 shadow-sm z-10">
    <div class="flex items-center space-x-3">
        <div class="flex items-center space-x-1">
            {#if channel?.is_private && channel?.name.startsWith('DM-')}
                <MessageSquare class="w-5 h-5 text-base-content/50" />
            {:else}
                <Hash class="w-5 h-5 text-base-content/50" />
            {/if}
            <h2 class="font-bold text-lg">{getChannelDisplayName(channel)}</h2>
        </div>
        {#if channel?.is_private && !channel?.name.startsWith('DM-')}
            <button
                class="btn btn-ghost btn-xs text-base-content/60 hover:text-primary"
                onclick={(e) => {
                    e.preventDefault();
                    console.log('Add Member clicked');
                    showAddMemberModal = true;
                }}
                title="Add Member">
                <UserPlus class="w-4 h-4" />
            </button>
        {/if}
        <button
            class="btn btn-ghost btn-xs text-base-content/40 hover:text-warning"
            onclick={() => alert('Star functionality coming soon!')}
            title="Star Channel">
            <Star class="w-4 h-4" />
        </button>
    </div>

    <div class="flex items-center space-x-4 text-base-content/60">
        <div class="join border border-base-300 rounded overflow-hidden h-8">
            <button
                class="join-item px-3 hover:bg-base-200 border-r border-base-300 flex items-center">
                <Search class="w-4 h-4" />
            </button>
            <input
                type="text"
                placeholder="Search..."
                class="join-item bg-base-100 px-2 outline-none w-32 focus:w-48 transition-all" />
        </div>
        <button class="btn btn-ghost btn-sm btn-square">
            <Bell class="w-5 h-5" />
        </button>
        {#if channel && !channel.name.startsWith('DM-')}
            <button
                class="btn btn-ghost btn-sm btn-square"
                onclick={openSettingsModal}>
                <Info class="w-5 h-5" />
            </button>
        {/if}
    </div>
</div>

<!-- Message Area -->
<div
    bind:this={messageContainer}
    class="flex-1 overflow-y-auto p-6 space-y-6 bg-base-100/50">
    {#if loading}
        <div
            class="flex flex-col items-center justify-center h-full text-base-content/30 italic">
            <span class="loading loading-spinner loading-lg mb-2"></span>
            <span>Decrypting messages...</span>
        </div>
    {:else if messages.length === 0}
        <div
            class="flex flex-col items-center justify-center h-full text-base-content/30 italic">
            <p>This is the start of a legendary conversation.</p>
        </div>
    {:else}
        {#each messages as msg}
            <div class="flex space-x-4 group" in:fade>
                <div class="flex-shrink-0">
                    <div
                        class="w-10 h-10 rounded-lg bg-primary text-primary-content flex items-center justify-center font-bold text-lg shadow-sm">
                        {(msg.sender_name || 'U')[0].toUpperCase()}
                    </div>
                </div>
                <div class="flex-1 min-w-0">
                    <div class="flex items-baseline space-x-2">
                        <span
                            class="font-bold text-base-content hover:underline cursor-pointer">
                            {msg.sender_name || 'Unknown User'}
                        </span>
                        <span class="text-xs text-base-content/40">
                            {new Date(msg.created_at).toLocaleTimeString([], {
                                hour: '2-digit',
                                minute: '2-digit',
                            })}
                        </span>
                    </div>
                    <p
                        class="text-base-content/90 mt-0.5 leading-relaxed break-words whitespace-pre-wrap">
                        {@html formatMessageContent(msg.content)}
                    </p>
                </div>
                <!-- Message Actions on Hover -->
                <div
                    class="opacity-0 group-hover:opacity-100 transition-opacity flex space-x-1">
                    <!-- Placeholder for reactions/etc -->
                </div>
            </div>
        {/each}
    {/if}
</div>

<!-- Message Input -->
<div class="p-6 bg-base-100 border-t border-base-300 relative">
    <!-- Mentions Dropdown -->
    {#if showMentions && filteredMentions.length > 0}
        <div
            class="absolute bottom-full left-6 mb-2 bg-base-100 shadow-xl border border-base-300 rounded-lg max-h-48 overflow-y-auto w-64 z-50">
            <ul class="py-1">
                {#each filteredMentions as user, i}
                    <li>
                        <button
                            type="button"
                            class="w-full text-left px-4 py-2 hover:bg-base-200 transition-colors flex items-center space-x-2 {i ===
                            mentionSelectedIndex
                                ? 'bg-base-300'
                                : ''}"
                            onclick={() => insertMention(user)}>
                            <div
                                class="w-6 h-6 rounded bg-primary/20 text-primary flex items-center justify-center text-xs font-bold">
                                {((user as any).user_name ||
                                    user.userName)[0].toUpperCase()}
                            </div>
                            <span class="text-sm font-medium"
                                >@{(user as any).user_name ||
                                    user.userName}</span>
                        </button>
                    </li>
                {/each}
            </ul>
        </div>
    {/if}

    <form
        class="relative border-2 border-base-300 rounded-xl bg-base-100 focus-within:border-primary transition-colors shadow-sm"
        onsubmit={sendMessage}>
        <textarea
            bind:this={textareaRef}
            bind:value={newMessage}
            oninput={handleTextareaInput}
            onblur={() => setTimeout(() => (showMentions = false), 200)}
            class="w-full bg-transparent p-4 pr-16 outline-none resize-none min-h-[50px] max-h-[200px]"
            placeholder="Message #general"
            onkeydown={(e) => {
                if (showMentions && filteredMentions.length > 0) {
                    if (e.key === 'ArrowDown') {
                        e.preventDefault();
                        mentionSelectedIndex =
                            (mentionSelectedIndex + 1) %
                            filteredMentions.length;
                        return;
                    }
                    if (e.key === 'ArrowUp') {
                        e.preventDefault();
                        mentionSelectedIndex =
                            (mentionSelectedIndex -
                                1 +
                                filteredMentions.length) %
                            filteredMentions.length;
                        return;
                    }
                    if (e.key === 'Enter' || e.key === 'Tab') {
                        e.preventDefault();
                        insertMention(filteredMentions[mentionSelectedIndex]);
                        return;
                    }
                }

                if (e.key === 'Enter' && !e.shiftKey) {
                    e.preventDefault();
                    sendMessage();
                }
            }}></textarea>

        <div class="absolute right-3 bottom-3 flex items-center space-x-2">
            <button
                type="submit"
                class="btn btn-primary btn-sm btn-square"
                disabled={!newMessage.trim()}>
                <Send class="w-4 h-4" />
            </button>
        </div>
    </form>
    <div class="mt-2 flex text-[10px] text-base-content/30 space-x-3">
        <span><b>Return</b> to send</span>
        <span><b>Shift + Return</b> for new line</span>
    </div>
</div>

<!-- Add Member Modal -->
{#if showAddMemberModal}
    <div class="modal modal-open z-[100]">
        <div class="modal-box relative">
            <h3 class="font-bold text-lg">Add Member</h3>
            <button
                class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                onclick={() => (showAddMemberModal = false)}>✕</button>

            <form onsubmit={handleAddMember} class="py-4 space-y-4">
                <div class="form-control">
                    <label class="label" for="add-member-select"
                        ><span class="label-text">Select User</span></label>
                    <select
                        id="add-member-select"
                        bind:value={selectedUserId}
                        class="select select-bordered w-full"
                        required>
                        <option value="" disabled selected
                            >Select a person</option>
                        {#each users as user}
                            <option value={user.id}
                                >{user.userName ||
                                    (user as any).user_name}</option>
                        {/each}
                    </select>
                </div>
                <div class="modal-action mt-6">
                    <button
                        type="button"
                        class="btn btn-ghost"
                        onclick={() => (showAddMemberModal = false)}
                        >Cancel</button>
                    <button
                        type="submit"
                        class="btn btn-primary"
                        disabled={!selectedUserId || addingMember}>
                        {#if addingMember}
                            <span
                                class="loading loading-spinner loading-xs mr-2"
                            ></span>
                        {/if}
                        Add
                    </button>
                </div>
            </form>
        </div>
        <!-- Backdrop to close when clicked outside -->
        <button
            class="modal-backdrop bg-black/50 cursor-default"
            type="button"
            onclick={() => (showAddMemberModal = false)}>
            <span class="sr-only">Close Modal</span>
        </button>
    </div>
{/if}

<!-- Channel Settings Modal -->
{#if showSettingsModal}
    <div class="modal modal-open z-[100]">
        <div class="modal-box relative">
            <h3 class="font-bold text-lg mb-4">Channel Settings</h3>
            <button
                class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                onclick={() => (showSettingsModal = false)}>✕</button>

            <!-- Edit Details -->
            <form
                onsubmit={handleUpdateChannel}
                class="space-y-4 border-b border-base-300 pb-6 mb-6">
                <h4
                    class="font-semibold text-sm text-base-content/70 uppercase">
                    Details
                </h4>
                <div class="form-control">
                    <label class="label" for="edit-channel-name"
                        ><span class="label-text">Channel Name</span></label>
                    <input
                        id="edit-channel-name"
                        type="text"
                        bind:value={editingChannelName}
                        class="input input-bordered w-full"
                        required />
                </div>
                <div class="form-control">
                    <label class="label" for="edit-channel-desc"
                        ><span class="label-text">Description</span></label>
                    <textarea
                        id="edit-channel-desc"
                        bind:value={editingChannelDesc}
                        class="textarea textarea-bordered w-full resize-none h-20"
                        placeholder="What is this channel about?"></textarea>
                </div>
                <div class="flex justify-end">
                    <button
                        type="submit"
                        class="btn btn-primary btn-sm"
                        disabled={!editingChannelName.trim() ||
                            updatingChannel}>
                        {#if updatingChannel}
                            <span
                                class="loading loading-spinner loading-xs mr-2"
                            ></span>
                        {/if}
                        Save Changes
                    </button>
                </div>
            </form>

            <!-- Manage Members -->
            <div>
                <h4
                    class="font-semibold text-sm text-base-content/70 uppercase mb-4">
                    Members
                </h4>
                {#if loadingMembers}
                    <div class="flex justify-center p-4">
                        <span class="loading loading-spinner loading-sm"></span>
                    </div>
                {:else if channelMembers.length === 0}
                    <p class="text-sm text-base-content/50 italic py-2">
                        No members found.
                    </p>
                {:else}
                    <ul class="space-y-2 max-h-48 overflow-y-auto pr-2">
                        {#each channelMembers as member}
                            <li
                                class="flex items-center justify-between p-2 rounded hover:bg-base-200 transition-colors">
                                <span class="font-medium text-sm"
                                    >{(member as any).user_name ||
                                        member.userName}</span>
                                {#if member.id !== $userStore.user?.id}
                                    <button
                                        class="btn btn-ghost btn-xs text-error"
                                        onclick={() =>
                                            handleRemoveMember(member.id)}>
                                        Remove
                                    </button>
                                {:else}
                                    <span
                                        class="text-xs text-base-content/50 bg-base-300 px-2 rounded-full"
                                        >You</span>
                                {/if}
                            </li>
                        {/each}
                    </ul>
                {/if}
            </div>
        </div>
        <!-- Backdrop to close when clicked outside -->
        <button
            class="modal-backdrop bg-black/50 cursor-default"
            type="button"
            onclick={() => (showSettingsModal = false)}>
            <span class="sr-only">Close Modal</span>
        </button>
    </div>
{/if}
