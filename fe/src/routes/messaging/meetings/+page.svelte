<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { meetingService, type Meeting } from '$lib/services/meeting';
    import { Video, Plus, ArrowRight, Users } from 'lucide-svelte';

    let meetings: Meeting[] = $state([]);
    let loading = $state(true);
    let creating = $state(false);
    let joinInput = $state('');
    let joinError = $state('');

    onMount(async () => {
        await loadMeetings();
    });

    async function loadMeetings() {
        loading = true;
        try {
            meetings = await meetingService.listMyMeetings();
        } catch (error) {
            console.error('Failed to load meetings:', error);
        } finally {
            loading = false;
        }
    }

    async function handleNewMeeting() {
        if (creating) return;
        creating = true;
        try {
            const meeting = await meetingService.createMeeting({});
            goto(`/meetings/${meeting.id}`);
        } catch (error) {
            console.error('Failed to create meeting:', error);
        } finally {
            creating = false;
        }
    }

    function handleJoin(e: Event) {
        e.preventDefault();
        joinError = '';
        const raw = joinInput.trim();
        if (!raw) return;

        // Accept either a bare meeting id or a full /meetings/{id} URL.
        let id = raw;
        try {
            const url = new URL(raw);
            const parts = url.pathname.split('/').filter(Boolean);
            const idx = parts.indexOf('meetings');
            if (idx !== -1 && parts[idx + 1]) {
                id = parts[idx + 1];
            }
        } catch {
            // Not a URL — treat raw input as the id, possibly with a
            // "/meetings/<id>" prefix pasted without a scheme.
            const match = raw.match(/meetings\/([^/?#]+)/);
            if (match) id = match[1];
        }

        if (!id) {
            joinError = 'Enter a valid meeting ID or link.';
            return;
        }
        goto(`/meetings/${id}`);
    }

    function formatDate(value: string) {
        return new Date(value).toLocaleString([], {
            month: 'short',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
        });
    }
</script>

<div class="flex-1 overflow-y-auto p-8">
    <div class="max-w-3xl mx-auto space-y-8">
        <div class="flex items-center justify-between flex-wrap gap-4">
            <div>
                <h1 class="text-xl font-bold flex items-center gap-2">
                    <Video class="w-5 h-5 text-primary" />
                    Meetings
                </h1>
                <p class="text-sm text-base-content/60 mt-1">
                    Start an instant meeting or join one with an ID or link.
                </p>
            </div>
            <button
                class="btn btn-primary"
                onclick={handleNewMeeting}
                disabled={creating}>
                {#if creating}
                    <span class="loading loading-spinner loading-xs"></span>
                {:else}
                    <Plus class="w-4 h-4" />
                {/if}
                New meeting
            </button>
        </div>

        <!-- Join with a link or ID -->
        <form
            onsubmit={handleJoin}
            class="card bg-base-100 border border-base-300 p-4">
            <label for="join-meeting-input" class="label pt-0">
                <span class="label-text font-semibold"
                    >Join with a link or ID</span>
            </label>
            <div class="join w-full">
                <input
                    id="join-meeting-input"
                    type="text"
                    bind:value={joinInput}
                    placeholder="Paste a meeting link or ID"
                    class="input input-bordered join-item w-full" />
                <button type="submit" class="btn btn-neutral join-item">
                    Join
                    <ArrowRight class="w-4 h-4" />
                </button>
            </div>
            {#if joinError}
                <p class="text-error text-xs mt-2">{joinError}</p>
            {/if}
        </form>

        <!-- Recent meetings -->
        <div>
            <h2
                class="text-xs uppercase tracking-wider text-base-content/50 font-bold mb-3">
                Recent meetings
            </h2>

            {#if loading}
                <div class="flex justify-center p-8">
                    <span class="loading loading-spinner loading-md"></span>
                </div>
            {:else if meetings.length === 0}
                <div
                    class="flex flex-col items-center justify-center text-base-content/50 p-12 text-center border border-dashed border-base-300 rounded-xl">
                    <Video class="w-10 h-10 mb-3" />
                    <p>No meetings yet. Start one to get going.</p>
                </div>
            {:else}
                <div class="space-y-2">
                    {#each meetings as meeting}
                        <button
                            type="button"
                            class="w-full text-left card bg-base-100 border border-base-300 p-4 hover:border-primary transition-colors flex-row items-center justify-between gap-4"
                            onclick={() => goto(`/meetings/${meeting.id}`)}>
                            <div class="min-w-0">
                                <div class="flex items-center gap-2">
                                    <span class="font-semibold truncate"
                                        >{meeting.title || 'Untitled meeting'}</span>
                                    <span
                                        class="badge badge-sm {meeting.status ===
                                        'active'
                                            ? 'badge-success'
                                            : 'badge-neutral'}">
                                        {meeting.status}
                                    </span>
                                </div>
                                <p
                                    class="text-xs text-base-content/50 mt-1 flex items-center gap-1">
                                    <Users class="w-3 h-3" />
                                    Hosted by {meeting.host_name || 'Unknown'} · {formatDate(
                                        meeting.created_at,
                                    )}
                                </p>
                            </div>
                            <ArrowRight
                                class="w-4 h-4 text-base-content/30 shrink-0" />
                        </button>
                    {/each}
                </div>
            {/if}
        </div>
    </div>
</div>
