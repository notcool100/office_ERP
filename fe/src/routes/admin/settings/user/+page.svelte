<script lang="ts">
    import { onMount } from 'svelte';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        Settings,
        User as UserIcon,
        Plus,
        Loader2,
    } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import { userService } from '$lib/services/user-service';
    import { personService } from '$lib/services/person';
    import type { User, CreateUserRequest } from '$lib/types/user';
    import type { Person } from '$lib/types/person';
    import { userStore } from '$lib/stores/user';

    pageTitle.set({
        title: 'User Management',
        desc: 'Manage system users and access.',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'User', icon: UserIcon },
    ]);

    let users: User[] = [];
    let persons: Person[] = [];
    let loading = true;
    let showModal = false;
    let isSubmitting = false;
    let errorMessage = '';

    let formData: CreateUserRequest = {
        personId: '',
        userName: '',
        password: '',
        email: '',
        phone: '',
        isAdmin: false,
    };

    async function loadData() {
        loading = true;
        try {
            const [usersData, personsData] = await Promise.all([
                userService.getAll(),
                personService.getAll({ pageSize: 100 }),
            ]);
            users = usersData;
            persons = personsData.persons;
        } catch (e) {
            console.error('Failed to load data', e);
        } finally {
            loading = false;
        }
    }

    // Auto-fill email/phone when person is selected
    function onPersonSelect() {
        const selectedPerson = persons.find((p) => p.id === formData.personId);
        if (selectedPerson) {
            formData.email = selectedPerson.email || '';
            formData.phone = selectedPerson.phone || '';
            // Generate a default username if empty
            if (!formData.userName) {
                formData.userName = (
                    selectedPerson.first_name + (selectedPerson.last_name || '')
                )
                    .toLowerCase()
                    .replace(/\s/g, '');
            }
        }
    }

    async function handleSubmit() {
        isSubmitting = true;
        errorMessage = '';
        try {
            await userService.create(formData);
            showModal = false;
            resetForm();
            await loadData();
        } catch (e: any) {
            errorMessage =
                e.message ||
                'Failed to create user. Ensure username is unique.';
        } finally {
            isSubmitting = false;
        }
    }

    function resetForm() {
        formData = {
            personId: '',
            userName: '',
            password: '',
            email: '',
            phone: '',
            isAdmin: false,
        };
    }

    onMount(loadData);
</script>

<PageSection title="Users">
    <div class="mb-6 flex justify-between items-center">
        <div class="text-sm breadcrumbs">
            <!-- Breadcrumbs managed by store -->
        </div>
        <button
            class="btn btn-primary btn-sm gap-2"
            on:click={() => (showModal = true)}>
            <Plus size={16} />
            Add User
        </button>
    </div>

    {#if loading}
        <div class="flex justify-center p-8">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    {:else}
        <div class="overflow-x-auto bg-base-100 rounded-lg shadow">
            <table class="table w-full">
                <thead>
                    <tr>
                        <th>Username</th>
                        <th>Email</th>
                        <th>Phone</th>
                        <th>Role</th>
                        <th>Created At</th>
                        <th class="text-right">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {#each users as user}
                        <tr class="hover">
                            <td class="font-bold">{user.userName}</td>
                            <td>{user.email}</td>
                            <td>{user.phone}</td>
                            <td>
                                {#if user.isAdmin}
                                    <span class="badge badge-primary"
                                        >Admin</span>
                                {:else}
                                    <span class="badge badge-ghost">User</span>
                                {/if}
                            </td>
                            <td class="text-sm opacity-70"
                                >{new Date(
                                    user.createdAt,
                                ).toLocaleDateString()}</td>
                            <td class="text-right">
                                <a
                                    href="/admin/settings/user/{user.id}"
                                    class="btn btn-ghost btn-sm gap-2">
                                    <UserIcon size={16} />
                                    View
                                </a>
                            </td>
                        </tr>
                    {:else}
                        <tr>
                            <td colspan="6" class="text-center py-8 opacity-50"
                                >No users found</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {/if}
</PageSection>

{#if showModal}
    <dialog class="modal modal-open">
        <div class="modal-box w-full max-w-md">
            <h3 class="font-bold text-lg mb-4">Create New User</h3>

            {#if errorMessage}
                <div class="alert alert-error text-sm mb-4">
                    <span>{errorMessage}</span>
                </div>
            {/if}

            <form on:submit|preventDefault={handleSubmit} class="space-y-4">
                <div class="form-control">
                    <label class="label" for="personId">
                        <span class="label-text">Select Person</span>
                    </label>
                    <select
                        id="personId"
                        class="select select-bordered w-full"
                        bind:value={formData.personId}
                        on:change={onPersonSelect}
                        required>
                        <option value="" disabled selected
                            >Select a person...</option>
                        {#each persons as person}
                            <option value={person.id}>
                                {person.first_name}
                                {person.middle_name
                                    ? person.middle_name + ' '
                                    : ''}{person.last_name} ({person.email ||
                                    'No Email'})
                            </option>
                        {/each}
                    </select>
                    <label class="label">
                        <span class="label-text-alt text-warning"
                            >This links the login to an employee/person record.</span>
                    </label>
                </div>

                <div class="form-control">
                    <label class="label" for="userName">
                        <span class="label-text">Username</span>
                    </label>
                    <input
                        id="userName"
                        type="text"
                        class="input input-bordered w-full"
                        bind:value={formData.userName}
                        required />
                </div>

                <div class="form-control">
                    <label class="label" for="password">
                        <span class="label-text">Password</span>
                    </label>
                    <input
                        id="password"
                        type="password"
                        class="input input-bordered w-full"
                        bind:value={formData.password}
                        required
                        minlength="6" />
                </div>

                <div class="form-control">
                    <label class="label" for="email">
                        <span class="label-text">Email</span>
                    </label>
                    <input
                        id="email"
                        type="email"
                        class="input input-bordered w-full"
                        bind:value={formData.email}
                        required />
                </div>

                <div class="form-control">
                    <label class="label" for="phone">
                        <span class="label-text">Phone</span>
                    </label>
                    <input
                        id="phone"
                        type="tel"
                        class="input input-bordered w-full"
                        bind:value={formData.phone}
                        required />
                </div>

                <div class="form-control">
                    <label class="label cursor-pointer justify-start gap-4">
                        <span class="label-text">Is Admin User?</span>
                        <input
                            type="checkbox"
                            class="toggle toggle-primary"
                            bind:checked={formData.isAdmin} />
                    </label>
                </div>

                <div class="modal-action">
                    <button
                        type="button"
                        class="btn btn-ghost"
                        on:click={() => (showModal = false)}
                        disabled={isSubmitting}>Cancel</button>
                    <button
                        type="submit"
                        class="btn btn-primary"
                        disabled={isSubmitting}>
                        {#if isSubmitting}
                            <Loader2 class="animate-spin mr-2" size={16} />
                            Creating...
                        {:else}
                            Create User
                        {/if}
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
    </dialog>
{/if}
