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
        Layers,
        FileText,
        Bug,
        Square,
        CheckSquare,
        Link,
        AlertCircle,
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
        CardLink,
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
    let cardLinks: CardLink[] = [];
    let detailTab: 'details' | 'links' | 'comments' | 'attachments' | 'history' = 'details';
    let detailsLoading = false;
    let detailsError = '';

    let newComment = '';
    let selectedAttachmentFiles: File[] = [];
    let uploadInput: HTMLInputElement | null = null;
    let attachmentUploading = false;
    let attachmentPreviews: Record<string, string> = {};
    let newCardAttachments: { file: File, previewUrl: string }[] = [];

    let sprintFilter = 'all';
    let activeMainTab: 'board' | 'map' = 'board';

    let cardForm = {
        column_id: '',
        title: '',
        description: '',
        sprint_name: '',
        priority: 'medium',
        card_type: 'task',
        parent_id: '',
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

    function closeCardFormModal() {
        showCardModal = false;
        newCardAttachments.forEach(a => {
            if (a.previewUrl) URL.revokeObjectURL(a.previewUrl);
        });
        newCardAttachments = [];
    }

    function openCreateCardModal(columnId: string) {
        cardForm = {
            column_id: columnId,
            title: '',
            description: '',
            sprint_name: '',
            priority: 'medium',
            card_type: 'task',
            parent_id: '',
            assignee_id: '',
            due_date: '',
            sprint_id: sprintFilter !== 'all' ? sprintFilter : '',
        };
        editingCardId = null;
        isCardEdit = false;
        newCardAttachments = [];
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
            card_type: card.card_type,
            parent_id: card.parent_id || '',
            assignee_id: card.assignee_id || '',
            due_date: card.due_date || '',
            sprint_id: card.sprint_id || '',
        };
        isCardEdit = true;
        newCardAttachments = [];
        showCardModal = true;
    }

    function onNewCardAttachmentSelected(event: Event) {
        const input = event.currentTarget as HTMLInputElement;
        const files = Array.from(input.files || []);
        for (const file of files) {
            newCardAttachments = [...newCardAttachments, {
                file,
                previewUrl: file.type.startsWith('image/') ? URL.createObjectURL(file) : ''
            }];
        }
        input.value = '';
    }

    function removeNewCardAttachment(index: number) {
        const removed = newCardAttachments[index];
        if (removed.previewUrl) {
            URL.revokeObjectURL(removed.previewUrl);
        }
        newCardAttachments = newCardAttachments.filter((_, i) => i !== index);
    }

    async function handleCreateOrUpdateCard() {
        if (!cardForm.title.trim()) {
            return;
        }

        errorMessage = '';
        try {
            let savedCard: Card;
            if (isCardEdit && editingCardId) {
                savedCard = await projectService.updateCard(projectId, editingCardId, {
                    column_id: cardForm.column_id || undefined,
                    title: cardForm.title,
                    description: cardForm.description || undefined,
                    sprint_name: cardForm.sprint_name || undefined,
                    priority: cardForm.priority,
                    card_type: cardForm.card_type,
                    parent_id: cardForm.parent_id || undefined,
                    assignee_id: cardForm.assignee_id || undefined,
                    due_date: cardForm.due_date || undefined,
                    sprint_id: cardForm.sprint_id || undefined,
                });
            } else {
                savedCard = await projectService.createCard(projectId, {
                    column_id: cardForm.column_id || undefined,
                    title: cardForm.title,
                    description: cardForm.description || undefined,
                    sprint_name: cardForm.sprint_name || undefined,
                    priority: cardForm.priority,
                    card_type: cardForm.card_type,
                    parent_id: cardForm.parent_id || undefined,
                    assignee_id: cardForm.assignee_id || undefined,
                    due_date: cardForm.due_date || undefined,
                    sprint_id: cardForm.sprint_id || undefined,
                });
            }

            if (newCardAttachments.length > 0) {
                for (const attachment of newCardAttachments) {
                    await projectService.uploadCardAttachment(projectId, savedCard.id, attachment.file);
                }
            }

            closeCardFormModal();
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
        cardLinks = [];
        detailTab = 'details';
        detailsError = '';
        newComment = '';
        selectedAttachmentFiles = [];
        Object.values(attachmentPreviews).forEach(URL.revokeObjectURL);
        attachmentPreviews = {};
        if (uploadInput) {
            uploadInput.value = '';
        }
    }

    async function loadCardDetails(cardId: string) {
        detailsLoading = true;
        detailsError = '';
        try {
            const [comments, attachments, history, links] = await Promise.all([
                projectService.listCardComments(projectId, cardId),
                projectService.listCardAttachments(projectId, cardId),
                projectService.listCardHistory(projectId, cardId),
                projectService.listCardLinks(projectId, cardId),
            ]);
            cardComments = comments;
            cardAttachments = attachments;
            cardHistory = history;
            cardLinks = links;

            for (const att of attachments) {
                if (att.content_type.startsWith('image/') && !attachmentPreviews[att.id]) {
                    projectService.downloadCardAttachment(projectId, cardId, att.id)
                        .then(({ blob }) => {
                            attachmentPreviews[att.id] = URL.createObjectURL(blob);
                            attachmentPreviews = { ...attachmentPreviews };
                        })
                        .catch(err => console.error('Failed to load preview for attachment', att.id, err));
                }
            }
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
        selectedAttachmentFiles = Array.from(input.files || []);
    }

    async function uploadAttachment() {
        if (!selectedCard || selectedAttachmentFiles.length === 0) {
            return;
        }

        attachmentUploading = true;
        detailsError = '';
        try {
            for (const file of selectedAttachmentFiles) {
                await projectService.uploadCardAttachment(projectId, selectedCard.id, file);
            }
            selectedAttachmentFiles = [];
            if (uploadInput) {
                uploadInput.value = '';
            }
            await loadCardDetails(selectedCard.id);
        } catch (error) {
            console.error('Failed to upload attachments:', error);
            detailsError = 'Failed to upload attachments';
        } finally {
            attachmentUploading = false;
        }
    }

    async function addCardLink(targetCardId: string, linkType: string) {
        if (!selectedCard) return;
        detailsError = '';
        try {
            await projectService.createCardLink(projectId, selectedCard.id, {
                target_card_id: targetCardId,
                link_type: linkType,
            });
            await loadCardDetails(selectedCard.id);
        } catch (error) {
            console.error('Failed to add link:', error);
            detailsError = 'Failed to add card link';
        }
    }

    async function removeCardLink(linkId: string) {
        if (!selectedCard) return;
        detailsError = '';
        try {
            await projectService.deleteCardLink(projectId, selectedCard.id, linkId);
            await loadCardDetails(selectedCard.id);
        } catch (error) {
            console.error('Failed to remove link:', error);
            detailsError = 'Failed to remove card link';
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

    // Map View Logic
    let mapNodeRefs: Record<string, HTMLElement> = {};
    let mapSvgOverlay: SVGSVGElement | null = null;
    let mapLines: {id: string, x1: number, y1: number, x2: number, y2: number, color: string, type: string}[] = [];
    
    function drawMapLines() {
        if (activeMainTab !== 'map' || !mapSvgOverlay) return;
        
        requestAnimationFrame(() => {
            if (!mapSvgOverlay) return;
            const svgRect = mapSvgOverlay.getBoundingClientRect();
            const newLines = [];
            
            for (const card of cards) {
                if (card.parent_id && mapNodeRefs[card.id] && mapNodeRefs[card.parent_id]) {
                    const childRect = mapNodeRefs[card.id].getBoundingClientRect();
                    const parentRect = mapNodeRefs[card.parent_id].getBoundingClientRect();
                    
                    // Avoid drawing if nodes are hidden/collapsed
                    if (childRect.width === 0 || parentRect.width === 0) continue;
                    
                    newLines.push({
                        id: `v-${card.id}`,
                        x1: parentRect.left + 24 - svgRect.left,
                        y1: parentRect.bottom - svgRect.top - 5,
                        x2: parentRect.left + 24 - svgRect.left,
                        y2: childRect.top + (childRect.height / 2) - svgRect.top,
                        color: 'oklch(var(--bc) / 0.2)',
                        type: 'v'
                    });
                    
                    newLines.push({
                        id: `h-${card.id}`,
                        x1: parentRect.left + 24 - svgRect.left,
                        y1: childRect.top + (childRect.height / 2) - svgRect.top,
                        x2: childRect.left - svgRect.left - 5,
                        y2: childRect.top + (childRect.height / 2) - svgRect.top,
                        color: 'oklch(var(--bc) / 0.2)',
                        type: 'h'
                    });
                }
            }
            mapLines = newLines;
        });
    }

    $: if (activeMainTab === 'map' && cards.length >= 0) {
        setTimeout(drawMapLines, 50);
    }

    $: mapSprintsWithCards = sprints
        .filter(s => sprintFilter === 'all' || s.id === sprintFilter)
        .map(s => ({
            sprint: s,
            cards: cards.filter(c => c.sprint_id === s.id)
        }));
        
    $: unassignedMapCards = cards.filter(c => !c.sprint_id && (sprintFilter === 'all' || sprintFilter === ''));

    function getTopLevelCardsForGroup(groupCards: Card[]) {
        const groupCardIds = new Set(groupCards.map(c => c.id));
        return groupCards.filter(c => !c.parent_id || !groupCardIds.has(c.parent_id)).sort((a, b) => {
            if (a.card_type === 'epic' && b.card_type !== 'epic') return -1;
            if (b.card_type === 'epic' && a.card_type !== 'epic') return 1;
            return a.sequence_no - b.sequence_no;
        });
    }

    function getChildrenForGroup(groupCards: Card[], parentId: string) {
        return groupCards.filter(c => c.parent_id === parentId).sort((a,b) => a.sequence_no - b.sequence_no);
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
                    <div class="flex items-center gap-4">
                        <div>
                            <div class="text-sm opacity-70">
                                {project.project_key} • {project.status}
                            </div>
                            <h2 class="text-xl font-semibold">{project.name}</h2>
                        </div>
                        <div class="tabs tabs-boxed ml-4 bg-base-200/50">
                            <button class="tab {activeMainTab === 'board' ? 'tab-active' : ''}" on:click={() => activeMainTab = 'board'}>Board</button>
                            <button class="tab {activeMainTab === 'map' ? 'tab-active' : ''}" on:click={() => activeMainTab = 'map'}>Map</button>
                        </div>
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

                {#if activeMainTab === 'board'}
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
                                                        <div class="text-[11px] font-mono opacity-70 flex items-center gap-1">
                                                            {#if card.card_type === 'epic'}
                                                                <Layers class="w-2.5 h-2.5 text-purple-500" />
                                                            {:else if card.card_type === 'story'}
                                                                <FileText class="w-2.5 h-2.5 text-success" />
                                                            {:else if card.card_type === 'bug'}
                                                                <Bug class="w-2.5 h-2.5 text-error" />
                                                            {:else}
                                                                <CheckSquare class="w-2.5 h-2.5 text-info" />
                                                            {/if}
                                                            {card.card_key}
                                                            {#if card.parent_card_key}
                                                                <span class="opacity-50" title="Parent: {card.parent_card_key}">/ {card.parent_card_key}</span>
                                                            {/if}
                                                            {#if card.is_migrated}
                                                                <span class="text-warning" title="Migrated card">*</span>
                                                            {/if}
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
                {:else if activeMainTab === 'map'}
                    <div class="bg-base-200/30 rounded-xl border border-base-300 p-4 min-h-[600px] overflow-auto relative sprint-map-container"
                         on:scroll={() => requestAnimationFrame(drawMapLines)}>
                         
                        <svg bind:this={mapSvgOverlay} class="absolute top-0 left-0 w-full h-full pointer-events-none z-0">
                            {#each mapLines as line (line.id)}
                            	<path d={`M ${line.x1} ${line.y1} L ${line.x2} ${line.y2}`} stroke={line.color} stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" />
                            {/each}
                        </svg>

                        <div class="relative z-10 flex flex-col gap-8">
                            {#each mapSprintsWithCards as sprintGroup (sprintGroup.sprint.id)}
                                <!-- Only show sprint if it has cards or sprint filter is active on it -->
                                {#if sprintGroup.cards.length > 0 || sprintFilter === sprintGroup.sprint.id}
                                    <div class="flex flex-col gap-4">
                                        <div class="flex items-center gap-3 border-b border-base-300 pb-2">
                                            <div class="font-bold text-lg">{sprintGroup.sprint.name}</div>
                                            <div class="badge badge-sm badge-outline">{sprintGroup.cards.length} cards</div>
                                            {#if sprintGroup.sprint.goal}
                                                <div class="text-xs opacity-70 italic max-w-md truncate">{sprintGroup.sprint.goal}</div>
                                            {/if}
                                        </div>
                                        
                                        <div class="flex flex-col gap-2 pl-2">
                                            {#if sprintGroup.cards.length === 0}
                                                <div class="italic text-sm opacity-50 px-2">No cards in this sprint</div>
                                            {:else}
                                                {#each getTopLevelCardsForGroup(sprintGroup.cards) as topCard (topCard.id)}
                                                    <div class="flex flex-col">
                                                        <!-- Top Level (Epic / Story) -->
                                                        <div bind:this={mapNodeRefs[topCard.id]} class="card bg-base-100 border border-base-300 shadow w-64 cursor-pointer hover:border-primary transition-colors my-2" class:border-purple-500={topCard.card_type === 'epic'} class:border-success={topCard.card_type === 'story'} class:border-error={topCard.card_type === 'bug'} class:border-info={topCard.card_type === 'task'} on:click={() => openCardDetails(topCard)} role="button" tabindex="0" on:keydown={(e) => e.key === 'Enter' && openCardDetails(topCard)}>
                                                            <div class="p-3">
                                                                <div class="text-[10px] font-mono opacity-70 mb-1">{topCard.card_key} • <span class="uppercase">{topCard.card_type}</span></div>
                                                                <div class="font-medium text-sm leading-tight">{topCard.title}</div>
                                                                <div class="flex justify-between items-center mt-2">
                                                                    <div class="text-xs opacity-60">{topCard.assignee_name || 'Unassigned'}</div>
                                                                    <div class="badge badge-sm uppercase text-[9px]">{topCard.priority}</div>
                                                                </div>
                                                            </div>
                                                        </div>
                                                        <!-- Level 1 Children -->
                                                        {#each getChildrenForGroup(sprintGroup.cards, topCard.id) as child1 (child1.id)}
                                                            <div class="flex flex-col pl-12 relative">
                                                                <div bind:this={mapNodeRefs[child1.id]} class="card bg-base-100 border border-base-300 shadow w-64 cursor-pointer hover:border-primary transition-colors my-2" class:border-purple-500={child1.card_type === 'epic'} class:border-success={child1.card_type === 'story'} class:border-error={child1.card_type === 'bug'} class:border-info={child1.card_type === 'task'} on:click={() => openCardDetails(child1)} role="button" tabindex="0" on:keydown={(e) => e.key === 'Enter' && openCardDetails(child1)}>
                                                                    <div class="p-3">
                                                                        <div class="text-[10px] font-mono opacity-70 mb-1">{child1.card_key} • <span class="uppercase">{child1.card_type}</span></div>
                                                                        <div class="font-medium text-sm leading-tight">{child1.title}</div>
                                                                    </div>
                                                                </div>
                                                                <!-- Level 2 Children -->
                                                                {#each getChildrenForGroup(sprintGroup.cards, child1.id) as child2 (child2.id)}
                                                                    <div class="flex flex-col pl-12 relative">
                                                                        <div bind:this={mapNodeRefs[child2.id]} class="card bg-base-100 border border-base-300 shadow w-56 cursor-pointer hover:border-primary transition-colors my-1" class:border-purple-500={child2.card_type === 'epic'} class:border-success={child2.card_type === 'story'} class:border-error={child2.card_type === 'bug'} class:border-info={child2.card_type === 'task'} on:click={() => openCardDetails(child2)} role="button" tabindex="0" on:keydown={(e) => e.key === 'Enter' && openCardDetails(child2)}>
                                                                            <div class="p-2">
                                                                                <div class="text-[10px] font-mono opacity-70 mb-1">{child2.card_key} • <span class="uppercase">{child2.card_type}</span></div>
                                                                                <div class="font-medium text-xs leading-tight">{child2.title}</div>
                                                                            </div>
                                                                        </div>
                                                                    </div>
                                                                {/each}
                                                            </div>
                                                        {/each}
                                                    </div>
                                                {/each}
                                            {/if}
                                        </div>
                                    </div>
                                {/if}
                            {/each}
                            
                            <!-- Unassigned Cards -->
                            {#if unassignedMapCards.length > 0}
                                <div class="flex flex-col gap-4 opacity-80 mt-4">
                                    <div class="flex items-center gap-3 border-b border-base-300 pb-2">
                                        <div class="font-bold text-lg">Backlog / Unassigned</div>
                                        <div class="badge badge-sm badge-outline">{unassignedMapCards.length} cards</div>
                                    </div>
                                    <div class="flex flex-col gap-2 pl-2">
                                        {#each getTopLevelCardsForGroup(unassignedMapCards) as topCard (topCard.id)}
                                            <div class="flex flex-col">
                                                <div bind:this={mapNodeRefs[topCard.id]} class="card bg-base-100 border border-base-300 shadow w-64 cursor-pointer hover:border-primary transition-colors my-2" class:border-purple-500={topCard.card_type === 'epic'} class:border-success={topCard.card_type === 'story'} class:border-error={topCard.card_type === 'bug'} class:border-info={topCard.card_type === 'task'} on:click={() => openCardDetails(topCard)} role="button" tabindex="0" on:keydown={(e) => e.key === 'Enter' && openCardDetails(topCard)}>
                                                    <div class="p-3">
                                                        <div class="text-[10px] font-mono opacity-70 mb-1">{topCard.card_key} • <span class="uppercase">{topCard.card_type}</span></div>
                                                        <div class="font-medium text-sm leading-tight">{topCard.title}</div>
                                                    </div>
                                                </div>
                                                <!-- Level 1 Children -->
                                                {#each getChildrenForGroup(unassignedMapCards, topCard.id) as child1 (child1.id)}
                                                    <div class="flex flex-col pl-12 relative">
                                                        <div bind:this={mapNodeRefs[child1.id]} class="card bg-base-100 border border-base-300 shadow w-64 cursor-pointer hover:border-primary transition-colors my-2" class:border-purple-500={child1.card_type === 'epic'} class:border-success={child1.card_type === 'story'} class:border-error={child1.card_type === 'bug'} class:border-info={child1.card_type === 'task'} on:click={() => openCardDetails(child1)} role="button" tabindex="0" on:keydown={(e) => e.key === 'Enter' && openCardDetails(child1)}>
                                                            <div class="p-2">
                                                                <div class="text-[10px] font-mono opacity-70 mb-1">{child1.card_key}</div>
                                                                <div class="font-medium text-xs leading-tight">{child1.title}</div>
                                                            </div>
                                                        </div>
                                                        <!-- Level 2 Children -->
                                                        {#each getChildrenForGroup(unassignedMapCards, child1.id) as child2 (child2.id)}
                                                            <div class="flex flex-col pl-12 relative">
                                                                <div bind:this={mapNodeRefs[child2.id]} class="card bg-base-100 border border-base-300 shadow w-56 cursor-pointer hover:border-primary transition-colors my-1" class:border-purple-500={child2.card_type === 'epic'} class:border-success={child2.card_type === 'story'} class:border-error={child2.card_type === 'bug'} class:border-info={child2.card_type === 'task'} on:click={() => openCardDetails(child2)} role="button" tabindex="0" on:keydown={(e) => e.key === 'Enter' && openCardDetails(child2)}>
                                                                    <div class="p-2">
                                                                        <div class="text-[10px] font-mono opacity-70 mb-1">{child2.card_key}</div>
                                                                        <div class="font-medium text-[11px] leading-tight">{child2.title}</div>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        {/each}
                                                    </div>
                                                {/each}
                                            </div>
                                        {/each}
                                    </div>
                                </div>
                            {/if}
                        </div>
                    </div>
                {/if}
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
                        <label class="label" for="card-type">
                            <span class="label-text">Type</span>
                        </label>
                        <select
                            id="card-type"
                            class="select select-bordered w-full"
                            bind:value={cardForm.card_type}>
                            <option value="epic">Epic</option>
                            <option value="story">Story</option>
                            <option value="task">Task</option>
                            <option value="bug">Bug</option>
                        </select>
                    </div>
                </div>

                {#if cardForm.card_type !== 'epic'}
                    <div class="form-control mt-4">
                        <label class="label" for="parent-id">
                            <span class="label-text">
                                Parent {#if cardForm.card_type === 'story'}(Epic){:else if cardForm.card_type === 'task'}(Story){:else}(Story/Task){/if}
                            </span>
                        </label>
                        <select
                            id="parent-id"
                            class="select select-bordered w-full"
                            bind:value={cardForm.parent_id}>
                            <option value="">No Parent</option>
                            {#each (cardForm.card_type === 'story' ? cards.filter(c => c.card_type === 'epic') : (cardForm.card_type === 'task' ? cards.filter(c => c.card_type === 'story') : cards.filter(c => c.card_type === 'story' || c.card_type === 'task'))) as p}
                                <option value={p.id}>{p.card_key}: {p.title}</option>
                            {/each}
                        </select>
                    </div>
                {/if}

                <div class="grid grid-cols-2 gap-4 mt-4">
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
                    <div class="form-control">
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

                <div class="form-control mt-4">
                    <label class="label">
                        <span class="label-text">Attachments</span>
                    </label>
                    <input
                        type="file"
                        class="file-input file-input-bordered w-full"
                        multiple
                        on:change={onNewCardAttachmentSelected} />
                    {#if newCardAttachments.length > 0}
                        <div class="mt-3 flex flex-wrap gap-3">
                            {#each newCardAttachments as attachment, index}
                                <div class="bg-base-200 border border-base-300 rounded p-2 text-xs w-32 flex flex-col relative group">
                                    <button
                                        type="button"
                                        class="btn btn-xs btn-circle btn-neutral absolute -top-2 -right-2 opacity-0 group-hover:opacity-100 transition-opacity z-10"
                                        on:click={() => removeNewCardAttachment(index)}>✕</button>
                                    {#if attachment.previewUrl}
                                        <div class="h-20 w-full mb-2 bg-base-100 rounded flex items-center justify-center overflow-hidden">
                                            <img src={attachment.previewUrl} alt="Preview" class="object-cover w-full h-full" />
                                        </div>
                                    {:else}
                                        <div class="h-20 w-full mb-2 bg-base-100 rounded flex items-center justify-center">
                                            <Paperclip class="w-8 h-8 opacity-20" />
                                        </div>
                                    {/if}
                                    <div class="truncate text-center" title={attachment.file.name}>{attachment.file.name}</div>
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>

                <div class="modal-action">
                    <button type="button" class="btn" on:click={closeCardFormModal}>
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
            <button type="button" on:click={closeCardFormModal}>close</button>
        </form>
    </dialog>
{/if}

{#if showCardDetailModal && selectedCard}
    <dialog class="modal modal-open">
        <div class="modal-box w-11/12 max-w-6xl">
            <div class="flex flex-wrap justify-between gap-4 mb-4">
                <div>
                    <div class="text-xs font-mono opacity-70 flex items-center gap-2">
                        {#if selectedCard.card_type === 'epic'}
                            <Layers class="w-3 h-3 text-purple-500" />
                            <span class="uppercase">Epic</span>
                        {:else if selectedCard.card_type === 'story'}
                            <FileText class="w-3 h-3 text-success" />
                            <span class="uppercase">Story</span>
                        {:else if selectedCard.card_type === 'bug'}
                            <Bug class="w-3 h-3 text-error" />
                            <span class="uppercase">Bug</span>
                        {:else}
                            <CheckSquare class="w-3 h-3 text-info" />
                            <span class="uppercase">Task</span>
                        {/if}
                        <span>•</span>
                        <span>{selectedCard.card_key}</span>
                        {#if selectedCard.parent_card_key}
                            <span class="opacity-50">/</span>
                            <button class="link link-hover opacity-70" on:click={() => {
                                const parent = cards.find(c => c.id === selectedCard?.parent_id);
                                if (parent) openCardDetails(parent);
                            }}>
                                {selectedCard.parent_card_key}
                            </button>
                        {/if}
                    </div>
                    <h3 class="font-bold text-2xl mt-1">{selectedCard.title}</h3>
                </div>
                <div class="flex flex-wrap items-center gap-3">
                    <div class="flex flex-col items-end">
                        <span class="text-xs opacity-50 uppercase">Priority</span>
                        <span class="badge badge-sm badge-outline uppercase">{selectedCard.priority}</span>
                    </div>
                    <div class="flex flex-col items-end">
                        <span class="text-xs opacity-50 uppercase">Assignee</span>
                        <span class="text-sm font-medium">{selectedCard.assignee_name || 'Unassigned'}</span>
                    </div>
                    {#if selectedCard.sprint_id}
                        <div class="flex flex-col items-end">
                            <span class="text-xs opacity-50 uppercase">Sprint</span>
                            <span class="badge badge-sm badge-primary">{getSprintName(selectedCard.sprint_id)}</span>
                        </div>
                    {/if}
                    <div class="flex flex-col items-end">
                        <span class="text-xs opacity-50 uppercase">Due Date</span>
                        <span class="text-sm">{formatDate(selectedCard.due_date)}</span>
                    </div>
                </div>
            </div>

            <div class="tabs tabs-bordered mb-4">
                <button 
                    class="tab tab-md {detailTab === 'details' ? 'tab-active' : ''}" 
                    on:click={() => detailTab = 'details'}>
                    Details
                </button>
                <button 
                    class="tab tab-md {detailTab === 'links' ? 'tab-active' : ''}" 
                    on:click={() => detailTab = 'links'}>
                    Links ({cardLinks.length})
                </button>
                <button 
                    class="tab tab-md {detailTab === 'comments' ? 'tab-active' : ''}" 
                    on:click={() => detailTab = 'comments'}>
                    Comments ({cardComments.length})
                </button>
                <button 
                    class="tab tab-md {detailTab === 'attachments' ? 'tab-active' : ''}" 
                    on:click={() => detailTab = 'attachments'}>
                    Files ({cardAttachments.length})
                </button>
                <button 
                    class="tab tab-md {detailTab === 'history' ? 'tab-active' : ''}" 
                    on:click={() => detailTab = 'history'}>
                    Activity
                </button>
            </div>

            <div class="min-h-[400px]">
                {#if detailsLoading}
                    <div class="flex justify-center p-8">
                        <span class="loading loading-spinner loading-lg"></span>
                    </div>
                {:else}
                    {#if detailTab === 'details'}
                        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                            <div class="lg:col-span-2 space-y-4">
                                {#if selectedCard.description}
                                    <div class="prose prose-sm max-w-none bg-base-200/50 rounded-lg p-4">
                                        {@html selectedCard.description}
                                    </div>
                                {:else}
                                    <div class="italic opacity-50 p-4">No description provided.</div>
                                {/if}
                            </div>
                            <div class="space-y-4">
                                <div class="card bg-base-200 border border-base-300">
                                    <div class="card-body p-4">
                                        <h4 class="text-xs font-bold uppercase opacity-50 mb-2">Hierarchy</h4>
                                        <div class="space-y-2 text-sm">
                                            <div class="flex justify-between">
                                                <span class="opacity-70">Type</span>
                                                <span class="font-medium capitalize">{selectedCard.card_type}</span>
                                            </div>
                                            {#if selectedCard.parent_card_key}
                                                <div class="flex justify-between">
                                                    <span class="opacity-70">Parent</span>
                                                    <button class="link link-primary" on:click={() => {
                                                        const parent = cards.find(c => c.id === selectedCard?.parent_id);
                                                        if (parent) openCardDetails(parent);
                                                    }}>
                                                        {selectedCard.parent_card_key}
                                                    </button>
                                                </div>
                                            {/if}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    {:else if detailTab === 'links'}
                        <div class="space-y-6">
                            <div class="flex items-center justify-between">
                                <h4 class="font-bold">Relationships</h4>
                            </div>

                            {#if cardLinks.length === 0}
                                <div class="text-center py-8 bg-base-200/50 rounded-lg border-2 border-dashed border-base-300 italic opacity-50">
                                    No links or dependencies.
                                </div>
                            {:else}
                                <div class="grid gap-3">
                                    {#each cardLinks as link}
                                        {@const otherCard = link.source_card_id === selectedCard.id ? {id: link.target_card_id, key: link.target_card_key, title: link.target_title} : {id: link.source_card_id, key: link.source_card_key, title: link.source_title}}
                                        <div class="flex items-center justify-between p-3 bg-base-200 rounded-lg border border-base-300 group">
                                            <div class="flex items-center gap-3">
                                                <div class="badge badge-ghost badge-sm uppercase">{link.link_type.replace('_', ' ')}</div>
                                                <div class="flex flex-col">
                                                    <button class="link link-hover font-medium text-left" on:click={() => {
                                                        const card = cards.find(c => c.id === otherCard.id);
                                                        if (card) openCardDetails(card);
                                                    }}>
                                                        {otherCard.key}: {otherCard.title}
                                                    </button>
                                                </div>
                                            </div>
                                            <button 
                                                class="btn btn-ghost btn-xs text-error opacity-0 group-hover:opacity-100 transition-opacity"
                                                on:click={() => removeCardLink(link.id)}>
                                                <Trash2 class="w-3 h-3" />
                                            </button>
                                        </div>
                                    {/each}
                                </div>
                            {/if}

                            <div class="bg-base-200 p-4 rounded-lg">
                                <h4 class="text-sm font-bold mb-3">Add Relationship</h4>
                                <div class="flex flex-wrap gap-2">
                                    <select id="link-target" class="select select-sm select-bordered flex-1 min-w-[200px]">
                                        <option value="">Select card to link...</option>
                                        {#each cards.filter(c => c.id !== selectedCard?.id) as c}
                                            <option value={c.id}>{c.card_key}: {c.title}</option>
                                        {/each}
                                    </select>
                                    <select id="link-type" class="select select-sm select-bordered">
                                        <option value="depends_on">Blocks / Is Blocked By</option>
                                        <option value="relates_to">Relates To</option>
                                    </select>
                                    <button 
                                        class="btn btn-sm btn-primary"
                                        on:click={() => {
                                            const targetId = (document.getElementById('link-target') as HTMLSelectElement).value;
                                            const type = (document.getElementById('link-type') as HTMLSelectElement).value;
                                            if (targetId) addCardLink(targetId, type);
                                        }}>
                                        Add Link
                                    </button>
                                </div>
                            </div>
                        </div>
                    {:else if detailTab === 'comments'}
                        <div class="space-y-4">
                            {#if cardComments.length === 0}
                                <div class="text-sm opacity-50 italic">No comments yet.</div>
                            {:else}
                                <div class="space-y-3">
                                    {#each cardComments as comment}
                                        <div class="bg-base-200/50 rounded-lg p-3">
                                            <div class="flex justify-between items-center mb-1">
                                                <span class="font-bold text-xs">{comment.user_name}</span>
                                                <span class="text-[10px] opacity-50">{formatDate(comment.created_at)}</span>
                                            </div>
                                            <div class="text-sm whitespace-pre-wrap">{comment.comment}</div>
                                        </div>
                                    {/each}
                                </div>
                            {/if}
                            
                            <div class="divider"></div>
                            
                            <div class="space-y-2">
                                <textarea
                                    class="textarea textarea-bordered w-full"
                                    rows="3"
                                    placeholder="Write a comment..."
                                    bind:value={newComment}></textarea>
                                <div class="flex justify-end">
                                    <button
                                        class="btn btn-sm btn-primary"
                                        on:click={addComment}
                                        disabled={!newComment.trim() || !canUpdateCards}>
                                        Post Comment
                                    </button>
                                </div>
                            </div>
                        </div>
                    {:else if detailTab === 'attachments'}
                        <div class="space-y-4">
                            <div class="flex items-center gap-2">
                                <input
                                    class="file-input file-input-bordered file-input-sm w-full"
                                    type="file"
                                    multiple
                                    bind:this={uploadInput}
                                    on:change={onAttachmentSelected} />
                                <button
                                    class="btn btn-sm btn-primary"
                                    on:click={uploadAttachment}
                                    disabled={selectedAttachmentFiles.length === 0 || attachmentUploading || !canUpdateCards}>
                                    <Upload class="w-3 h-3 mr-1" /> Upload
                                </button>
                            </div>

                            {#if cardAttachments.length === 0}
                                <div class="text-sm opacity-50 italic">No attachments yet.</div>
                            {:else}
                                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                                    {#each cardAttachments as attachment}
                                        <div class="bg-base-200 rounded-lg p-3 border border-base-300 flex items-center gap-3">
                                            {#if attachment.content_type.startsWith('image/')}
                                                <div class="w-12 h-12 bg-base-300 rounded overflow-hidden flex-shrink-0">
                                                    {#if attachmentPreviews[attachment.id]}
                                                        <img src={attachmentPreviews[attachment.id]} alt="Preview" class="w-full h-full object-cover" />
                                                    {:else}
                                                        <div class="flex items-center justify-center h-full">
                                                            <span class="loading loading-spinner loading-xs"></span>
                                                        </div>
                                                    {/if}
                                                </div>
                                            {:else}
                                                <div class="w-12 h-12 bg-base-300 rounded flex items-center justify-center flex-shrink-0">
                                                    <Paperclip class="w-6 h-6 opacity-30" />
                                                </div>
                                            {/if}
                                            <div class="flex-1 min-w-0">
                                                <div class="text-xs font-medium truncate" title={attachment.file_name}>{attachment.file_name}</div>
                                                <div class="text-[10px] opacity-50">{formatBytes(attachment.file_size)}</div>
                                            </div>
                                            <div class="flex gap-1">
                                                <button class="btn btn-ghost btn-xs" on:click={() => downloadAttachment(attachment)}>
                                                    <Download class="w-4 h-4" />
                                                </button>
                                            </div>
                                        </div>
                                    {/each}
                                </div>
                            {/if}
                        </div>
                    {:else if detailTab === 'history'}
                        <div class="space-y-4">
                            {#if cardHistory.length === 0}
                                <div class="text-sm opacity-50 italic">No history yet.</div>
                            {:else}
                                <div class="relative pl-6 space-y-6 before:absolute before:left-[11px] before:top-2 before:bottom-2 before:w-0.5 before:bg-base-300">
                                    {#each cardHistory as activity}
                                        <div class="relative">
                                            <div class="absolute -left-[19px] top-1 w-3 h-3 rounded-full bg-base-300 border-2 border-base-100"></div>
                                            <div class="text-xs">
                                                <span class="font-bold">{activity.actor_name || 'System'}</span>
                                                <span class="opacity-50 mx-1">•</span>
                                                <span class="opacity-50">{formatDate(activity.created_at)}</span>
                                            </div>
                                            <div class="text-sm mt-1">{activity.description}</div>
                                        </div>
                                    {/each}
                                </div>
                            {/if}
                        </div>
                    {/if}
                {/if}
            </div>

            <div class="text-sm text-error mt-4">{detailsError}</div>

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
