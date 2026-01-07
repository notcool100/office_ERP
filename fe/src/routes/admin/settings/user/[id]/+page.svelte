<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        Settings,
        User as UserIcon,
        Edit,
        Trash2,
        Save,
        X,
        Key,
        Loader2,
        UserCircle,
        Mail,
        Phone,
        Calendar,
        Shield,
        ArrowLeft,
    } from 'lucide-svelte';
    import PageSection from '../../../../../components/PageSection.svelte';
    import { userService } from '$lib/services/user-service';
    import { personService } from '$lib/services/person';
    import type { User } from '$lib/types/user';
    import type { Person } from '$lib/types/person';

    pageTitle.set({
        title: 'User Information',
        desc: 'View and manage user details.',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'User', icon: UserIcon },
    ]);

    let userId: string;
    let user: User | null = null;
    let person: Person | null = null;
    let loading = true;
    let isEditing = false;
    let isSaving = false;
    let showDeleteModal = false;
    let showPasswordModal = false;
    let errorMessage = '';
    let successMessage = '';

    // Edit form data
    let editData = {
        userName: '',
        email: '',
        phone: '',
        isAdmin: false,
    };

    // Password change data
    let passwordData = {
        newPassword: '',
        confirmPassword: '',
    };

    $: userId = $page.params.id;

    async function loadUserData() {
        loading = true;
        errorMessage = '';
        try {
            user = await userService.getById(userId);
            if (user) {
                // Load associated person
                try {
                    person = await personService.getById(user.personId);
                } catch (e) {
                    console.error('Failed to load person data', e);
                }

                // Initialize edit data
                editData = {
                    userName: user.userName,
                    email: user.email,
                    phone: user.phone,
                    isAdmin: user.isAdmin,
                };
            }
        } catch (e: any) {
            errorMessage = e.message || 'Failed to load user data';
        } finally {
            loading = false;
        }
    }

    function enableEdit() {
        isEditing = true;
        successMessage = '';
        errorMessage = '';
    }

    function cancelEdit() {
        isEditing = false;
        errorMessage = '';
        if (user) {
            editData = {
                userName: user.userName,
                email: user.email,
                phone: user.phone,
                isAdmin: user.isAdmin,
            };
        }
    }

    async function saveChanges() {
        isSaving = true;
        errorMessage = '';
        successMessage = '';
        try {
            await userService.update(userId, editData);
            successMessage = 'User updated successfully!';
            isEditing = false;
            await loadUserData();
        } catch (e: any) {
            errorMessage = e.message || 'Failed to update user';
        } finally {
            isSaving = false;
        }
    }

    async function handleDelete() {
        try {
            await userService.delete(userId);
            goto('/admin/settings/user');
        } catch (e: any) {
            errorMessage = e.message || 'Failed to delete user';
            showDeleteModal = false;
        }
    }

    async function handlePasswordChange() {
        if (passwordData.newPassword !== passwordData.confirmPassword) {
            errorMessage = 'Passwords do not match';
            return;
        }
        if (passwordData.newPassword.length < 6) {
            errorMessage = 'Password must be at least 6 characters';
            return;
        }

        try {
            await userService.changePassword(userId, passwordData.newPassword);
            successMessage = 'Password changed successfully!';
            showPasswordModal = false;
            passwordData = { newPassword: '', confirmPassword: '' };
        } catch (e: any) {
            errorMessage = e.message || 'Failed to change password';
        }
    }

    onMount(loadUserData);
</script>

<PageSection title={user?.userName || 'User Information'}>
    <!-- Back Button -->
    <div class="mb-4">
        <button
            class="btn btn-ghost btn-sm gap-2"
            on:click={() => goto('/admin/settings/user')}>
            <ArrowLeft size={16} />
            Back to Users
        </button>
    </div>

    {#if successMessage}
        <div class="alert alert-success mb-4">
            <span>{successMessage}</span>
        </div>
    {/if}

    {#if errorMessage}
        <div class="alert alert-error mb-4">
            <span>{errorMessage}</span>
        </div>
    {/if}

    {#if loading}
        <div class="flex justify-center p-8">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    {:else if user}
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <!-- Main User Information -->
            <div class="lg:col-span-2 space-y-6">
                <!-- User Details Card -->
                <div class="card bg-base-100 shadow-lg">
                    <div class="card-body">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="card-title text-2xl">User Details</h2>
                            {#if !isEditing}
                                <div class="flex gap-2">
                                    <button
                                        class="btn btn-primary btn-sm gap-2"
                                        on:click={enableEdit}>
                                        <Edit size={16} />
                                        Edit
                                    </button>
                                    <button
                                        class="btn btn-error btn-sm gap-2"
                                        on:click={() =>
                                            (showDeleteModal = true)}>
                                        <Trash2 size={16} />
                                        Delete
                                    </button>
                                </div>
                            {/if}
                        </div>

                        {#if isEditing}
                            <!-- Edit Form -->
                            <form
                                on:submit|preventDefault={saveChanges}
                                class="space-y-4">
                                <div class="form-control">
                                    <label class="label" for="userName">
                                        <span class="label-text">Username</span>
                                    </label>
                                    <input
                                        id="userName"
                                        type="text"
                                        class="input input-bordered w-full"
                                        bind:value={editData.userName}
                                        required />
                                </div>

                                <div class="form-control">
                                    <label class="label" for="email">
                                        <span class="label-text">Email</span>
                                    </label>
                                    <input
                                        id="email"
                                        type="email"
                                        class="input input-bordered w-full"
                                        bind:value={editData.email}
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
                                        bind:value={editData.phone}
                                        required />
                                </div>

                                <div class="form-control">
                                    <label
                                        class="label cursor-pointer justify-start gap-4">
                                        <span class="label-text"
                                            >Admin User</span>
                                        <input
                                            type="checkbox"
                                            class="toggle toggle-primary"
                                            bind:checked={editData.isAdmin} />
                                    </label>
                                </div>

                                <div class="flex gap-2 pt-4">
                                    <button
                                        type="submit"
                                        class="btn btn-primary gap-2"
                                        disabled={isSaving}>
                                        {#if isSaving}
                                            <Loader2
                                                class="animate-spin"
                                                size={16} />
                                            Saving...
                                        {:else}
                                            <Save size={16} />
                                            Save Changes
                                        {/if}
                                    </button>
                                    <button
                                        type="button"
                                        class="btn btn-ghost gap-2"
                                        on:click={cancelEdit}
                                        disabled={isSaving}>
                                        <X size={16} />
                                        Cancel
                                    </button>
                                </div>
                            </form>
                        {:else}
                            <!-- View Mode -->
                            <div class="space-y-4">
                                <div class="flex items-start gap-3">
                                    <UserCircle
                                        class="text-primary mt-1"
                                        size={20} />
                                    <div>
                                        <p class="text-sm opacity-70">
                                            Username
                                        </p>
                                        <p class="text-lg font-bold">
                                            {user.userName}
                                        </p>
                                    </div>
                                </div>

                                <div class="flex items-start gap-3">
                                    <Mail class="text-primary mt-1" size={20} />
                                    <div>
                                        <p class="text-sm opacity-70">Email</p>
                                        <p class="text-lg">{user.email}</p>
                                    </div>
                                </div>

                                <div class="flex items-start gap-3">
                                    <Phone
                                        class="text-primary mt-1"
                                        size={20} />
                                    <div>
                                        <p class="text-sm opacity-70">Phone</p>
                                        <p class="text-lg">{user.phone}</p>
                                    </div>
                                </div>

                                <div class="flex items-start gap-3">
                                    <Shield
                                        class="text-primary mt-1"
                                        size={20} />
                                    <div>
                                        <p class="text-sm opacity-70">Role</p>
                                        <p class="text-lg">
                                            {#if user.isAdmin}
                                                <span
                                                    class="badge badge-primary"
                                                    >Administrator</span>
                                            {:else}
                                                <span class="badge badge-ghost"
                                                    >User</span>
                                            {/if}
                                        </p>
                                    </div>
                                </div>

                                <div class="flex items-start gap-3">
                                    <Calendar
                                        class="text-primary mt-1"
                                        size={20} />
                                    <div>
                                        <p class="text-sm opacity-70">
                                            Created At
                                        </p>
                                        <p class="text-lg">
                                            {new Date(
                                                user.createdAt,
                                            ).toLocaleString()}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        {/if}
                    </div>
                </div>

                <!-- Associated Person Card -->
                {#if person}
                    <div class="card bg-base-100 shadow-lg">
                        <div class="card-body">
                            <h2 class="card-title">Associated Person</h2>
                            <div class="space-y-3">
                                <div>
                                    <p class="text-sm opacity-70">Full Name</p>
                                    <p class="text-lg font-semibold">
                                        {person.first_name}
                                        {person.middle_name || ''}
                                        {person.last_name || ''}
                                    </p>
                                </div>
                                {#if person.date_of_birth}
                                    <div>
                                        <p class="text-sm opacity-70">
                                            Date of Birth
                                        </p>
                                        <p class="text-lg">
                                            {new Date(
                                                person.date_of_birth,
                                            ).toLocaleDateString()}
                                        </p>
                                    </div>
                                {/if}
                                {#if person.citizenship_number}
                                    <div>
                                        <p class="text-sm opacity-70">
                                            Citizenship Number
                                        </p>
                                        <p class="text-lg">
                                            {person.citizenship_number}
                                        </p>
                                    </div>
                                {/if}
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Sidebar Actions -->
            <div class="space-y-6">
                <!-- Quick Actions Card -->
                <div class="card bg-base-100 shadow-lg">
                    <div class="card-body">
                        <h2 class="card-title">Quick Actions</h2>
                        <div class="space-y-2">
                            <button
                                class="btn btn-outline btn-block gap-2"
                                on:click={() => (showPasswordModal = true)}>
                                <Key size={16} />
                                Change Password
                            </button>
                        </div>
                    </div>
                </div>

                <!-- Account Status Card -->
                <div class="card bg-base-100 shadow-lg">
                    <div class="card-body">
                        <h2 class="card-title">Account Status</h2>
                        <div class="space-y-3">
                            <div>
                                <p class="text-sm opacity-70">Status</p>
                                <span class="badge badge-success">Active</span>
                            </div>
                            <div>
                                <p class="text-sm opacity-70">User ID</p>
                                <p class="text-xs font-mono break-all">
                                    {user.id}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    {:else}
        <div class="alert alert-warning">
            <span>User not found</span>
        </div>
    {/if}
</PageSection>

<!-- Delete Confirmation Modal -->
{#if showDeleteModal}
    <dialog class="modal modal-open">
        <div class="modal-box">
            <h3 class="font-bold text-lg">Delete User</h3>
            <p class="py-4">
                Are you sure you want to delete this user? This action cannot be
                undone.
            </p>
            <div class="modal-action">
                <button
                    class="btn btn-ghost"
                    on:click={() => (showDeleteModal = false)}>Cancel</button>
                <button class="btn btn-error" on:click={handleDelete}>
                    Delete
                </button>
            </div>
        </div>
        <form
            method="dialog"
            class="modal-backdrop"
            on:click={() => (showDeleteModal = false)}>
            <button>close</button>
        </form>
    </dialog>
{/if}

<!-- Password Change Modal -->
{#if showPasswordModal}
    <dialog class="modal modal-open">
        <div class="modal-box">
            <h3 class="font-bold text-lg mb-4">Change Password</h3>

            <form
                on:submit|preventDefault={handlePasswordChange}
                class="space-y-4">
                <div class="form-control">
                    <label class="label" for="newPassword">
                        <span class="label-text">New Password</span>
                    </label>
                    <input
                        id="newPassword"
                        type="password"
                        class="input input-bordered w-full"
                        bind:value={passwordData.newPassword}
                        required
                        minlength="6" />
                </div>

                <div class="form-control">
                    <label class="label" for="confirmPassword">
                        <span class="label-text">Confirm Password</span>
                    </label>
                    <input
                        id="confirmPassword"
                        type="password"
                        class="input input-bordered w-full"
                        bind:value={passwordData.confirmPassword}
                        required
                        minlength="6" />
                </div>

                <div class="modal-action">
                    <button
                        type="button"
                        class="btn btn-ghost"
                        on:click={() => {
                            showPasswordModal = false;
                            passwordData = {
                                newPassword: '',
                                confirmPassword: '',
                            };
                        }}>Cancel</button>
                    <button type="submit" class="btn btn-primary">
                        Change Password
                    </button>
                </div>
            </form>
        </div>
        <form
            method="dialog"
            class="modal-backdrop"
            on:click={() => {
                showPasswordModal = false;
                passwordData = { newPassword: '', confirmPassword: '' };
            }}>
            <button>close</button>
        </form>
    </dialog>
{/if}
