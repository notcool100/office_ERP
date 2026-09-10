<script lang="ts">
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    import { userStore } from '$lib/stores/user';
    import {
        meetingService,
        MeetingApiError,
        type Meeting,
        type MeetingAttachment,
        type MeetingMinutes,
    } from '$lib/services/meeting';
    import {
        ArrowLeft,
        Video,
        Download,
        Trash2,
        FileText,
        Film,
        Loader2,
        AlertTriangle,
        Save,
        Users,
    } from 'lucide-svelte';
    import RichTextEditor from '$lib/components/RichTextEditor.svelte';

    const meetingId = $page.params.id;

    let meeting: Meeting | null = $state(null);
    let attachments: MeetingAttachment[] = $state([]);
    let minutes: MeetingMinutes | null = $state(null);
    let minutesContent = $state('');

    let loading = $state(true);
    let notFound = $state(false);
    let forbidden = $state(false);
    let error = $state('');

    let uploading = $state(false);
    let deletingId: string | null = $state(null);
    let downloadingId: string | null = $state(null);
    let savingMinutes = $state(false);
    let minutesDirty = $state(false);
    let fileInput: HTMLInputElement | null = $state(null);

    let currentUserId = $derived($userStore.user?.id);
    let isHost = $derived.by(() => {
        const m = meeting;
        return m !== null && m.host_id === currentUserId;
    });

    onMount(async () => {
        await userStore.init();
        if (!$userStore.isAuthenticated) {
            goto('/login');
            return;
        }
        await loadAll();
    });

    async function loadAll() {
        loading = true;
        notFound = false;
        forbidden = false;
        error = '';

        try {
            meeting = await meetingService.getMeeting(meetingId);
        } catch (e) {
            console.error('Failed to load meeting:', e);
            notFound = true;
            loading = false;
            return;
        }

        try {
            const [attachmentList, minutesData] = await Promise.all([
                meetingService.listAttachments(meetingId),
                meetingService.getMinutes(meetingId),
            ]);
            attachments = attachmentList;
            minutes = minutesData;
            minutesContent = minutesData.content;
            minutesDirty = false;
        } catch (e) {
            if (e instanceof MeetingApiError && e.status === 403) {
                forbidden = true;
            } else {
                console.error('Failed to load meeting details:', e);
                error = 'Failed to load attachments and minutes.';
            }
        } finally {
            loading = false;
        }
    }

    function onFileSelected(event: Event) {
        const input = event.currentTarget as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) return;
        uploadFile(file);
    }

    async function uploadFile(file: File) {
        uploading = true;
        error = '';
        try {
            const attachment = await meetingService.uploadAttachment(meetingId, file, 'document');
            attachments = [attachment, ...attachments];
        } catch (e) {
            console.error('Failed to upload attachment:', e);
            error = 'Failed to upload the file.';
        } finally {
            uploading = false;
            if (fileInput) fileInput.value = '';
        }
    }

    async function handleDownload(attachment: MeetingAttachment) {
        downloadingId = attachment.id;
        error = '';
        try {
            await meetingService.downloadAttachment(meetingId, attachment.id, attachment.file_name);
        } catch (e) {
            console.error('Failed to download attachment:', e);
            error = 'Failed to download the file.';
        } finally {
            downloadingId = null;
        }
    }

    async function handleDelete(attachment: MeetingAttachment) {
        if (!confirm(`Delete "${attachment.file_name}"?`)) return;
        deletingId = attachment.id;
        error = '';
        try {
            await meetingService.deleteAttachment(meetingId, attachment.id);
            attachments = attachments.filter((a) => a.id !== attachment.id);
        } catch (e) {
            console.error('Failed to delete attachment:', e);
            error = 'Failed to delete the file.';
        } finally {
            deletingId = null;
        }
    }

    function canDelete(attachment: MeetingAttachment): boolean {
        return isHost || attachment.uploaded_by === currentUserId;
    }

    function handleMinutesChange(e: CustomEvent<string>) {
        minutesContent = e.detail;
        minutesDirty = true;
    }

    async function saveMinutes() {
        savingMinutes = true;
        error = '';
        try {
            minutes = await meetingService.updateMinutes(meetingId, minutesContent);
            minutesContent = minutes.content;
            minutesDirty = false;
        } catch (e) {
            console.error('Failed to save minutes:', e);
            error = 'Failed to save minutes.';
        } finally {
            savingMinutes = false;
        }
    }

    function formatDate(value: string) {
        return new Date(value).toLocaleString([], {
            month: 'short',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
        });
    }

    function formatBytes(size: number) {
        if (size < 1024) return `${size} B`;
        if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
        return `${(size / (1024 * 1024)).toFixed(1)} MB`;
    }
</script>

<svelte:head>
    <title>{meeting?.title || 'Meeting details'} — Office ERP</title>
</svelte:head>

<div class="flex-1 overflow-y-auto p-8">
    <div class="max-w-3xl mx-auto space-y-6">
        <div class="flex items-center gap-3">
            <button
                class="btn btn-ghost btn-sm btn-square"
                onclick={() => goto('/meetings')}
                aria-label="Back to meetings">
                <ArrowLeft class="w-4 h-4" />
            </button>
            <h1 class="text-xl font-bold flex items-center gap-2">
                <Video class="w-5 h-5 text-primary" />
                Meeting details
            </h1>
        </div>

        {#if loading}
            <div class="flex justify-center p-12">
                <span class="loading loading-spinner loading-md"></span>
            </div>
        {:else if notFound}
            <div
                class="flex flex-col items-center justify-center text-base-content/50 p-12 text-center border border-dashed border-base-300 rounded-xl gap-3">
                <AlertTriangle class="w-8 h-8" />
                <p>This meeting could not be found.</p>
                <button class="btn btn-primary btn-sm" onclick={() => goto('/meetings')}>
                    Back to meetings
                </button>
            </div>
        {:else if forbidden}
            <div
                class="flex flex-col items-center justify-center text-base-content/50 p-12 text-center border border-dashed border-base-300 rounded-xl gap-3">
                <AlertTriangle class="w-8 h-8" />
                <p>You're not a participant of this meeting, so you don't have access to its details.</p>
                <button class="btn btn-primary btn-sm" onclick={() => goto('/meetings')}>
                    Back to meetings
                </button>
            </div>
        {:else}
            <!-- Metadata -->
            <div class="card bg-base-100 border border-base-300 p-4">
                <div class="flex items-center justify-between flex-wrap gap-2">
                    <div>
                        <h2 class="font-semibold">{meeting?.title || 'Untitled meeting'}</h2>
                        <p class="text-xs text-base-content/50 mt-1 flex items-center gap-1">
                            <Users class="w-3 h-3" />
                            Hosted by {meeting?.host_name || 'Unknown'} · Created {meeting
                                ? formatDate(meeting.created_at)
                                : ''}
                        </p>
                    </div>
                    <div class="flex items-center gap-2">
                        <span
                            class="badge badge-sm {meeting?.status === 'active'
                                ? 'badge-success'
                                : 'badge-neutral'}">
                            {meeting?.status}
                        </span>
                        {#if meeting?.status === 'active'}
                            <button class="btn btn-primary btn-sm" onclick={() => goto(`/meetings/${meetingId}`)}>
                                Join
                            </button>
                        {/if}
                    </div>
                </div>
            </div>

            {#if error}
                <div class="alert alert-error text-sm py-2">{error}</div>
            {/if}

            <!-- Attachments -->
            <div class="card bg-base-100 border border-base-300 p-4 space-y-3">
                <div class="flex items-center justify-between flex-wrap gap-2">
                    <h2 class="font-semibold">Attachments</h2>
                    <div class="flex items-center gap-2">
                        <label class="sr-only" for="meeting-attachment-input">Upload file</label>
                        <input
                            id="meeting-attachment-input"
                            type="file"
                            bind:this={fileInput}
                            onchange={onFileSelected}
                            disabled={uploading}
                            class="file-input file-input-bordered file-input-sm" />
                        {#if uploading}
                            <span class="loading loading-spinner loading-xs"></span>
                        {/if}
                    </div>
                </div>

                {#if attachments.length === 0}
                    <p class="text-sm text-base-content/50 italic">No attachments yet.</p>
                {:else}
                    <ul class="space-y-2">
                        {#each attachments as attachment (attachment.id)}
                            <li
                                class="flex items-center gap-3 p-3 bg-base-200 rounded-lg border border-base-300">
                                <div
                                    class="w-9 h-9 rounded bg-base-300 flex items-center justify-center shrink-0">
                                    {#if attachment.category === 'recording'}
                                        <Film class="w-4 h-4 opacity-60" />
                                    {:else}
                                        <FileText class="w-4 h-4 opacity-60" />
                                    {/if}
                                </div>
                                <div class="min-w-0 flex-1">
                                    <p class="text-sm font-medium truncate" title={attachment.file_name}>
                                        {attachment.file_name}
                                    </p>
                                    <p class="text-xs text-base-content/50">
                                        {formatBytes(attachment.file_size)} · Uploaded by {attachment.uploaded_by_name ||
                                            'Unknown'} · {formatDate(attachment.created_at)}
                                    </p>
                                </div>
                                <div class="flex items-center gap-1 shrink-0">
                                    <button
                                        class="btn btn-ghost btn-xs btn-square"
                                        onclick={() => handleDownload(attachment)}
                                        disabled={downloadingId === attachment.id}
                                        title="Download"
                                        aria-label="Download {attachment.file_name}">
                                        {#if downloadingId === attachment.id}
                                            <Loader2 class="w-4 h-4 animate-spin" />
                                        {:else}
                                            <Download class="w-4 h-4" />
                                        {/if}
                                    </button>
                                    {#if canDelete(attachment)}
                                        <button
                                            class="btn btn-ghost btn-xs btn-square text-error"
                                            onclick={() => handleDelete(attachment)}
                                            disabled={deletingId === attachment.id}
                                            title="Delete"
                                            aria-label="Delete {attachment.file_name}">
                                            {#if deletingId === attachment.id}
                                                <Loader2 class="w-4 h-4 animate-spin" />
                                            {:else}
                                                <Trash2 class="w-4 h-4" />
                                            {/if}
                                        </button>
                                    {/if}
                                </div>
                            </li>
                        {/each}
                    </ul>
                {/if}
            </div>

            <!-- Minutes -->
            <div class="card bg-base-100 border border-base-300 p-4 space-y-3">
                <div class="flex items-center justify-between flex-wrap gap-2">
                    <h2 class="font-semibold">Minutes</h2>
                    {#if minutes?.updated_at}
                        <p class="text-xs text-base-content/50">
                            Last updated by {minutes.updated_by_name || 'Unknown'} · {formatDate(
                                minutes.updated_at,
                            )}
                        </p>
                    {/if}
                </div>
                <RichTextEditor
                    content={minutesContent}
                    placeholder="Write meeting minutes here…"
                    on:change={handleMinutesChange} />
                <div class="flex justify-end">
                    <button
                        class="btn btn-primary btn-sm"
                        onclick={saveMinutes}
                        disabled={savingMinutes || !minutesDirty}>
                        {#if savingMinutes}
                            <span class="loading loading-spinner loading-xs"></span>
                        {:else}
                            <Save class="w-4 h-4" />
                        {/if}
                        Save minutes
                    </button>
                </div>
            </div>
        {/if}
    </div>
</div>
