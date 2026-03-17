<script lang="ts">
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        ClipboardList,
        Users,
        Plus,
        Trash2,
        RefreshCcw,
        Pen,
        PenIcon,
    } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import { projectService } from '$lib/services/project';
    import { userService } from '$lib/services/user-service';
    import type {
        Board,
        Card,
        Project,
        ProjectMember,
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
    let showCardModal = false;
    let IsCardEdit = false;
    let showMemberModal = false;
    let editingCardId: string | null = null;
let selectedCards: Card[] = [];
    let cardForm = {
        column_id: '',
        title: '',
        description: '',
        priority: 'medium',
        assignee_id: '',
        due_date: '',
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

    async function loadProjectData() {
        loading = true;
        errorMessage = '';
        try {
            project = await projectService.getById(projectId);
            board = await projectService.getBoard(projectId);
            cards = await projectService.listCards(projectId);
            members = await projectService.listMembers(projectId);
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
        } catch (error) {
            console.error('Failed to load project:', error);
            errorMessage = 'Failed to load project data';
        } finally {
            loading = false;
        }
    }

    function openCardModal(columnId: string) {
        cardForm = {
            column_id: columnId,
            title: '',
            description: '',
            priority: 'medium',
            assignee_id: '',
            due_date: '',
        };
        showCardModal = true;
        IsCardEdit = false;
    }
    function openEditCardModal(card:Card){
    editingCardId = card.id;
    cardForm = {
        column_id:card.column_id||"",
        title:card.title,
        description:card.description||"",
        priority:card.priority,
        assignee_id:card.assignee_id||"",
        due_date:card.due_date||""
    };
    showCardModal = true;
    IsCardEdit = true;
    }

    async function handleCreateUpdateCard() {
        if (!cardForm.title.trim()) return;
        try {
            if(!IsCardEdit){
            await projectService.createCard(projectId, {
                column_id: cardForm.column_id || undefined,
                title: cardForm.title,
                description: cardForm.description || undefined,
                priority: cardForm.priority,
                assignee_id: cardForm.assignee_id || undefined,
                due_date: cardForm.due_date || undefined,
            });
             
            }
       else{
        if(!editingCardId) return;
        await projectService.updateCard(projectId,editingCardId,{
            column_id: cardForm.column_id || undefined,
                title: cardForm.title,
                description: cardForm.description || undefined,
                priority: cardForm.priority,
                assignee_id: cardForm.assignee_id || undefined,
                due_date: cardForm.due_date || undefined,
        })
       }
            showCardModal = false;
            await loadProjectData();
        } catch (error) {
            console.error('Failed to create card:', error);
            errorMessage = 'Failed to create card';
        }
    }

    async function moveCard(card: Card, columnId: string) {
        try {
            await projectService.updateCard(projectId, card.id, {
                column_id: columnId,
            });
            cards = await projectService.listCards(projectId);
        } catch (error) {
            console.error('Failed to move card:', error);
            errorMessage = 'Failed to move card';
        }
    }

    async function deleteCard(card: Card) {
        if (!confirm('Delete this card?')) return;
        try {
            await projectService.deleteCard(projectId, card.id);
            cards = await projectService.listCards(projectId);
        } catch (error) {
            console.error('Failed to delete card:', error);
            errorMessage = 'Failed to delete card';
        }
    }

    async function handleAddMember() {
        if (!memberForm.user_id) return;
        try {
            await projectService.addMember(projectId, {
                user_id: memberForm.user_id,
                role: memberForm.role,
            });
            showMemberModal = false;
            members = await projectService.listMembers(projectId);
        } catch (error) {
            console.error('Failed to add member:', error);
            errorMessage = 'Failed to add member';
        }
    }

    function cardsForColumn(columnId: string) {
        return cards
            .filter((card) => card.column_id === columnId)
            .sort((a, b) => a.display_order - b.display_order);
    }

    function availableUsers() {
        const memberIds = new Set(members.map((m) => m.user_id));
        return users.filter((user) => !memberIds.has(user.id));
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
                <div class="flex items-center justify-between">
                    <div>
                        <div class="text-sm opacity-70">
                            {project.project_key} • {project.status}
                        </div>
                        <h2 class="text-xl font-semibold">{project.name}</h2>
                    </div>
                    <div class="text-sm capitalize">Role: {effectiveRole}</div>
                </div>

                <div
                    class="grid gap-4"
                    style={`grid-template-columns: repeat(${board.columns.length}, minmax(220px, 1fr));`}>
                    {#each board.columns as column}
                        <div class="bg-base-200 rounded-lg p-3 space-y-3">
                            <div class="flex items-center justify-between">
                                <h3 class="font-semibold">{column.name}</h3>
                                <button
                                    class="btn btn-xs btn-primary"
                                    disabled={!canCreateCards}
                                    on:click={() =>
                                        openCardModal(column.id)}>
                                    <Plus class="w-3 h-3 mr-1" /> Add
                                </button>
                            </div>

                            <div class="space-y-2">
                                {#each cardsForColumn(column.id) as card}
                                    <div class="card bg-base-100 border border-base-300 shadow-sm">
                                        <div class="card-body p-3 space-y-2">
                                            <div class="font-medium">
                                                {card.title}
                                                <button
                                                    class="btn btn-xs btn-ghost text-warning"
                                                    
                                                    on:click={() =>
                                                        openEditCardModal(card)}
                                                    title="Edit card">
                                                    <PenIcon class="w-3 h-3" />
                                                </button>
                                            </div>
                                          
                                            {#if card.description}
                                                <div class="text-xs opacity-70">
                                                    {card.description}
                                                </div>
                                            {/if}
                                            <div class="text-xs uppercase opacity-70">
                                                {card.priority}
                                            </div>
                                            <div class="text-xs">
                                                {card.assignee_name ||
                                                    'Unassigned'}
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
                                                        <option value={col.id}>
                                                            {col.name}
                                                        </option>
                                                    {/each}
                                                </select>
                                                <button
                                                    class="btn btn-xs btn-ghost text-error"
                                                    disabled={!canDeleteCards}
                                                    on:click={() =>
                                                        deleteCard(card)}
                                                    title="Delete card">
                                                    <Trash2 class="w-3 h-3" />
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                {/each}
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
                                    <div class="text-sm font-medium">
                                        {member.user_name}
                                    </div>
                                    <div class="text-xs opacity-70">
                                        {member.email}
                                    </div>
                                </div>
                                <div class="text-xs uppercase opacity-70">
                                    {member.role}
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
            <h3 class="font-bold text-lg mb-4">Add Card</h3>
            <form on:submit|preventDefault={handleCreateUpdateCard}>
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
                    <textarea
                        id="card-desc"
                        class="textarea textarea-bordered"
                        rows="3"
                        bind:value={cardForm.description}></textarea>
                </div>
                <div class="form-control mt-4">
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
                    </select>
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
                            <option value={member.user_id}>
                                {member.user_name}
                            </option>
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
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showCardModal = false)}>
                        Cancel
                    </button>
                    <button type="submit" class="btn btn-primary">
                        {#if IsCardEdit}
updateCard
{:else}
create
{/if}                    </button>
                </div>
            </form>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button type="button" on:click={() => (showCardModal = false)}>
                close
            </button>
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
                            <option value={user.id}>
                                {user.userName} ({user.email})
                            </option>
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
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showMemberModal = false)}>
                        Cancel
                    </button>
                    <button type="submit" class="btn btn-primary">
                        Add
                    </button>
                </div>
            </form>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button type="button" on:click={() => (showMemberModal = false)}>
                close
            </button>
        </form>
    </dialog>
{/if}
