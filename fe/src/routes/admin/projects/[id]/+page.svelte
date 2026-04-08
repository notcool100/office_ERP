<script lang="ts">
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import RichTextEditor from '$lib/components/RichTextEditor.svelte';
    import {
        Home,
        ClipboardList,
        Users,
        Plus,
        Trash2,
        RefreshCcw,
        Edit,
        Eye,
        MessageSquare,
        Paperclip,
        History,
        Download,
        Upload,
        Calendar,
        CalendarRange,
    } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import { projectService } from '$lib/services/project';
    import { userService } from '$lib/services/user-service';
    import type {
        Board,
        Card,
        Project,
        ProjectMember,
        CardComment,
        CardAttachment,
        CardActivity,
        Sprint,
        CreateSprintDto,
        UpdateSprintDto,
    } from '$lib/types/project';
    import type { User } from '$lib/types/user';
    import {
        navigationStore,
        canCreate,
        canUpdate,
        canDelete,
    } from '$lib/stores/navigation';

    const navPath = '/admin/projects';

    let projectId = '';
    let project: Project | null = null;
    let board: Board | null = null;
    let cards: Card[] = [];
    let members: ProjectMember[] = [];
    let users: User[] = [];
    let loading = true;
    let errorMessage = '';

    let sprints: Sprint[] = [];
    let showSprintModal = false;
    let isSprintEdit = false;
    let editingSprintId: string | null = null;
    let sprintForm = {
        name: '',
        goal: '',
        start_date: '',
        end_date: '',
        status: 'planning',
    };

    let showCardModal = false;
    let isCardEdit = false;
    let editingCardId: string | null = null;

    let showMemberModal = false;

    let showCardDetailModal = false;
    let selectedCard: Card | null = null;
    let cardComments: CardComment[] = [];
    let cardAttachments: CardAttachment[] = [];
    let cardHistory: CardActivity[] = [];
    let detailsLoading = false;
    let detailsError = '';

    let newComment = '';
    let selectedAttachmentFile: File | null = null;
    let uploadInput: HTMLInputElement | null = null;
    let attachmentUploading = false;

    let sprintFilter = 'all';

    let cardForm = {
        column_id: '',
        title: '',
        description: '',
        sprint_name: '',
        priority: 'medium',
        assignee_id: '',
        due_date: '',
        sprint_id: '',
    };

    let memberForm = {
        user_id: '',
        role: 'member',
    };

    pageTitle.set({
        title: 'Project Board',
        desc: 'Manage tasks and assignments',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Projects', icon: ClipboardList },
    ]);

    $: projectId = $page.params.id;
    $: canCreateHere = canCreate(navPath, $navigationStore);
    $: canUpdateHere = canUpdate(navPath, $navigationStore);
    $: canDeleteHere = canDelete(navPath, $navigationStore);
    $: effectiveRole = project?.member_role || 'admin';
    $: canWriteRole = ['owner', 'admin', 'member'].includes(effectiveRole);
    $: canCreateCards = canCreateHere && canWriteRole;
    $: canUpdateCards = canUpdateHere && canWriteRole;
    $: canDeleteCards = canDeleteHere && canWriteRole;
    $: canManageMembers =
        canUpdateHere && ['owner', 'admin'].includes(effectiveRole);
    $: canManageSprints =
        canUpdateHere && ['owner', 'admin'].includes(effectiveRole);
    $: sprintOptions = sprints.sort((a, b) => b.created_at.localeCompare(a.created_at));

    async function loadProjectData() {
        loading = true;
        errorMessage = '';
        try {
            project = await projectService.getById(projectId);
            board = await projectService.getBoard(projectId);
            cards = await projectService.listCards(projectId);
            members = await projectService.listMembers(projectId);
            sprints = await projectService.listSprints(projectId);
            users = await userService.getAll();

            if (project) {
                pageTitle.set({
                    title: `${project.project_key}: ${project.name}`,
                    desc: 'Project board and tasks',
                });
                breadcrumb.set([
                    { label: 'Home', icon: Home },
                    { label: 'Projects', icon: ClipboardList },
                    { label: project.name, icon: ClipboardList },
                ]);
            }

            const currentSprints = new Set(
                cards
                    .map((card) => card.sprint_name)
                    .filter((value): value is string => !!value),
            );
            if (sprintFilter !== 'all' && !currentSprints.has(sprintFilter)) {
                sprintFilter = 'all';
            }
        } catch (error) {
            console.error('Failed to load project:', error);
            errorMessage = 'Failed to load project data';
        } finally {
            loading = false;
        }
    }

    function openCreateCardModal(columnId: string) {
        cardForm = {
            column_id: columnId,
            title: '',
            description: '',
            sprint_name: '',
            priority: 'medium',
            assignee_id: '',
            due_date: '',
            sprint_id: sprintFilter !== 'all' ? sprintFilter : '',
        };
        editingCardId = null;
        isCardEdit = false;
        showCardModal = true;
    }

    function openEditCardModal(card: Card) {
        editingCardId = card.id;
        cardForm = {
            column_id: card.column_id || '',
            title: card.title,
            description: card.description || '',
            sprint_name: card.sprint_name || '',
            priority: card.priority,
            assignee_id: card.assignee_id || '',
            due_date: card.due_date || '',
            sprint_id: card.sprint_id || '',
        };
        isCardEdit = true;
        showCardModal = true;
    }

    async function handleCreateOrUpdateCard() {
        if (!cardForm.title.trim()) {
            return;
        }

        errorMessage = '';
        try {
            if (isCardEdit && editingCardId) {
                await projectService.updateCard(projectId, editingCardId, {
                    column_id: cardForm.column_id || undefined,
                    title: cardForm.title,
                    description: cardForm.description || undefined,
                    sprint_name: cardForm.sprint_name || undefined,
                    priority: cardForm.priority,
                    assignee_id: cardForm.assignee_id || undefined,
                    due_date: cardForm.due_date || undefined,
                    sprint_id: cardForm.sprint_id || undefined,
                });
            } else {
                await projectService.createCard(projectId, {
                    column_id: cardForm.column_id || undefined,
                    title: cardForm.title,
                    description: cardForm.description || undefined,
                    sprint_name: cardForm.sprint_name || undefined,
                    priority: cardForm.priority,
                    assignee_id: cardForm.assignee_id || undefined,
                    due_date: cardForm.due_date || undefined,
                    sprint_id: cardForm.sprint_id || undefined,
                });
            }

            showCardModal = false;
            await loadProjectData();

            if (selectedCard) {
                const refreshed = cards.find((card) => card.id === selectedCard?.id) || null;
                selectedCard = refreshed;
                if (refreshed) {
                    await loadCardDetails(refreshed.id);
                }
            }
        } catch (error) {
            console.error('Failed to save card:', error);
            errorMessage = 'Failed to save card';
        }
    }

    async function moveCard(card: Card, columnId: string) {
        errorMessage = '';
        try {
            await projectService.updateCard(projectId, card.id, {
                column_id: columnId,
            });
            cards = await projectService.listCards(projectId);

            if (selectedCard?.id === card.id) {
                selectedCard = cards.find((current) => current.id === card.id) || null;
                await loadCardDetails(card.id);
            }
        } catch (error) {
            console.error('Failed to move card:', error);
            errorMessage = 'Failed to move card';
        }
    }

    async function deleteCard(card: Card) {
        if (!confirm(`Delete card ${card.card_key}?`)) {
            return;
        }

        errorMessage = '';
        try {
            await projectService.deleteCard(projectId, card.id);
            cards = await projectService.listCards(projectId);
            if (selectedCard?.id === card.id) {
                closeCardDetails();
            }
        } catch (error) {
            console.error('Failed to delete card:', error);
            errorMessage = 'Failed to delete card';
        }
    }

    async function openCardDetails(card: Card) {
        selectedCard = card;
        showCardDetailModal = true;
        await loadCardDetails(card.id);
    }

    function closeCardDetails() {
        showCardDetailModal = false;
        selectedCard = null;
        cardComments = [];
        cardAttachments = [];
        cardHistory = [];
        detailsError = '';
        newComment = '';
        selectedAttachmentFile = null;
        if (uploadInput) {
            uploadInput.value = '';
        }
    }

    async function loadCardDetails(cardId: string) {
        detailsLoading = true;
        detailsError = '';
        try {
            const [comments, attachments, history] = await Promise.all([
                projectService.listCardComments(projectId, cardId),
                projectService.listCardAttachments(projectId, cardId),
                projectService.listCardHistory(projectId, cardId),
            ]);
            cardComments = comments;
            cardAttachments = attachments;
            cardHistory = history;
        } catch (error) {
            console.error('Failed to load card details:', error);
            detailsError = 'Failed to load card details';
        } finally {
            detailsLoading = false;
        }
    }

    async function addComment() {
        if (!selectedCard || !newComment.trim()) {
            return;
        }

        detailsError = '';
        try {
            await projectService.createCardComment(projectId, selectedCard.id, {
                comment: newComment.trim(),
            });
            newComment = '';
            await loadCardDetails(selectedCard.id);
        } catch (error) {
            console.error('Failed to add comment:', error);
            detailsError = 'Failed to add comment';
        }
    }

    function onAttachmentSelected(event: Event) {
        const input = event.currentTarget as HTMLInputElement;
        const file = input.files?.[0] || null;
        selectedAttachmentFile = file;
    }

    async function uploadAttachment() {
        if (!selectedCard || !selectedAttachmentFile) {
            return;
        }

        attachmentUploading = true;
        detailsError = '';
        try {
            await projectService.uploadCardAttachment(
                projectId,
                selectedCard.id,
                selectedAttachmentFile,
            );
            selectedAttachmentFile = null;
            if (uploadInput) {
                uploadInput.value = '';
            }
            await loadCardDetails(selectedCard.id);
        } catch (error) {
            console.error('Failed to upload attachment:', error);
            detailsError = 'Failed to upload attachment';
        } finally {
            attachmentUploading = false;
        }
    }

    async function downloadAttachment(attachment: CardAttachment, openInNewTab = false) {
        if (!selectedCard) {
            return;
        }

        detailsError = '';
        try {
            const { blob, fileName } = await projectService.downloadCardAttachment(
                projectId,
                selectedCard.id,
                attachment.id,
            );

            const url = URL.createObjectURL(blob);
            if (openInNewTab) {
                window.open(url, '_blank');
                setTimeout(() => URL.revokeObjectURL(url), 10_000);
                return;
            }

            const link = document.createElement('a');
            link.href = url;
            link.download = fileName || attachment.file_name;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
            URL.revokeObjectURL(url);
        } catch (error) {
            console.error('Failed to download attachment:', error);
            detailsError = 'Failed to download attachment';
        }
    }

    async function handleAddMember() {
        if (!memberForm.user_id) {
            return;
        }

        errorMessage = '';
        try {
            await projectService.addMember(projectId, {
                user_id: memberForm.user_id,
                role: memberForm.role,
            });
            showMemberModal = false;
            memberForm = {
                user_id: '',
                role: 'member',
            };
            members = await projectService.listMembers(projectId);
        } catch (error) {
            console.error('Failed to add member:', error);
            errorMessage = 'Failed to add member';
        }
    }

    function cardsForColumn(columnId: string) {
        return cards
            .filter((card) => card.column_id === columnId)
            .filter((card) => sprintFilter === 'all' || card.sprint_name === sprintFilter)
            .sort((a, b) => a.display_order - b.display_order || a.sequence_no - b.sequence_no);
    }

    function availableUsers() {
        const memberIds = new Set(members.map((member) => member.user_id));
        return users.filter((user) => !memberIds.has(user.id));
    }

    function formatDate(value: string | null | undefined) {
        if (!value) {
            return '-';
        }
        return new Date(value).toLocaleString();
    }

    function formatBytes(size: number) {
        if (size < 1024) return `${size} B`;
        if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
        return `${(size / (1024 * 1024)).toFixed(1)} MB`;
    }

    function getSprintName(sprintId: string | null) {
        if (!sprintId) return null;
        const sprint = sprints.find((s) => s.id === sprintId);
        return sprint ? sprint.name : null;
    }

    function openCreateSprintModal() {
        sprintForm = {
            name: '',
            goal: '',
            start_date: '',
            end_date: '',
            status: 'planning',
        };
        editingSprintId = null;
        isSprintEdit = false;
        showSprintModal = true;
    }

    function openEditSprintModal(sprint: Sprint) {
        editingSprintId = sprint.id;
        sprintForm = {
            name: sprint.name,
            goal: sprint.goal || '',
            start_date: sprint.start_date || '',
            end_date: sprint.end_date || '',
            status: sprint.status,
        };
        isSprintEdit = true;
        showSprintModal = true;
    }

    async function handleCreateOrUpdateSprint() {
        if (!sprintForm.name.trim()) return;

        errorMessage = '';
        try {
            if (isSprintEdit && editingSprintId) {
                await projectService.updateSprint(projectId, editingSprintId, sprintForm);
            } else {
                await projectService.createSprint(projectId, sprintForm);
            }
            showSprintModal = false;
            await loadProjectData();
        } catch (error) {
            console.error('Failed to save sprint:', error);
            errorMessage = 'Failed to save sprint';
        }
    }

    async function handleDeleteSprint(sprintId: string) {
        if (!confirm('Are you sure you want to delete this sprint? Cards will be moved to backlog.'))
            return;

        errorMessage = '';
        try {
            await projectService.deleteSprint(projectId, sprintId);
            await loadProjectData();
            if (sprintFilter === sprintId) {
                sprintFilter = 'all';
            }
        } catch (error) {
            console.error('Failed to delete sprint:', error);
            errorMessage = 'Failed to delete sprint';
        }
    }

    function stripHtml(html: string): string {
        const div = document.createElement('div');
        div.innerHTML = html;
        return div.textContent || div.innerText || '';
    }

    function handleDescriptionChange(event: CustomEvent<string>) {
        cardForm.description = event.detail;
    }

    onMount(() => {
        loadProjectData();
    });
</script>

<PageSection>
    <div class="flex items-center justify-between mb-4">
        <div class="text-sm text-error">{errorMessage}</div>
        <button class="btn btn-sm" on:click={loadProjectData}>
            <RefreshCcw class="w-4 h-4 mr-1" /> Refresh
        </button>
    </div>

    {#if loading}
        <div class="flex justify-center p-8">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    {:else if !project || !board}
        <div class="text-sm">Project not found.</div>
    {:else}
        <div class="grid grid-cols-1 xl:grid-cols-[2fr_1fr] gap-6">
            <div class="space-y-4">
                <div class="flex items-center justify-between gap-4 flex-wrap">
                    <div>
                        <div class="text-sm opacity-70">
                            {project.project_key} • {project.status}
                        </div>
                        <h2 class="text-xl font-semibold">{project.name}</h2>
                    </div>
                    <div class="flex items-center gap-3 flex-wrap">
                        <div class="form-control">
                            <label class="label py-0" for="sprint-filter">
                                <span class="label-text text-xs opacity-70">Sprint</span>
                            </label>
                            <select
                                id="sprint-filter"
                                class="select select-bordered select-sm"
                                bind:value={sprintFilter}>
                                <option value="all">All Sprints</option>
                                <option value="">No Sprint</option>
                                {#each sprints as sprint}
                                    <option value={sprint.id}>{sprint.name}</option>
                                {/each}
                            </select>
                        </div>
                        <div class="text-sm capitalize">Role: {effectiveRole}</div>
                    </div>
                </div>

                <div
                    class="grid gap-4"
                    style={`grid-template-columns: repeat(${board.columns.length}, minmax(240px, 1fr));`}>
                    {#each board.columns as column}
                        <div class="bg-base-200 rounded-lg p-3 space-y-3">
                            <div class="flex items-center justify-between">
                                <h3 class="font-semibold">{column.name}</h3>
                                <button
                                    class="btn btn-xs btn-primary"
                                    disabled={!canCreateCards}
                                    on:click={() => openCreateCardModal(column.id)}>
                                    <Plus class="w-3 h-3 mr-1" /> Add
                                </button>
                            </div>

                            <div class="space-y-2">
                                {#if cardsForColumn(column.id).length === 0}
                                    <div class="text-xs opacity-60 px-1 py-2">
                                        No cards in this column.
                                    </div>
                                {:else}
                                    {#each cardsForColumn(column.id) as card}
                                        <div class="card bg-base-100 border border-base-300 shadow-sm">
                                            <div class="card-body p-3 space-y-2">
                                                <div class="flex items-start justify-between gap-2">
                                                    <div>
                                                        <div class="text-[11px] font-mono opacity-70">
                                                            {card.card_key}
                                                        </div>
                                                        <div class="font-medium leading-tight">
                                                            {card.title}
                                                        </div>
                                                    </div>
                                                    <div class="join">
                                                        <button
                                                            class="btn btn-xs btn-ghost join-item"
                                                            title="Card details"
                                                            on:click={() => openCardDetails(card)}>
                                                            <Eye class="w-3 h-3" />
                                                        </button>
                                                        <button
                                                            class="btn btn-xs btn-ghost text-warning join-item"
                                                            disabled={!canUpdateCards}
                                                            title="Edit card"
                                                            on:click={() => openEditCardModal(card)}>
                                                            <Edit class="w-3 h-3" />
                                                        </button>
                                                    </div>
                                                </div>

                                                {#if card.description}
                                                    <div class="text-xs opacity-70 line-clamp-3">
                                                        {stripHtml(card.description)}
                                                    </div>
                                                {/if}

                                                <div class="flex flex-wrap gap-1 text-xs">
                                                    <span class="badge badge-outline badge-sm uppercase">
                                                        {card.priority}
                                                    </span>
                                                    {#if card.sprint_name}
                                                        <span class="badge badge-info badge-sm">
                                                            {card.sprint_name}
                                                        </span>
                                                    {/if}
                                                </div>

                                                <div class="text-xs">
                                                    {card.assignee_name || 'Unassigned'}
                                                </div>
                                                <div class="text-xs opacity-70">
                                                    Due: {formatDate(card.due_date)}
                                                </div>

                                                <div class="flex items-center gap-2">
                                                    <select
                                                        class="select select-xs select-bordered"
                                                        disabled={!canUpdateCards}
                                                        value={card.column_id || ''}
                                                        on:change={(event) =>
                                                            moveCard(
                                                                card,
                                                                (
                                                                    event.currentTarget as HTMLSelectElement
                                                                ).value,
                                                            )}>
                                                        {#each board.columns as col}
                                                            <option value={col.id}>{col.name}</option>
                                                        {/each}
                                                    </select>
                                                    <button
                                                        class="btn btn-xs btn-ghost text-error"
                                                        disabled={!canDeleteCards}
                                                        on:click={() => deleteCard(card)}
                                                        title="Delete card">
                                                        <Trash2 class="w-3 h-3" />
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                    {/each}
                                {/if}
                            </div>
                        </div>
                    {/each}
                </div>
            </div>

            <div class="space-y-4">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2">
                        <Users class="w-4 h-4" />
                        <h3 class="font-semibold">Team Members</h3>
                    </div>
                    <button
                        class="btn btn-xs btn-primary"
                        disabled={!canManageMembers}
                        on:click={() => (showMemberModal = true)}>
                        <Plus class="w-3 h-3 mr-1" /> Add
                    </button>
                </div>

                <div class="space-y-2">
                    {#if members.length === 0}
                        <div class="text-sm opacity-70">No members yet.</div>
                    {:else}
                        {#each members as member}
                            <div class="flex items-center justify-between bg-base-200 p-2 rounded">
                                <div>
                                    <div class="text-sm font-medium">{member.user_name}</div>
                                    <div class="text-xs opacity-70">{member.email}</div>
                                </div>
                                <div class="text-xs uppercase opacity-70">{member.role}</div>
                            </div>
                        {/each}
                    {/if}
                </div>

                <div class="divider"></div>

                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2">
                        <CalendarRange class="w-4 h-4" />
                        <h3 class="font-semibold">Sprints</h3>
                    </div>
                    <button
                        class="btn btn-xs btn-primary"
                        disabled={!canManageSprints}
                        on:click={openCreateSprintModal}>
                        <Plus class="w-3 h-3 mr-1" /> New
                    </button>
                </div>

                <div class="space-y-2">
                    {#if sprints.length === 0}
                        <div class="text-sm opacity-70">No sprints yet.</div>
                    {:else}
                        {#each sprints as sprint}
                            <div class="bg-base-200 p-2 rounded group">
                                <div class="flex items-start justify-between">
                                    <div class="flex-1 min-w-0">
                                        <div class="flex items-center gap-2">
                                            <div class="text-sm font-medium truncate">{sprint.name}</div>
                                            {#if sprint.status === 'active'}
                                                <div class="badge badge-success badge-xs">Active</div>
                                            {:else if sprint.status === 'completed'}
                                                <div class="badge badge-neutral badge-xs">Done</div>
                                            {:else}
                                                <div class="badge badge-ghost badge-xs">Planned</div>
                                            {/if}
                                        </div>
                                        {#if sprint.start_date || sprint.end_date}
                                            <div class="text-[10px] opacity-60 mt-0.5">
                                                {sprint.start_date || '?'} to {sprint.end_date || '?'}
                                            </div>
                                        {/if}
                                    </div>
                                    <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                                        <button
                                            class="btn btn-ghost btn-xs px-1"
                                            disabled={!canManageSprints}
                                            on:click={() => openEditSprintModal(sprint)}>
                                            <Edit class="w-3 h-3" />
                                        </button>
                                        <button
                                            class="btn btn-ghost btn-xs px-1 text-error"
                                            disabled={!canManageSprints}
                                            on:click={() => handleDeleteSprint(sprint.id)}>
                                            <Trash2 class="w-3 h-3" />
                                        </button>
                                    </div>
                                </div>
                            </div>
                        {/each}
                    {/if}
                </div>
            </div>
        </div>
    {/if}
</PageSection>

{#if showCardModal}
    <dialog class="modal modal-open">
        <div class="modal-box">
            <h3 class="font-bold text-lg mb-4">
                {isCardEdit ? 'Edit Card' : 'Create Card'}
            </h3>
            <form on:submit|preventDefault={handleCreateOrUpdateCard}>
                <div class="text-xs opacity-70 mb-3">
                    Card ID is auto-generated in format like
                    <span class="font-mono">pro-project-01</span>.
                </div>
                <div class="form-control">
                    <label class="label" for="card-title">
                        <span class="label-text">Title *</span>
                    </label>
                    <input
                        id="card-title"
                        type="text"
                        class="input input-bordered"
                        bind:value={cardForm.title}
                        required />
                </div>

                <div class="form-control mt-4">
                    <label class="label" for="card-desc">
                        <span class="label-text">Description</span>
                    </label>
                    <RichTextEditor
                        content={cardForm.description}
                        placeholder="Describe this card..."
                        on:change={handleDescriptionChange} />
                </div>

                <div class="grid grid-cols-2 gap-4 mt-4">
                    <div class="form-control">
                        <label class="label" for="card-sprint">
                            <span class="label-text">Sprint</span>
                        </label>
                        <select
                            id="card-sprint"
                            class="select select-bordered"
                            bind:value={cardForm.sprint_id}>
                            <option value="">No Sprint</option>
                            {#each sprints as sprint}
                                <option value={sprint.id}>{sprint.name}</option>
                            {/each}
                        </select>
                    </div>

                    <div class="form-control">
                        <label class="label" for="card-priority">
                            <span class="label-text">Priority</span>
                        </label>
                        <select
                            id="card-priority"
                            class="select select-bordered"
                            bind:value={cardForm.priority}>
                            <option value="low">Low</option>
                            <option value="medium">Medium</option>
                            <option value="high">High</option>
                            <option value="urgent">Urgent</option>
                        </select>
                    </div>
                </div>

                <div class="form-control mt-4">
                    <label class="label" for="card-assignee">
                        <span class="label-text">Assignee</span>
                    </label>
                    <select
                        id="card-assignee"
                        class="select select-bordered"
                        bind:value={cardForm.assignee_id}>
                        <option value="">Unassigned</option>
                        {#each members as member}
                            <option value={member.user_id}>{member.user_name}</option>
                        {/each}
                    </select>
                </div>

                <div class="form-control mt-4">
                    <label class="label" for="card-due">
                        <span class="label-text">Due Date</span>
                    </label>
                    <input
                        id="card-due"
                        type="date"
                        class="input input-bordered"
                        bind:value={cardForm.due_date} />
                </div>

                <div class="modal-action">
                    <button type="button" class="btn" on:click={() => (showCardModal = false)}>
                        Cancel
                    </button>
                    <button
                        type="submit"
                        class="btn btn-primary"
                        disabled={isCardEdit ? !canUpdateCards : !canCreateCards}>
                        {isCardEdit ? 'Update' : 'Create'}
                    </button>
                </div>
            </form>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button type="button" on:click={() => (showCardModal = false)}>close</button>
        </form>
    </dialog>
{/if}

{#if showCardDetailModal && selectedCard}
    <dialog class="modal modal-open">
        <div class="modal-box w-11/12 max-w-6xl">
            <div class="flex flex-wrap justify-between gap-4 mb-4">
                <div>
                    <div class="text-xs font-mono opacity-70">{selectedCard.card_key}</div>
                    <h3 class="font-bold text-lg">{selectedCard.title}</h3>
                    <div class="text-sm opacity-70 mt-1">
                        {selectedCard.assignee_name || 'Unassigned'} • Priority {selectedCard.priority}
                    </div>
                </div>
                <div class="flex flex-wrap items-center gap-2 mb-4">
                    {#if getSprintName(selectedCard.sprint_id)}
                        <div class="badge badge-outline gap-1">
                            <Calendar class="w-3 h-3" />
                            {getSprintName(selectedCard.sprint_id)}
                        </div>
                    {/if}
                    <span class="badge badge-outline">Due: {formatDate(selectedCard.due_date)}</span>
                </div>
            </div>

            {#if selectedCard.description}
                <div class="bg-base-200 rounded-lg p-3 text-sm mb-4 prose prose-sm max-w-none">
                    {@html selectedCard.description}
                </div>
            {/if}

            <div class="text-sm text-error mb-2">{detailsError}</div>

            {#if detailsLoading}
                <div class="flex justify-center p-8">
                    <span class="loading loading-spinner loading-lg"></span>
                </div>
            {:else}
                <div class="grid grid-cols-1 xl:grid-cols-[2fr_1fr] gap-4">
                    <div class="space-y-4">
                        <div class="bg-base-200 rounded-lg p-4 space-y-3">
                            <div class="flex items-center gap-2">
                                <Paperclip class="w-4 h-4" />
                                <h4 class="font-semibold">Attachments</h4>
                            </div>

                            <div class="flex flex-col sm:flex-row gap-2 items-start sm:items-center">
                                <input
                                    class="file-input file-input-bordered file-input-sm w-full"
                                    type="file"
                                    bind:this={uploadInput}
                                    on:change={onAttachmentSelected} />
                                <button
                                    class="btn btn-sm btn-primary"
                                    on:click={uploadAttachment}
                                    disabled={!selectedAttachmentFile || attachmentUploading || !canUpdateCards}>
                                    <Upload class="w-3 h-3 mr-1" /> Upload
                                </button>
                            </div>

                            {#if cardAttachments.length === 0}
                                <div class="text-xs opacity-70">No attachments yet.</div>
                            {:else}
                                <div class="space-y-2">
                                    {#each cardAttachments as attachment}
                                        <div class="bg-base-100 border border-base-300 rounded p-2 text-xs">
                                            <div class="flex items-start justify-between gap-3">
                                                <div>
                                                    <div class="font-medium">{attachment.file_name}</div>
                                                    <div class="opacity-70">
                                                        {formatBytes(attachment.file_size)} •
                                                        {attachment.uploader_name || 'Unknown'} •
                                                        {formatDate(attachment.created_at)}
                                                    </div>
                                                </div>
                                                <div class="join">
                                                    {#if attachment.content_type.startsWith('image/')}
                                                        <button
                                                            class="btn btn-xs join-item"
                                                            on:click={() =>
                                                                downloadAttachment(attachment, true)}>
                                                            <Eye class="w-3 h-3" />
                                                        </button>
                                                    {/if}
                                                    <button
                                                        class="btn btn-xs join-item"
                                                        on:click={() =>
                                                            downloadAttachment(attachment)}>
                                                        <Download class="w-3 h-3" />
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                    {/each}
                                </div>
                            {/if}
                        </div>

                        <div class="bg-base-200 rounded-lg p-4 space-y-3">
                            <div class="flex items-center gap-2">
                                <MessageSquare class="w-4 h-4" />
                                <h4 class="font-semibold">Comments</h4>
                            </div>

                            {#if cardComments.length === 0}
                                <div class="text-xs opacity-70">No comments yet.</div>
                            {:else}
                                <div class="space-y-2 max-h-64 overflow-y-auto pr-1">
                                    {#each cardComments as comment}
                                        <div class="bg-base-100 border border-base-300 rounded p-2 text-sm">
                                            <div class="text-xs opacity-70 mb-1">
                                                {comment.user_name} • {formatDate(comment.created_at)}
                                            </div>
                                            <div>{comment.comment}</div>
                                        </div>
                                    {/each}
                                </div>
                            {/if}

                            <textarea
                                class="textarea textarea-bordered"
                                rows="3"
                                placeholder="Write a comment..."
                                bind:value={newComment}></textarea>
                            <div class="flex justify-end">
                                <button
                                    class="btn btn-sm btn-primary"
                                    on:click={addComment}
                                    disabled={!newComment.trim() || !canUpdateCards}>
                                    Add Comment
                                </button>
                            </div>
                        </div>
                    </div>

                    <div class="bg-base-200 rounded-lg p-4 space-y-3">
                        <div class="flex items-center gap-2">
                            <History class="w-4 h-4" />
                            <h4 class="font-semibold">History</h4>
                        </div>

                        {#if cardHistory.length === 0}
                            <div class="text-xs opacity-70">No activity yet.</div>
                        {:else}
                            <div class="space-y-2 max-h-[32rem] overflow-y-auto pr-1">
                                {#each cardHistory as activity}
                                    <div class="bg-base-100 border border-base-300 rounded p-2 text-xs">
                                        <div class="font-medium">
                                            {activity.actor_name || 'System'} • {formatDate(activity.created_at)}
                                        </div>
                                        <div class="uppercase opacity-60">{activity.action_type}</div>
                                        <div class="mt-1 text-sm opacity-90">{activity.description}</div>
                                    </div>
                                {/each}
                            </div>
                        {/if}
                    </div>
                </div>
            {/if}

            <div class="modal-action">
                <button type="button" class="btn" on:click={closeCardDetails}>Close</button>
            </div>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button type="button" on:click={closeCardDetails}>close</button>
        </form>
    </dialog>
{/if}

{#if showMemberModal}
    <dialog class="modal modal-open">
        <div class="modal-box">
            <h3 class="font-bold text-lg mb-4">Add Member</h3>
            <form on:submit|preventDefault={handleAddMember}>
                <div class="form-control">
                    <label class="label" for="member-user">
                        <span class="label-text">User *</span>
                    </label>
                    <select
                        id="member-user"
                        class="select select-bordered"
                        bind:value={memberForm.user_id}
                        required>
                        <option value="">Select a user</option>
                        {#each availableUsers() as user}
                            <option value={user.id}>{user.userName} ({user.email})</option>
                        {/each}
                    </select>
                </div>
                <div class="form-control mt-4">
                    <label class="label" for="member-role">
                        <span class="label-text">Role</span>
                    </label>
                    <select
                        id="member-role"
                        class="select select-bordered"
                        bind:value={memberForm.role}>
                        <option value="member">Member</option>
                        <option value="admin">Admin</option>
                        <option value="viewer">Viewer</option>
                    </select>
                </div>
                <div class="modal-action">
                    <button type="button" class="btn" on:click={() => (showMemberModal = false)}>
                        Cancel
                    </button>
                    <button type="submit" class="btn btn-primary">Add</button>
                </div>
            </form>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button type="button" on:click={() => (showMemberModal = false)}>close</button>
        </form>
    </dialog>
{/if}

{#if showSprintModal}
    <dialog class="modal modal-open">
        <div class="modal-box max-w-lg">
            <h3 class="font-bold text-lg mb-4">
                {isSprintEdit ? 'Edit Sprint' : 'Create Sprint'}
            </h3>
            <form on:submit|preventDefault={handleCreateOrUpdateSprint}>
                <div class="form-control">
                    <label class="label" for="sprint-name">
                        <span class="label-text">Sprint Name *</span>
                    </label>
                    <input
                        type="text"
                        id="sprint-name"
                        class="input input-bordered"
                        bind:value={sprintForm.name}
                        required
                        placeholder="e.g. Sprint 1" />
                </div>

                <div class="form-control mt-4">
                    <label class="label" for="sprint-goal">
                        <span class="label-text">Sprint Goal</span>
                    </label>
                    <textarea
                        id="sprint-goal"
                        class="textarea textarea-bordered"
                        rows="2"
                        bind:value={sprintForm.goal}
                        placeholder="What are we achieving in this sprint?"></textarea>
                </div>

                <div class="grid grid-cols-2 gap-4 mt-4">
                    <div class="form-control">
                        <label class="label" for="sprint-start">
                            <span class="label-text">Start Date</span>
                        </label>
                        <input
                            type="date"
                            id="sprint-start"
                            class="input input-bordered"
                            bind:value={sprintForm.start_date} />
                    </div>
                    <div class="form-control">
                        <label class="label" for="sprint-end">
                            <span class="label-text">End Date</span>
                        </label>
                        <input
                            type="date"
                            id="sprint-end"
                            class="input input-bordered"
                            bind:value={sprintForm.end_date} />
                    </div>
                </div>

                {#if isSprintEdit}
                    <div class="form-control mt-4">
                        <label class="label" for="sprint-status">
                            <span class="label-text">Status</span>
                        </label>
                        <select
                            id="sprint-status"
                            class="select select-bordered"
                            bind:value={sprintForm.status}>
                            <option value="planning">Planning</option>
                            <option value="active">Active</option>
                            <option value="completed">Completed</option>
                        </select>
                    </div>
                {/if}

                <div class="modal-action">
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showSprintModal = false)}>
                        Cancel
                    </button>
                    <button type="submit" class="btn btn-primary">
                        {isSprintEdit ? 'Update' : 'Create'}
                    </button>
                </div>
            </form>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button type="button" on:click={() => (showSprintModal = false)}>close</button>
        </form>
    </dialog>
{/if}

<div class="drawer drawer-end contents">
    <input id="sprint-drawer" type="checkbox" class="drawer-toggle" />
</div>
