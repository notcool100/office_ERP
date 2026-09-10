<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import {
        Folder,
        FolderPlus,
        Upload,
        FileText,
        Download,
        Trash2,
        Pencil,
        ChevronRight,
        Home,
        X,
    } from 'lucide-svelte';
    import {
        documentsService,
        formatFileSize,
        type DocCategory,
        type DocumentMeta,
        type FolderListing,
        type FolderMeta,
    } from '$lib/services/documents';
    import { navigationStore, canCreate, canUpdate, canDelete } from '$lib/stores/navigation';

    export let category: DocCategory;
    export let ownerId: string | undefined = undefined;
    export let navPath: string;
    export let rootLabel: string = 'Documents';

    let currentFolderId: string | undefined = undefined;
    let listing: FolderListing = { folders: [], documents: [], breadcrumb: [] };
    let loading = false;
    let errorMessage = '';

    let showNewFolderModal = false;
    let newFolderName = '';
    let creatingFolder = false;

    let showUploadModal = false;
    let uploadFile: File | null = null;
    let uploadForm = { title: '', docType: '', description: '', expiryDate: '' };
    let uploading = false;

    let showEditModal = false;
    let editingDoc: DocumentMeta | null = null;
    let editForm = { title: '', docType: '', description: '', expiryDate: '' };
    let saving = false;

    let showPreview = false;
    let previewDoc: DocumentMeta | null = null;
    let previewUrl = '';
    let previewLoading = false;

    const blobUrls: Record<string, string> = {};

    $: canCreateHere = canCreate(navPath, $navigationStore);
    $: canUpdateHere = canUpdate(navPath, $navigationStore);
    $: canDeleteHere = canDelete(navPath, $navigationStore);

    async function load() {
        loading = true;
        errorMessage = '';
        try {
            listing = await documentsService.list(category, ownerId, currentFolderId);
        } catch (err) {
            console.error('Failed to load documents:', err);
            errorMessage = 'Failed to load documents';
        } finally {
            loading = false;
        }
    }

    function enterFolder(folder: FolderMeta) {
        currentFolderId = folder.id;
        load();
    }

    function goToRoot() {
        currentFolderId = undefined;
        load();
    }

    function goToBreadcrumb(folder: FolderMeta) {
        currentFolderId = folder.id;
        load();
    }

    function openNewFolderModal() {
        newFolderName = '';
        showNewFolderModal = true;
    }

    async function submitNewFolder() {
        if (!newFolderName.trim()) return;
        creatingFolder = true;
        try {
            await documentsService.createFolder(category, ownerId, currentFolderId, newFolderName);
            showNewFolderModal = false;
            await load();
        } catch (err: any) {
            alert(err.message || 'Failed to create folder');
        } finally {
            creatingFolder = false;
        }
    }

    async function handleDeleteFolder(folder: FolderMeta) {
        if (!confirm(`Delete folder "${folder.name}" and everything inside it?`)) return;
        try {
            await documentsService.deleteFolder(category, ownerId, folder.id);
            await load();
        } catch (err) {
            console.error('Failed to delete folder:', err);
            alert('Failed to delete folder');
        }
    }

    function openUploadModal() {
        uploadFile = null;
        uploadForm = { title: '', docType: '', description: '', expiryDate: '' };
        showUploadModal = true;
    }

    function handleFileSelect(e: Event) {
        const input = e.target as HTMLInputElement;
        uploadFile = input.files?.[0] ?? null;
        if (uploadFile && !uploadForm.title) {
            uploadForm.title = uploadFile.name;
        }
    }

    async function submitUpload() {
        if (!uploadFile) {
            alert('Please choose a file');
            return;
        }
        uploading = true;
        try {
            const formData = new FormData();
            formData.append('file', uploadFile);
            if (ownerId) formData.append('ownerId', ownerId);
            if (currentFolderId) formData.append('folderId', currentFolderId);
            formData.append('title', uploadForm.title);
            if (uploadForm.docType) formData.append('docType', uploadForm.docType);
            if (uploadForm.description) formData.append('description', uploadForm.description);
            if (uploadForm.expiryDate) formData.append('expiryDate', uploadForm.expiryDate);

            await documentsService.upload(category, formData);
            showUploadModal = false;
            await load();
        } catch (err: any) {
            alert(err.message || 'Upload failed');
        } finally {
            uploading = false;
        }
    }

    function openEditModal(doc: DocumentMeta) {
        editingDoc = doc;
        editForm = {
            title: doc.title,
            docType: doc.docType || '',
            description: doc.description || '',
            expiryDate: doc.expiryDate || '',
        };
        showEditModal = true;
    }

    async function submitEdit() {
        if (!editingDoc) return;
        saving = true;
        try {
            await documentsService.update(category, ownerId, editingDoc.id, {
                title: editForm.title,
                docType: editForm.docType || undefined,
                description: editForm.description || undefined,
                expiryDate: editForm.expiryDate || undefined,
            });
            showEditModal = false;
            await load();
        } catch (err) {
            console.error('Failed to update document:', err);
            alert('Failed to update document');
        } finally {
            saving = false;
        }
    }

    async function handleDeleteDocument(doc: DocumentMeta) {
        if (!confirm(`Delete "${doc.title}"?`)) return;
        try {
            await documentsService.delete(category, ownerId, doc.id);
            if (blobUrls[doc.id]) {
                URL.revokeObjectURL(blobUrls[doc.id]);
                delete blobUrls[doc.id];
            }
            await load();
        } catch (err) {
            console.error('Failed to delete document:', err);
            alert('Failed to delete document');
        }
    }

    async function openPreview(doc: DocumentMeta) {
        previewDoc = doc;
        previewUrl = '';
        previewLoading = true;
        showPreview = true;
        try {
            previewUrl = blobUrls[doc.id] ?? (await documentsService.fetchBlob(category, ownerId, doc.id));
            blobUrls[doc.id] = previewUrl;
        } catch (err) {
            console.error('Failed to load file:', err);
        } finally {
            previewLoading = false;
        }
    }

    async function downloadDocument(doc: DocumentMeta) {
        try {
            const url = blobUrls[doc.id] ?? (await documentsService.fetchBlob(category, ownerId, doc.id));
            blobUrls[doc.id] = url;
            const link = document.createElement('a');
            link.href = url;
            link.download = doc.fileName;
            link.click();
        } catch (err) {
            console.error('Download failed:', err);
            alert('Download failed');
        }
    }

    onMount(load);

    onDestroy(() => {
        for (const url of Object.values(blobUrls)) URL.revokeObjectURL(url);
    });
</script>

<div>
    <div class="flex flex-wrap justify-between items-center gap-2 mb-4">
        <div class="flex items-center gap-1 text-sm breadcrumbs-nav flex-wrap">
            <button class="btn btn-ghost btn-xs gap-1" on:click={goToRoot}>
                <Home size={14} />
                {rootLabel}
            </button>
            {#each listing.breadcrumb as crumb}
                <ChevronRight size={14} class="opacity-50" />
                <button class="btn btn-ghost btn-xs" on:click={() => goToBreadcrumb(crumb)}>
                    {crumb.name}
                </button>
            {/each}
        </div>

        <div class="flex gap-2">
            <button
                class="btn btn-sm btn-outline"
                disabled={!canCreateHere}
                title={!canCreateHere ? 'No permission to create' : ''}
                on:click={openNewFolderModal}>
                <FolderPlus size={16} />
                New Folder
            </button>
            <button
                class="btn btn-sm btn-primary"
                disabled={!canCreateHere}
                title={!canCreateHere ? 'No permission to upload' : ''}
                on:click={openUploadModal}>
                <Upload size={16} />
                Upload
            </button>
        </div>
    </div>

    {#if loading}
        <div class="flex justify-center py-8">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    {:else if errorMessage}
        <div class="alert alert-error">{errorMessage}</div>
    {:else if listing.folders.length === 0 && listing.documents.length === 0}
        <div class="text-center py-12 opacity-60">This folder is empty</div>
    {:else}
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3">
            {#each listing.folders as folder}
                <div class="card bg-base-200 hover:bg-base-300 transition-colors group relative">
                    <button
                        class="card-body items-center text-center p-4 gap-1 w-full"
                        on:click={() => enterFolder(folder)}>
                        <Folder size={36} class="text-primary" />
                        <span class="text-sm font-medium truncate w-full" title={folder.name}>
                            {folder.name}
                        </span>
                    </button>
                    {#if canDeleteHere}
                        <button
                            class="btn btn-ghost btn-xs absolute top-1 right-1 opacity-0 group-hover:opacity-100 text-error"
                            title="Delete folder"
                            on:click={() => handleDeleteFolder(folder)}>
                            <Trash2 size={14} />
                        </button>
                    {/if}
                </div>
            {/each}

            {#each listing.documents as doc}
                <div class="card bg-base-200 hover:bg-base-300 transition-colors group relative">
                    <button
                        class="card-body items-center text-center p-4 gap-1 w-full"
                        on:click={() => openPreview(doc)}>
                        <FileText size={36} class="text-secondary" />
                        <span class="text-sm font-medium truncate w-full" title={doc.title}>
                            {doc.title}
                        </span>
                        <span class="text-xs opacity-60">{formatFileSize(doc.fileSize)}</span>
                        {#if doc.expiryDate}
                            <span class="badge badge-xs badge-warning">exp {doc.expiryDate}</span>
                        {/if}
                    </button>
                    <div
                        class="absolute top-1 right-1 flex gap-1 opacity-0 group-hover:opacity-100">
                        <button
                            class="btn btn-ghost btn-xs"
                            title="Download"
                            on:click={() => downloadDocument(doc)}>
                            <Download size={14} />
                        </button>
                        {#if canUpdateHere}
                            <button
                                class="btn btn-ghost btn-xs"
                                title="Edit details"
                                on:click={() => openEditModal(doc)}>
                                <Pencil size={14} />
                            </button>
                        {/if}
                        {#if canDeleteHere}
                            <button
                                class="btn btn-ghost btn-xs text-error"
                                title="Delete"
                                on:click={() => handleDeleteDocument(doc)}>
                                <Trash2 size={14} />
                            </button>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<!-- New folder modal -->
{#if showNewFolderModal}
    <div class="modal modal-open">
        <div class="modal-box max-w-sm">
            <h3 class="font-bold text-lg mb-4">New Folder</h3>
            <form on:submit|preventDefault={submitNewFolder} class="space-y-4">
                <input
                    type="text"
                    class="input input-bordered w-full"
                    placeholder="Folder name"
                    bind:value={newFolderName}
                    required />
                <div class="modal-action">
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showNewFolderModal = false)}>Cancel</button>
                    <button type="submit" class="btn btn-primary" disabled={creatingFolder}>
                        {creatingFolder ? 'Creating...' : 'Create'}
                    </button>
                </div>
            </form>
        </div>
        <button
            class="modal-backdrop"
            aria-label="Close"
            on:click={() => (showNewFolderModal = false)}></button>
    </div>
{/if}

<!-- Upload modal -->
{#if showUploadModal}
    <div class="modal modal-open">
        <div class="modal-box max-w-md">
            <h3 class="font-bold text-lg mb-4">Upload Document</h3>
            <form on:submit|preventDefault={submitUpload} class="space-y-4">
                <div class="form-control">
                    <label class="label" for="doc-file"><span class="label-text">File</span></label>
                    <input
                        id="doc-file"
                        type="file"
                        class="file-input file-input-bordered w-full"
                        on:change={handleFileSelect}
                        required />
                </div>
                <div class="form-control">
                    <label class="label" for="doc-title"><span class="label-text">Title</span></label>
                    <input
                        id="doc-title"
                        type="text"
                        class="input input-bordered w-full"
                        bind:value={uploadForm.title}
                        required />
                </div>
                <div class="form-control">
                    <label class="label" for="doc-type"><span class="label-text">Type (optional, e.g. PAN, Contract)</span></label>
                    <input
                        id="doc-type"
                        type="text"
                        class="input input-bordered w-full"
                        bind:value={uploadForm.docType} />
                </div>
                <div class="form-control">
                    <label class="label" for="doc-expiry"><span class="label-text">Expiry date (optional)</span></label>
                    <input
                        id="doc-expiry"
                        type="date"
                        class="input input-bordered w-full"
                        bind:value={uploadForm.expiryDate} />
                </div>
                <div class="form-control">
                    <label class="label" for="doc-desc"><span class="label-text">Description (optional)</span></label>
                    <textarea
                        id="doc-desc"
                        class="textarea textarea-bordered w-full"
                        bind:value={uploadForm.description}></textarea>
                </div>
                <div class="modal-action">
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showUploadModal = false)}>Cancel</button>
                    <button type="submit" class="btn btn-primary" disabled={uploading}>
                        {uploading ? 'Uploading...' : 'Upload'}
                    </button>
                </div>
            </form>
        </div>
        <button
            class="modal-backdrop"
            aria-label="Close"
            on:click={() => (showUploadModal = false)}></button>
    </div>
{/if}

<!-- Edit modal -->
{#if showEditModal && editingDoc}
    <div class="modal modal-open">
        <div class="modal-box max-w-md">
            <h3 class="font-bold text-lg mb-4">Edit Document</h3>
            <form on:submit|preventDefault={submitEdit} class="space-y-4">
                <div class="form-control">
                    <label class="label" for="edit-title"><span class="label-text">Title</span></label>
                    <input
                        id="edit-title"
                        type="text"
                        class="input input-bordered w-full"
                        bind:value={editForm.title}
                        required />
                </div>
                <div class="form-control">
                    <label class="label" for="edit-type"><span class="label-text">Type</span></label>
                    <input
                        id="edit-type"
                        type="text"
                        class="input input-bordered w-full"
                        bind:value={editForm.docType} />
                </div>
                <div class="form-control">
                    <label class="label" for="edit-expiry"><span class="label-text">Expiry date</span></label>
                    <input
                        id="edit-expiry"
                        type="date"
                        class="input input-bordered w-full"
                        bind:value={editForm.expiryDate} />
                </div>
                <div class="form-control">
                    <label class="label" for="edit-desc"><span class="label-text">Description</span></label>
                    <textarea
                        id="edit-desc"
                        class="textarea textarea-bordered w-full"
                        bind:value={editForm.description}></textarea>
                </div>
                <div class="modal-action">
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showEditModal = false)}>Cancel</button>
                    <button type="submit" class="btn btn-primary" disabled={saving}>
                        {saving ? 'Saving...' : 'Save'}
                    </button>
                </div>
            </form>
        </div>
        <button
            class="modal-backdrop"
            aria-label="Close"
            on:click={() => (showEditModal = false)}></button>
    </div>
{/if}

<!-- Preview modal -->
{#if showPreview && previewDoc}
    <div class="modal modal-open">
        <div class="modal-box max-w-2xl">
            <div class="flex justify-between items-start mb-4">
                <h3 class="font-bold text-lg truncate">{previewDoc.title}</h3>
                <button class="btn btn-ghost btn-sm btn-circle" on:click={() => (showPreview = false)}>
                    <X size={18} />
                </button>
            </div>

            {#if previewLoading}
                <div class="flex justify-center py-8">
                    <span class="loading loading-spinner loading-lg"></span>
                </div>
            {:else if previewUrl && previewDoc.isImage}
                <img src={previewUrl} alt={previewDoc.title} class="w-full max-h-[60vh] object-contain rounded-lg" />
            {:else}
                <div class="flex flex-col items-center py-8 gap-2 opacity-70">
                    <FileText size={48} />
                    <p>{previewDoc.fileName}</p>
                    <p class="text-sm">{previewDoc.contentType}</p>
                </div>
            {/if}

            <div class="grid grid-cols-2 gap-2 text-sm mt-4">
                <span>Size: {formatFileSize(previewDoc.fileSize)}</span>
                {#if previewDoc.docType}<span>Type: {previewDoc.docType}</span>{/if}
                {#if previewDoc.expiryDate}<span>Expires: {previewDoc.expiryDate}</span>{/if}
                {#if previewDoc.uploadedByName}<span>Uploaded by: {previewDoc.uploadedByName}</span>{/if}
                {#if previewDoc.description}
                    <p class="col-span-2">{previewDoc.description}</p>
                {/if}
            </div>

            <div class="modal-action">
                <button class="btn btn-primary" on:click={() => previewDoc && downloadDocument(previewDoc)}>
                    <Download size={16} />
                    Download
                </button>
            </div>
        </div>
        <button class="modal-backdrop" aria-label="Close preview" on:click={() => (showPreview = false)}></button>
    </div>
{/if}
