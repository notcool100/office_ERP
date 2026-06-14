<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Megaphone, FileImage, Upload, Pencil, Trash2, Download, Search, X, Eye } from 'lucide-svelte';
    import { onMount, onDestroy } from 'svelte';
    import { navigationStore, canCreate, canUpdate, canDelete } from '$lib/stores/navigation';
    import {
        mediaLibraryService,
        CATEGORIES, CATEGORY_COLORS, formatFileSize,
    } from '$lib/services/media-library';
    import type { MediaAsset, AssetCategory } from '$lib/services/media-library';

    pageTitle.set({ title: 'Media Library', desc: 'Centralized storage for logos, creatives, and brand assets' });
    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Digital Marketing', icon: Megaphone },
        { label: 'Media Library', icon: FileImage },
    ]);

    const navPath = '/admin/digital-marketing/media-library';

    // ── State ────────────────────────────────────────────────────────────
    let assets: MediaAsset[] = [];
    let loading = true;
    let error = '';
    let filterCategory = '';
    let searchTerm = '';
    let searchInput = '';
    let searchTimeout: ReturnType<typeof setTimeout>;

    // Blob URL cache for images
    let blobUrls: Record<string, string> = {};

    // Upload modal
    let showUpload = false;
    let uploading = false;
    let uploadError = '';
    let uploadFile: File | null = null;
    let uploadName = '';
    let uploadDescription = '';
    let uploadCategory: AssetCategory = 'other';
    let uploadTags = '';
    let fileInput: HTMLInputElement;

    // Edit modal
    let showEdit = false;
    let editAsset: MediaAsset | null = null;
    let editName = '';
    let editDescription = '';
    let editCategory: AssetCategory = 'other';
    let editTags = '';
    let saving = false;
    let saveError = '';

    // Preview modal
    let showPreview = false;
    let previewAsset: MediaAsset | null = null;
    let previewUrl = '';
    let previewLoading = false;

    // ── Helpers ──────────────────────────────────────────────────────────
    async function loadAssets() {
        loading = true;
        error = '';
        try {
            assets = await mediaLibraryService.list({
                category: filterCategory || undefined,
                search: searchTerm || undefined,
            });
            // Preload image blobs
            for (const a of assets) {
                if (a.is_image && !blobUrls[a.id]) {
                    loadImageBlob(a.id);
                }
            }
        } catch (e: any) {
            error = e.message ?? 'Failed to load assets';
        } finally {
            loading = false;
        }
    }

    async function loadImageBlob(id: string) {
        try {
            blobUrls[id] = await mediaLibraryService.fetchBlob(id);
            blobUrls = { ...blobUrls };
        } catch {
            // ignore
        }
    }

    function onSearchInput() {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(() => {
            searchTerm = searchInput;
            loadAssets();
        }, 400);
    }

    function onCategoryChange() {
        loadAssets();
    }

    // ── Upload ───────────────────────────────────────────────────────────
    function openUpload() {
        uploadFile = null;
        uploadName = '';
        uploadDescription = '';
        uploadCategory = 'other';
        uploadTags = '';
        uploadError = '';
        showUpload = true;
    }

    function onFileSelect(e: Event) {
        const target = e.target as HTMLInputElement;
        uploadFile = target.files?.[0] ?? null;
        if (uploadFile && !uploadName) {
            uploadName = uploadFile.name.replace(/\.[^.]+$/, '');
        }
    }

    async function submitUpload() {
        if (!uploadFile) {
            uploadError = 'Please select a file.';
            return;
        }
        uploading = true;
        uploadError = '';
        try {
            const fd = new FormData();
            fd.append('file', uploadFile);
            fd.append('name', uploadName || uploadFile.name);
            if (uploadDescription) fd.append('description', uploadDescription);
            fd.append('category', uploadCategory);
            if (uploadTags.trim()) fd.append('tags', uploadTags);
            const created = await mediaLibraryService.upload(fd);
            assets = [created, ...assets];
            if (created.is_image) loadImageBlob(created.id);
            showUpload = false;
        } catch (e: any) {
            uploadError = e.message ?? 'Upload failed';
        } finally {
            uploading = false;
        }
    }

    // ── Edit ─────────────────────────────────────────────────────────────
    function openEdit(a: MediaAsset) {
        editAsset = a;
        editName = a.name;
        editDescription = a.description ?? '';
        editCategory = a.category;
        editTags = a.tags.join(', ');
        saveError = '';
        showEdit = true;
    }

    async function submitEdit() {
        if (!editAsset) return;
        saving = true;
        saveError = '';
        try {
            const updated = await mediaLibraryService.update(editAsset.id, {
                name: editName || undefined,
                description: editDescription || undefined,
                category: editCategory,
                tags: editTags ? editTags.split(',').map(t => t.trim()).filter(Boolean) : [],
            });
            assets = assets.map(a => a.id === updated.id ? updated : a);
            showEdit = false;
        } catch (e: any) {
            saveError = e.message ?? 'Failed to save';
        } finally {
            saving = false;
        }
    }

    // ── Delete ───────────────────────────────────────────────────────────
    async function deleteAsset(a: MediaAsset) {
        if (!confirm(`Delete "${a.name}"? This cannot be undone.`)) return;
        try {
            await mediaLibraryService.delete(a.id);
            if (blobUrls[a.id]) URL.revokeObjectURL(blobUrls[a.id]);
            assets = assets.filter(x => x.id !== a.id);
        } catch (e: any) {
            alert(e.message ?? 'Failed to delete');
        }
    }

    // ── Preview ──────────────────────────────────────────────────────────
    async function openPreview(a: MediaAsset) {
        previewAsset = a;
        previewUrl = '';
        previewLoading = true;
        showPreview = true;
        try {
            previewUrl = blobUrls[a.id] ?? await mediaLibraryService.fetchBlob(a.id);
            if (!blobUrls[a.id]) {
                blobUrls[a.id] = previewUrl;
                blobUrls = { ...blobUrls };
            }
        } catch {
            previewUrl = '';
        } finally {
            previewLoading = false;
        }
    }

    // ── Download ─────────────────────────────────────────────────────────
    async function downloadAsset(a: MediaAsset) {
        try {
            const url = blobUrls[a.id] ?? await mediaLibraryService.fetchBlob(a.id);
            const link = document.createElement('a');
            link.href = url;
            link.download = a.file_name;
            link.click();
        } catch (e: any) {
            alert('Download failed');
        }
    }

    // ── Lifecycle ────────────────────────────────────────────────────────
    onMount(loadAssets);

    onDestroy(() => {
        clearTimeout(searchTimeout);
        for (const url of Object.values(blobUrls)) URL.revokeObjectURL(url);
    });
</script>

<!-- ── Upload Modal ─────────────────────────────────────────────────────── -->
{#if showUpload}
<div class="modal modal-open">
    <div class="modal-box max-w-lg">
        <h3 class="font-bold text-lg mb-4">Upload Asset</h3>

        <div class="space-y-3">
            <div>
                <label for="ml-file" class="label"><span class="label-text">File <span class="text-error">*</span></span></label>
                <input id="ml-file" type="file" class="file-input file-input-bordered w-full"
                    bind:this={fileInput} on:change={onFileSelect} />
                <p class="text-xs text-base-content/50 mt-1">Max 50 MB</p>
            </div>
            <div>
                <label for="ml-name" class="label"><span class="label-text">Name</span></label>
                <input id="ml-name" type="text" class="input input-bordered w-full" placeholder="Asset name"
                    bind:value={uploadName} />
            </div>
            <div>
                <label for="ml-category" class="label"><span class="label-text">Category</span></label>
                <select id="ml-category" class="select select-bordered w-full" bind:value={uploadCategory}>
                    {#each CATEGORIES.slice(1) as cat}
                        <option value={cat.value}>{cat.label}</option>
                    {/each}
                </select>
            </div>
            <div>
                <label for="ml-tags" class="label"><span class="label-text">Tags <span class="text-xs text-base-content/50">(comma-separated)</span></span></label>
                <input id="ml-tags" type="text" class="input input-bordered w-full" placeholder="e.g. brand, 2024, Q1"
                    bind:value={uploadTags} />
            </div>
            <div>
                <label for="ml-desc" class="label"><span class="label-text">Description</span></label>
                <textarea id="ml-desc" class="textarea textarea-bordered w-full" rows="2" placeholder="Optional description"
                    bind:value={uploadDescription}></textarea>
            </div>
        </div>

        {#if uploadError}
            <div class="alert alert-error mt-3 text-sm">{uploadError}</div>
        {/if}

        <div class="modal-action">
            <button class="btn btn-ghost" on:click={() => showUpload = false} disabled={uploading}>Cancel</button>
            <button class="btn btn-primary" on:click={submitUpload} disabled={uploading || !uploadFile}>
                {#if uploading}<span class="loading loading-spinner loading-xs"></span>{/if}
                Upload
            </button>
        </div>
    </div>
    <button class="modal-backdrop" aria-label="Close modal" on:click={() => showUpload = false}></button>
</div>
{/if}

<!-- ── Edit Modal ────────────────────────────────────────────────────────── -->
{#if showEdit && editAsset}
<div class="modal modal-open">
    <div class="modal-box max-w-lg">
        <h3 class="font-bold text-lg mb-4">Edit Asset</h3>

        <div class="space-y-3">
            <div>
                <label for="ml-edit-name" class="label"><span class="label-text">Name</span></label>
                <input id="ml-edit-name" type="text" class="input input-bordered w-full" bind:value={editName} />
            </div>
            <div>
                <label for="ml-edit-category" class="label"><span class="label-text">Category</span></label>
                <select id="ml-edit-category" class="select select-bordered w-full" bind:value={editCategory}>
                    {#each CATEGORIES.slice(1) as cat}
                        <option value={cat.value}>{cat.label}</option>
                    {/each}
                </select>
            </div>
            <div>
                <label for="ml-edit-tags" class="label"><span class="label-text">Tags <span class="text-xs text-base-content/50">(comma-separated)</span></span></label>
                <input id="ml-edit-tags" type="text" class="input input-bordered w-full" bind:value={editTags} />
            </div>
            <div>
                <label for="ml-edit-desc" class="label"><span class="label-text">Description</span></label>
                <textarea id="ml-edit-desc" class="textarea textarea-bordered w-full" rows="2"
                    bind:value={editDescription}></textarea>
            </div>
        </div>

        {#if saveError}
            <div class="alert alert-error mt-3 text-sm">{saveError}</div>
        {/if}

        <div class="modal-action">
            <button class="btn btn-ghost" on:click={() => showEdit = false} disabled={saving}>Cancel</button>
            <button class="btn btn-primary" on:click={submitEdit} disabled={saving}>
                {#if saving}<span class="loading loading-spinner loading-xs"></span>{/if}
                Save
            </button>
        </div>
    </div>
    <button class="modal-backdrop" aria-label="Close modal" on:click={() => showEdit = false}></button>
</div>
{/if}

<!-- ── Preview Modal ─────────────────────────────────────────────────────── -->
{#if showPreview && previewAsset}
<div class="modal modal-open">
    <div class="modal-box max-w-3xl">
        <div class="flex items-center justify-between mb-4">
            <h3 class="font-bold text-lg truncate">{previewAsset.name}</h3>
            <button class="btn btn-ghost btn-sm btn-circle" on:click={() => showPreview = false}><X size={16} /></button>
        </div>

        {#if previewLoading}
            <div class="flex justify-center py-12"><span class="loading loading-spinner loading-lg"></span></div>
        {:else if previewUrl && previewAsset.is_image}
            <img src={previewUrl} alt={previewAsset.name} class="w-full max-h-[60vh] object-contain rounded-lg" />
        {:else}
            <div class="flex flex-col items-center justify-center py-12 gap-3 text-base-content/50">
                <FileImage size={48} />
                <p>{previewAsset.file_name}</p>
                <p class="text-sm">{previewAsset.content_type}</p>
            </div>
        {/if}

        <div class="mt-4 grid grid-cols-2 gap-2 text-sm text-base-content/70">
            <span>Size: {formatFileSize(previewAsset.file_size)}</span>
            <span>Category: {previewAsset.category}</span>
            {#if previewAsset.tags.length > 0}
                <span class="col-span-2">Tags: {previewAsset.tags.join(', ')}</span>
            {/if}
            {#if previewAsset.description}
                <p class="col-span-2">{previewAsset.description}</p>
            {/if}
        </div>

        <div class="modal-action">
            <button class="btn btn-ghost" on:click={() => showPreview = false}>Close</button>
            <button class="btn btn-primary" on:click={() => previewAsset && downloadAsset(previewAsset)}>
                <Download size={16} /> Download
            </button>
        </div>
    </div>
    <button class="modal-backdrop" aria-label="Close preview" on:click={() => showPreview = false}></button>
</div>
{/if}

<!-- ── Page ──────────────────────────────────────────────────────────────── -->
<div class="p-6 space-y-6">
    <!-- Header -->
    <div class="flex flex-wrap items-center justify-between gap-3">
        <div>
            <h1 class="text-2xl font-bold">Media Library</h1>
            <p class="text-base-content/60 text-sm">Logos, creatives, brand assets, and more</p>
        </div>
        {#if canCreate(navPath, $navigationStore)}
            <button class="btn btn-primary" on:click={openUpload}>
                <Upload size={16} /> Upload Asset
            </button>
        {/if}
    </div>

    <!-- Filters -->
    <div class="flex flex-wrap gap-3">
        <label class="input input-bordered flex items-center gap-2 flex-1 min-w-52">
            <Search size={16} class="text-base-content/50" />
            <input type="text" placeholder="Search assets…" class="grow"
                bind:value={searchInput} on:input={onSearchInput} />
            {#if searchInput}
                <button on:click={() => { searchInput = ''; searchTerm = ''; loadAssets(); }}><X size={14} /></button>
            {/if}
        </label>
        <select class="select select-bordered" bind:value={filterCategory} on:change={onCategoryChange}>
            {#each CATEGORIES as cat}
                <option value={cat.value}>{cat.label}</option>
            {/each}
        </select>
    </div>

    <!-- Content -->
    {#if loading}
        <div class="flex justify-center py-20"><span class="loading loading-spinner loading-lg"></span></div>
    {:else if error}
        <div class="alert alert-error">{error}</div>
    {:else if assets.length === 0}
        <div class="flex flex-col items-center justify-center py-20 gap-4 text-base-content/40">
            <FileImage size={64} />
            <p class="text-lg">No assets found</p>
            {#if canCreate(navPath, $navigationStore)}
                <button class="btn btn-primary btn-sm" on:click={openUpload}>Upload your first asset</button>
            {/if}
        </div>
    {:else}
        <!-- Grid -->
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
            {#each assets as asset (asset.id)}
                <div class="card card-compact bg-base-100 border border-base-300 hover:shadow-md transition-shadow group">
                    <!-- Thumbnail -->
                    <figure class="relative h-32 bg-base-200 rounded-t-xl overflow-hidden">
                        {#if asset.is_image && blobUrls[asset.id]}
                            <img src={blobUrls[asset.id]} alt={asset.name}
                                class="w-full h-full object-cover" />
                        {:else if asset.is_image}
                            <div class="flex items-center justify-center h-full text-base-content/30">
                                <span class="loading loading-spinner loading-sm"></span>
                            </div>
                        {:else}
                            <div class="flex flex-col items-center justify-center h-full text-base-content/30 gap-1">
                                <FileImage size={32} />
                                <span class="text-xs uppercase font-mono">
                                    {asset.file_name.split('.').pop() ?? 'file'}
                                </span>
                            </div>
                        {/if}

                        <!-- Hover overlay -->
                        <div class="absolute inset-0 bg-base-300/80 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center gap-2">
                            {#if asset.is_image}
                                <button class="btn btn-circle btn-xs btn-ghost" title="Preview"
                                    on:click={() => openPreview(asset)}>
                                    <Eye size={14} />
                                </button>
                            {/if}
                            <button class="btn btn-circle btn-xs btn-ghost" title="Download"
                                on:click={() => downloadAsset(asset)}>
                                <Download size={14} />
                            </button>
                            {#if canUpdate(navPath, $navigationStore)}
                                <button class="btn btn-circle btn-xs btn-ghost" title="Edit"
                                    on:click={() => openEdit(asset)}>
                                    <Pencil size={14} />
                                </button>
                            {/if}
                            {#if canDelete(navPath, $navigationStore)}
                                <button class="btn btn-circle btn-xs btn-ghost text-error" title="Delete"
                                    on:click={() => deleteAsset(asset)}>
                                    <Trash2 size={14} />
                                </button>
                            {/if}
                        </div>
                    </figure>

                    <!-- Info -->
                    <div class="card-body p-2">
                        <p class="font-medium text-xs truncate" title={asset.name}>{asset.name}</p>
                        <div class="flex items-center justify-between gap-1">
                            <span class="badge badge-xs {CATEGORY_COLORS[asset.category]} truncate max-w-[70%]">
                                {asset.category}
                            </span>
                            <span class="text-xs text-base-content/50">{formatFileSize(asset.file_size)}</span>
                        </div>
                        {#if asset.tags.length > 0}
                            <div class="flex flex-wrap gap-1 mt-1">
                                {#each asset.tags.slice(0, 2) as tag}
                                    <span class="badge badge-xs badge-outline">{tag}</span>
                                {/each}
                                {#if asset.tags.length > 2}
                                    <span class="badge badge-xs badge-outline">+{asset.tags.length - 2}</span>
                                {/if}
                            </div>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>

        <!-- List view below grid -->
        <div class="overflow-x-auto rounded-box border border-base-300">
            <table class="table table-sm">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Category</th>
                        <th>Type</th>
                        <th>Size</th>
                        <th>Uploaded by</th>
                        <th>Date</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {#each assets as asset (asset.id)}
                        <tr class="hover">
                            <td class="font-medium max-w-xs truncate">{asset.name}</td>
                            <td><span class="badge badge-sm {CATEGORY_COLORS[asset.category]}">{asset.category}</span></td>
                            <td class="text-xs text-base-content/60">{asset.content_type}</td>
                            <td class="text-xs">{formatFileSize(asset.file_size)}</td>
                            <td class="text-xs text-base-content/60">{asset.uploaded_by_name ?? '—'}</td>
                            <td class="text-xs text-base-content/60">
                                {new Date(asset.created_at).toLocaleDateString()}
                            </td>
                            <td>
                                <div class="flex gap-1 justify-end">
                                    {#if asset.is_image}
                                        <button class="btn btn-ghost btn-xs" title="Preview" on:click={() => openPreview(asset)}>
                                            <Eye size={13} />
                                        </button>
                                    {/if}
                                    <button class="btn btn-ghost btn-xs" title="Download" on:click={() => downloadAsset(asset)}>
                                        <Download size={13} />
                                    </button>
                                    {#if canUpdate(navPath, $navigationStore)}
                                        <button class="btn btn-ghost btn-xs" title="Edit" on:click={() => openEdit(asset)}>
                                            <Pencil size={13} />
                                        </button>
                                    {/if}
                                    {#if canDelete(navPath, $navigationStore)}
                                        <button class="btn btn-ghost btn-xs text-error" title="Delete" on:click={() => deleteAsset(asset)}>
                                            <Trash2 size={13} />
                                        </button>
                                    {/if}
                                </div>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {/if}
</div>
