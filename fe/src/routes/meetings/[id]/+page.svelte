<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    import { userStore } from '$lib/stores/user';
    import { meetingStore } from '$lib/stores/meeting';
    import { meetingService, type Meeting } from '$lib/services/meeting';
    import {
        MeetingClient,
        type MeetingClientState,
        type RemoteParticipant,
    } from '$lib/webrtc/meetingClient';
    import {
        Mic,
        MicOff,
        Video,
        VideoOff,
        ScreenShare,
        ScreenShareOff,
        Circle,
        PhoneOff,
        Users,
        MessageSquare,
        Send,
        X,
        Loader2,
        AlertTriangle,
        Crown,
    } from 'lucide-svelte';

    const meetingId = $page.params.id;

    let meeting: Meeting | null = $state(null);
    let loadError = $state('');
    let loading = $state(true);

    let client: MeetingClient | null = $state(null);
    let clientState: MeetingClientState = $state({
        localStream: null,
        micOn: true,
        cameraOn: true,
        isScreenSharing: false,
        isRecording: false,
        connectionState: 'idle',
        participants: {},
        roster: {},
        chatMessages: [],
        meetingEnded: false,
        error: null,
    });
    let unsubscribe: (() => void) | null = null;

    let showParticipants = $state(false);
    let showChat = $state(false);
    let chatInput = $state('');
    let chatContainer: HTMLDivElement | null = $state(null);

    let leaving = $state(false);
    let endedNoticeShown = $state(false);

    let remoteParticipants = $derived(Object.values(clientState.participants) as RemoteParticipant[]);
    let tileCount = $derived(1 + remoteParticipants.length);
    let gridCols = $derived(Math.min(4, Math.ceil(Math.sqrt(tileCount || 1))));
    let rosterEntries = $derived(Object.values(clientState.roster));

    // Auto-scroll chat to bottom on new messages.
    $effect(() => {
        clientState.chatMessages.length;
        if (chatContainer) {
            setTimeout(() => {
                if (chatContainer) chatContainer.scrollTop = chatContainer.scrollHeight;
            }, 50);
        }
    });

    // Host ended the meeting: show a brief notice then leave automatically.
    $effect(() => {
        if (clientState.meetingEnded && !endedNoticeShown) {
            endedNoticeShown = true;
            setTimeout(() => {
                handleLeave();
            }, 2500);
        }
    });

    onMount(async () => {
        await userStore.init();
        if (!$userStore.isAuthenticated) {
            goto('/login');
            return;
        }

        try {
            meeting = await meetingService.getMeeting(meetingId);
        } catch (e) {
            console.error('Failed to load meeting:', e);
            loadError = 'This meeting could not be found or is no longer available.';
            loading = false;
            return;
        }

        const user = $userStore.user;
        if (!user) {
            loadError = 'You must be signed in to join this meeting.';
            loading = false;
            return;
        }

        const c = new MeetingClient(meetingId, user.id, user.userName);
        client = c;
        unsubscribe = c.store.subscribe((s) => {
            clientState = s;
        });

        try {
            await c.join();
            meetingStore.set(meetingId);
        } catch (e) {
            console.error('Failed to join meeting:', e);
            loadError =
                'Could not access your camera/microphone or connect to the meeting. Check permissions and try again.';
        } finally {
            loading = false;
        }
    });

    onDestroy(() => {
        unsubscribe?.();
        meetingStore.clear();
        if (client && !leaving) {
            client.leave().catch(() => {});
        }
    });

    async function handleLeave() {
        if (leaving) return;
        leaving = true;
        try {
            await client?.leave();
        } catch (e) {
            console.error('Error while leaving meeting:', e);
        } finally {
            goto('/messaging');
        }
    }

    async function handleToggleScreenShare() {
        if (!client) return;
        try {
            if (clientState.isScreenSharing) {
                await client.stopScreenShare();
            } else {
                await client.startScreenShare();
            }
        } catch (e) {
            console.error('Screen share failed:', e);
        }
    }

    async function handleToggleRecording() {
        if (!client) return;
        try {
            if (clientState.isRecording) {
                client.stopRecording();
            } else {
                await client.startRecording();
            }
        } catch (e) {
            console.error('Recording failed:', e);
        }
    }

    function handleSendChat(e: Event) {
        e.preventDefault();
        if (!chatInput.trim() || !client) return;
        client.sendChatMessage(chatInput);
        chatInput = '';
    }

    function initials(name: string): string {
        return (name || '?')
            .split(' ')
            .map((p) => p[0])
            .filter(Boolean)
            .slice(0, 2)
            .join('')
            .toUpperCase();
    }

    function formatTime(value: string): string {
        return new Date(value).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }

    function videoStream(node: HTMLVideoElement, stream: MediaStream | null) {
        node.srcObject = stream;
        return {
            update(newStream: MediaStream | null) {
                if (node.srcObject !== newStream) node.srcObject = newStream;
            },
            destroy() {
                node.srcObject = null;
            },
        };
    }
</script>

<svelte:head>
    <title>{meeting?.title || 'Meeting'} — Office ERP</title>
</svelte:head>

<div class="h-screen w-screen bg-neutral text-neutral-content flex flex-col overflow-hidden">
    {#if loading}
        <div class="flex-1 flex flex-col items-center justify-center gap-3">
            <Loader2 class="animate-spin" size={36} />
            <p class="font-medium">Connecting to meeting…</p>
        </div>
    {:else if loadError}
        <div class="flex-1 flex flex-col items-center justify-center gap-4 p-8 text-center">
            <AlertTriangle class="text-error" size={40} />
            <p class="max-w-sm opacity-80">{loadError}</p>
            <button class="btn btn-primary btn-sm" onclick={() => goto('/messaging')}>
                Back to Messaging
            </button>
        </div>
    {:else}
        <!-- Top bar -->
        <div class="h-14 flex items-center justify-between px-4 border-b border-neutral-content/10 shrink-0">
            <div class="flex items-center gap-2 min-w-0">
                <Video size={18} class="opacity-70 shrink-0" />
                <h1 class="font-semibold truncate">{meeting?.title || 'Untitled meeting'}</h1>
                {#if clientState.isRecording}
                    <span class="badge badge-error gap-1 badge-sm">
                        <Circle size={8} class="fill-current animate-pulse" />
                        REC
                    </span>
                {/if}
            </div>
            <div class="flex items-center gap-2 text-xs opacity-70 shrink-0">
                <span class="capitalize">{clientState.connectionState}</span>
                <span class="opacity-40">·</span>
                <span>{tileCount} in call</span>
            </div>
        </div>

        {#if clientState.meetingEnded}
            <div class="alert alert-warning rounded-none justify-center text-sm py-2">
                Meeting ended by host. Leaving…
            </div>
        {/if}

        <!-- Main area: video grid + optional side panels -->
        <div class="flex-1 flex overflow-hidden">
            <!-- Video grid -->
            <div class="flex-1 p-4 overflow-auto">
                <div
                    class="grid gap-3 h-full"
                    style="grid-template-columns: repeat({gridCols}, minmax(0, 1fr));">
                    <!-- Local tile -->
                    <div class="relative bg-black rounded-xl overflow-hidden aspect-video min-h-[160px]">
                        {#if clientState.localStream}
                            <!-- svelte-ignore a11y_media_has_caption -->
                            <video
                                use:videoStream={clientState.localStream}
                                autoplay
                                muted
                                playsinline
                                class="w-full h-full object-cover {clientState.isScreenSharing
                                    ? ''
                                    : '-scale-x-100'}"></video>
                        {/if}
                        {#if !clientState.cameraOn && !clientState.isScreenSharing}
                            <div class="absolute inset-0 flex items-center justify-center bg-neutral-800">
                                <div
                                    class="avatar avatar-placeholder w-16 h-16 rounded-full bg-primary text-primary-content flex items-center justify-center text-xl font-bold">
                                    {initials($userStore.user?.userName ?? 'You')}
                                </div>
                            </div>
                        {/if}
                        <div
                            class="absolute bottom-2 left-2 flex items-center gap-1.5 bg-black/50 rounded-lg px-2 py-1 text-xs">
                            <span>You</span>
                            {#if !clientState.micOn}
                                <MicOff size={12} class="text-error" />
                            {/if}
                        </div>
                    </div>

                    <!-- Remote tiles -->
                    {#each remoteParticipants as p (p.userId)}
                        <div class="relative bg-black rounded-xl overflow-hidden aspect-video min-h-[160px]">
                            {#if p.stream}
                                <!-- svelte-ignore a11y_media_has_caption -->
                                <video
                                    use:videoStream={p.stream}
                                    autoplay
                                    playsinline
                                    class="w-full h-full object-cover"></video>
                            {/if}
                            {#if !p.cameraOn || !p.stream}
                                <div class="absolute inset-0 flex items-center justify-center bg-neutral-800">
                                    <div
                                        class="avatar avatar-placeholder w-16 h-16 rounded-full bg-secondary text-secondary-content flex items-center justify-center text-xl font-bold">
                                        {initials(p.name)}
                                    </div>
                                </div>
                            {/if}
                            <div
                                class="absolute bottom-2 left-2 flex items-center gap-1.5 bg-black/50 rounded-lg px-2 py-1 text-xs">
                                <span>{p.name}</span>
                                {#if !p.micOn}
                                    <MicOff size={12} class="text-error" />
                                {/if}
                            </div>
                        </div>
                    {/each}
                </div>
            </div>

            <!-- Participants panel -->
            {#if showParticipants}
                <div class="w-72 border-l border-neutral-content/10 bg-neutral-900/40 flex flex-col shrink-0">
                    <div class="h-12 flex items-center justify-between px-4 border-b border-neutral-content/10">
                        <h2 class="font-semibold text-sm">Participants ({tileCount})</h2>
                        <button
                            class="btn btn-ghost btn-xs btn-square"
                            onclick={() => (showParticipants = false)}
                            aria-label="Close participants panel">
                            <X size={14} />
                        </button>
                    </div>
                    <ul class="flex-1 overflow-y-auto p-2 space-y-1">
                        <li class="flex items-center justify-between px-2 py-1.5 rounded text-sm">
                            <span>You</span>
                            {#if meeting?.host_id === $userStore.user?.id}
                                <Crown size={13} class="text-warning" />
                            {/if}
                        </li>
                        {#each rosterEntries as entry (entry.userId)}
                            <li class="flex items-center justify-between px-2 py-1.5 rounded text-sm">
                                <span class="truncate">{entry.name}</span>
                                {#if entry.role === 'host'}
                                    <Crown size={13} class="text-warning" />
                                {/if}
                            </li>
                        {/each}
                    </ul>
                </div>
            {/if}

            <!-- Chat panel -->
            {#if showChat}
                <div class="w-80 border-l border-neutral-content/10 bg-neutral-900/40 flex flex-col shrink-0">
                    <div class="h-12 flex items-center justify-between px-4 border-b border-neutral-content/10">
                        <h2 class="font-semibold text-sm">In-call chat</h2>
                        <button
                            class="btn btn-ghost btn-xs btn-square"
                            onclick={() => (showChat = false)}
                            aria-label="Close chat panel">
                            <X size={14} />
                        </button>
                    </div>
                    <div bind:this={chatContainer} class="flex-1 overflow-y-auto p-3 space-y-3">
                        {#if clientState.chatMessages.length === 0}
                            <p class="text-xs opacity-50 text-center mt-4">
                                No messages yet. Say hello!
                            </p>
                        {:else}
                            {#each clientState.chatMessages as msg, i (i)}
                                <div class="text-sm">
                                    <div class="flex items-baseline gap-2">
                                        <span class="font-semibold text-xs">{msg.fromName}</span>
                                        <span class="text-[10px] opacity-40">{formatTime(msg.sentAt)}</span>
                                    </div>
                                    <p class="opacity-90 break-words">{msg.text}</p>
                                </div>
                            {/each}
                        {/if}
                    </div>
                    <form onsubmit={handleSendChat} class="p-3 border-t border-neutral-content/10 flex gap-2">
                        <label class="sr-only" for="meeting-chat-input">Chat message</label>
                        <input
                            id="meeting-chat-input"
                            type="text"
                            bind:value={chatInput}
                            placeholder="Message everyone"
                            class="input input-bordered input-sm flex-1 bg-neutral-800" />
                        <button
                            type="submit"
                            class="btn btn-primary btn-sm btn-square"
                            disabled={!chatInput.trim()}
                            aria-label="Send message">
                            <Send size={14} />
                        </button>
                    </form>
                </div>
            {/if}
        </div>

        <!-- Control bar -->
        <div class="h-20 flex items-center justify-center gap-3 border-t border-neutral-content/10 shrink-0 px-4">
            <button
                class="btn btn-circle {clientState.micOn ? 'btn-neutral' : 'btn-error'}"
                onclick={() => client?.toggleMic()}
                title={clientState.micOn ? 'Mute microphone' : 'Unmute microphone'}
                aria-label={clientState.micOn ? 'Mute microphone' : 'Unmute microphone'}
                aria-pressed={!clientState.micOn}>
                {#if clientState.micOn}
                    <Mic size={18} />
                {:else}
                    <MicOff size={18} />
                {/if}
            </button>

            <button
                class="btn btn-circle {clientState.cameraOn ? 'btn-neutral' : 'btn-error'}"
                onclick={() => client?.toggleCamera()}
                title={clientState.cameraOn ? 'Turn off camera' : 'Turn on camera'}
                aria-label={clientState.cameraOn ? 'Turn off camera' : 'Turn on camera'}
                aria-pressed={!clientState.cameraOn}>
                {#if clientState.cameraOn}
                    <Video size={18} />
                {:else}
                    <VideoOff size={18} />
                {/if}
            </button>

            <button
                class="btn btn-circle {clientState.isScreenSharing ? 'btn-primary' : 'btn-neutral'}"
                onclick={handleToggleScreenShare}
                title={clientState.isScreenSharing ? 'Stop screen share' : 'Share screen'}
                aria-label={clientState.isScreenSharing ? 'Stop screen share' : 'Share screen'}
                aria-pressed={clientState.isScreenSharing}>
                {#if clientState.isScreenSharing}
                    <ScreenShareOff size={18} />
                {:else}
                    <ScreenShare size={18} />
                {/if}
            </button>

            <button
                class="btn btn-circle {clientState.isRecording ? 'btn-error' : 'btn-neutral'}"
                onclick={handleToggleRecording}
                title={clientState.isRecording ? 'Stop recording' : 'Start recording'}
                aria-label={clientState.isRecording ? 'Stop recording' : 'Start recording'}
                aria-pressed={clientState.isRecording}>
                <Circle size={18} class={clientState.isRecording ? 'fill-current animate-pulse' : ''} />
            </button>

            <div class="w-px h-8 bg-neutral-content/10 mx-1"></div>

            <button
                class="btn btn-circle {showParticipants ? 'btn-primary' : 'btn-neutral'}"
                onclick={() => (showParticipants = !showParticipants)}
                title="Participants"
                aria-label="Toggle participants panel"
                aria-pressed={showParticipants}>
                <Users size={18} />
            </button>

            <button
                class="btn btn-circle {showChat ? 'btn-primary' : 'btn-neutral'}"
                onclick={() => (showChat = !showChat)}
                title="Chat"
                aria-label="Toggle chat panel"
                aria-pressed={showChat}>
                <MessageSquare size={18} />
            </button>

            <div class="w-px h-8 bg-neutral-content/10 mx-1"></div>

            <button
                class="btn btn-circle btn-error"
                onclick={handleLeave}
                disabled={leaving}
                title="Leave meeting"
                aria-label="Leave meeting">
                {#if leaving}
                    <Loader2 size={18} class="animate-spin" />
                {:else}
                    <PhoneOff size={18} />
                {/if}
            </button>
        </div>
    {/if}
</div>
