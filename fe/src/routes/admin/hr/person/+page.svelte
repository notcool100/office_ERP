<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        Users,
        User,
        Plus,
        Search,
        Edit,
        Trash2,
    } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import { personService } from '$lib/services/person';
    import type {
        Person,
        CreatePersonRequest,
        UpdatePersonRequest,
    } from '$lib/services/person';
    import { onMount } from 'svelte';

    pageTitle.set({
        title: 'Person Management',
        desc: 'Manage person records',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'HR', icon: Users },
        { label: 'Person', icon: User },
    ]);

    let persons: Person[] = [];
    let total = 0;
    let page = 1;
    let pageSize = 10;
    let searchText = '';
    let loading = false;
    let showModal = false;
    let modalMode: 'create' | 'edit' = 'create';
    let selectedPerson: Person | null = null;
    let error = '';

    let formData = {
        first_name: '',
        middle_name: '',
        last_name: '',
    };

    async function loadPersons() {
        loading = true;
        error = '';
        try {
            const response = await personService.getAll({
                page,
                pageSize,
                search: searchText || undefined,
            });
            persons = response.persons;
            total = response.total;
        } catch (err: any) {
            console.error('Failed to load persons:', err);
            error = 'Failed to load persons: ' + err.message;
        } finally {
            loading = false;
        }
    }

    function openCreateModal() {
        modalMode = 'create';
        selectedPerson = null;
        formData = {
            first_name: '',
            middle_name: '',
            last_name: '',
        };
        showModal = true;
    }

    function openEditModal(person: Person) {
        modalMode = 'edit';
        selectedPerson = person;
        formData = {
            first_name: person.first_name,
            middle_name: person.middle_name || '',
            last_name: person.last_name,
        };
        showModal = true;
    }

    async function handleSubmit() {
        try {
            const payload = {
                first_name: formData.first_name,
                middle_name: formData.middle_name || undefined,
                last_name: formData.last_name,
            };

            if (modalMode === 'create') {
                await personService.create(payload);
            } else if (selectedPerson) {
                await personService.update(selectedPerson.id, payload);
            }
            showModal = false;
            await loadPersons();
        } catch (err: any) {
            console.error('Failed to save person:', err);
            alert('Failed to save person: ' + err.message);
        }
    }

    async function handleDelete(person: Person) {
        if (
            confirm(
                `Are you sure you want to delete ${person.first_name} ${person.last_name}?`,
            )
        ) {
            try {
                await personService.delete(person.id);
                await loadPersons();
            } catch (err: any) {
                console.error('Failed to delete person:', err);
                alert('Failed to delete person: ' + err.message);
            }
        }
    }

    function copyToClipboard(text: string) {
        navigator.clipboard.writeText(text);
        alert('Copied ID: ' + text);
    }

    onMount(() => {
        loadPersons();
    });

    $: if (searchText !== undefined || page) {
        loadPersons();
    }

    $: totalPages = Math.ceil(total / pageSize);
</script>

<PageSection title="Person Management">
    <div class="mb-6 flex flex-wrap gap-4 justify-between items-center">
        <div class="flex gap-2 flex-1 max-w-md">
            <input
                type="text"
                placeholder="Search persons..."
                bind:value={searchText}
                class="input input-bordered flex-1" />
            <button class="btn btn-square" on:click={loadPersons}>
                <Search size={20} />
            </button>
        </div>

        <button class="btn btn-primary" on:click={openCreateModal}>
            <Plus size={20} />
            Add Person
        </button>
    </div>

    {#if error}
        <div class="alert alert-error mb-4">
            <span>{error}</span>
        </div>
    {/if}

    {#if loading}
        <div class="flex justify-center py-8">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    {:else}
        <div class="overflow-x-auto">
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>First Name</th>
                        <th>Middle Name</th>
                        <th>Last Name</th>
                        <th>ID (Click to copy)</th>
                        <th>Created At</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {#each persons as person}
                        <tr>
                            <td>{person.first_name}</td>
                            <td>{person.middle_name || '-'}</td>
                            <td>{person.last_name}</td>
                            <td>
                                <button
                                    class="btn btn-xs btn-ghost text-xs font-mono"
                                    on:click={() => copyToClipboard(person.id)}
                                    title="Click to copy UUID">
                                    {person.id.substring(0, 8)}...
                                </button>
                            </td>
                            <td
                                >{new Date(
                                    person.created_at,
                                ).toLocaleDateString()}</td>
                            <td>
                                <div class="flex gap-2">
                                    <button
                                        class="btn btn-sm btn-ghost"
                                        on:click={() => openEditModal(person)}>
                                        <Edit size={16} />
                                    </button>
                                    <button
                                        class="btn btn-sm btn-ghost text-error"
                                        on:click={() => handleDelete(person)}>
                                        <Trash2 size={16} />
                                    </button>
                                </div>
                            </td>
                        </tr>
                    {:else}
                        <tr>
                            <td colspan="6" class="text-center py-8"
                                >No persons found</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>

        <div class="flex justify-center mt-4 gap-2">
            <button
                class="btn btn-sm"
                disabled={page <= 1}
                on:click={() => (page = 1)}>First</button>
            <button
                class="btn btn-sm"
                disabled={page <= 1}
                on:click={() => page--}>Previous</button>
            <span class="flex items-center px-4"
                >Page {page} of {totalPages || 1}</span>
            <button
                class="btn btn-sm"
                disabled={page >= totalPages}
                on:click={() => page++}>Next</button>
            <button
                class="btn btn-sm"
                disabled={page >= totalPages}
                on:click={() => (page = totalPages)}>Last</button>
        </div>
    {/if}
</PageSection>

{#if showModal}
    <div class="modal modal-open">
        <div class="modal-box">
            <h3 class="font-bold text-lg mb-4">
                {modalMode === 'create' ? 'Add New Person' : 'Edit Person'}
            </h3>

            <form on:submit|preventDefault={handleSubmit} class="space-y-4">
                <div class="form-control">
                    <label class="label" for="firstName">
                        <span class="label-text">First Name</span>
                    </label>
                    <input
                        id="firstName"
                        type="text"
                        class="input input-bordered"
                        bind:value={formData.first_name}
                        required />
                </div>

                <div class="form-control">
                    <label class="label" for="middleName">
                        <span class="label-text">Middle Name</span>
                    </label>
                    <input
                        id="middleName"
                        type="text"
                        class="input input-bordered"
                        bind:value={formData.middle_name} />
                </div>

                <div class="form-control">
                    <label class="label" for="lastName">
                        <span class="label-text">Last Name</span>
                    </label>
                    <input
                        id="lastName"
                        type="text"
                        class="input input-bordered"
                        bind:value={formData.last_name}
                        required />
                </div>

                <div class="modal-action">
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showModal = false)}>Cancel</button>
                    <button type="submit" class="btn btn-primary">
                        {modalMode === 'create' ? 'Create' : 'Update'}
                    </button>
                </div>
            </form>
        </div>
        <form
            method="dialog"
            class="modal-backdrop"
            on:click={() => (showModal = false)}>
            <button>close</button>
        </form>
    </div>
{/if}
