<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { userStore } from '$lib/stores/user';
    import {
        Home,
        CircleUser,
        Edit,
        Save,
        X,
        Key,
        Loader2,
        User as UserIcon,
        Mail,
        Phone,
        Calendar,
        Shield,
    } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import { userService } from '$lib/services/user-service';
    import { personService } from '$lib/services/person';
    import type { User } from '$lib/types/user';
    import type { Person } from '$lib/types/person';

    pageTitle.set({
        title: 'Profile Information',
        desc: 'Your personal and account details',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Profile', icon: CircleUser },
    ]);

    let person: Person | null = null;
    let loading = true;
    let isEditingUser = false;
    let isEditingPerson = false;
    let isSaving = false;
    let showPasswordModal = false;
    let errorMessage = '';
    let successMessage = '';

    // Reactively get current user from store
    $: currentUser = $userStore.user;

    // Edit form data
    let userEditData = {
        userName: '',
        email: '',
        phone: '',
    };

    let personEditData = {
        first_name: '',
        middle_name: '',
        last_name: '',
    };

    // Password change data
    let passwordData = {
        newPassword: '',
        confirmPassword: '',
    };

    // Load current user and person data
    async function loadProfileData() {
        loading = true;
        errorMessage = '';
        try {
            if (currentUser && currentUser.personId) {
                // Load associated person
                person = await personService.getMe();

                // Initialize edit data
                userEditData = {
                    userName: currentUser.userName,
                    email: currentUser.email,
                    phone: currentUser.phone,
                };

                personEditData = {
                    first_name: person.first_name,
                    middle_name: person.middle_name || '',
                    last_name: person.last_name || '',
                };
            }
        } catch (e: any) {
            errorMessage = e.message || 'Failed to load profile data';
        } finally {
            loading = false;
        }
    }

    // Watch for changes in currentUser and reload data
    $: if (currentUser) {
        loadProfileData();
    }

    function enableUserEdit() {
        isEditingUser = true;
        successMessage = '';
        errorMessage = '';
    }

    function cancelUserEdit() {
        isEditingUser = false;
        errorMessage = '';
        if (currentUser) {
            userEditData = {
                userName: currentUser.userName,
                email: currentUser.email,
                phone: currentUser.phone,
            };
        }
    }

    function enablePersonEdit() {
        isEditingPerson = true;
        successMessage = '';
        errorMessage = '';
    }

    function cancelPersonEdit() {
        isEditingPerson = false;
        errorMessage = '';
        if (person) {
            personEditData = {
                first_name: person.first_name,
                middle_name: person.middle_name || '',
                last_name: person.last_name || '',
            };
        }
    }

    async function saveUserChanges() {
        if (!currentUser) return;

        isSaving = true;
        errorMessage = '';
        successMessage = '';
        try {
            await userService.update(currentUser.id, userEditData);
            successMessage = 'User information updated successfully!';
            isEditingUser = false;
            await loadProfileData();
        } catch (e: any) {
            errorMessage = e.message || 'Failed to update user information';
        } finally {
            isSaving = false;
        }
    }

    async function savePersonChanges() {
        if (!person) return;

        isSaving = true;
        errorMessage = '';
        successMessage = '';
        try {
            await personService.update(person.id, {
                first_name: personEditData.first_name || undefined,
                middle_name: personEditData.middle_name || undefined,
                last_name: personEditData.last_name || undefined,
            });
            successMessage = 'Personal information updated successfully!';
            isEditingPerson = false;
            await loadProfileData();
        } catch (e: any) {
            errorMessage = e.message || 'Failed to update personal information';
        } finally {
            isSaving = false;
        }
    }

    async function handlePasswordChange() {
        if (!currentUser) return;

        if (passwordData.newPassword !== passwordData.confirmPassword) {
            errorMessage = 'Passwords do not match';
            return;
        }
        if (passwordData.newPassword.length < 6) {
            errorMessage = 'Password must be at least 6 characters';
            return;
        }

        try {
            await userService.changePassword(
                currentUser.id,
                passwordData.newPassword,
            );
            successMessage = 'Password changed successfully!';
            showPasswordModal = false;
            passwordData = { newPassword: '', confirmPassword: '' };
        } catch (e: any) {
            errorMessage = e.message || 'Failed to change password';
        }
    }
</script>

<PageSection title="My Profile">
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
    {:else if currentUser}
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <!-- Main Profile Information -->
            <div class="lg:col-span-2 space-y-6">
                <!-- Personal Information Card -->
                <div class="card bg-base-100 shadow-lg">
                    <div class="card-body">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="card-title text-xl">
                                Personal Information
                            </h2>
                            {#if !isEditingPerson}
                                <button
                                    class="btn btn-primary btn-sm gap-2"
                                    on:click={enablePersonEdit}>
                                    <Edit size={16} />
                                    Edit
                                </button>
                            {/if}
                        </div>

                        {#if isEditingPerson && person}
                            <!-- Edit Form -->
                            <form
                                on:submit|preventDefault={savePersonChanges}
                                class="space-y-4">
                                <div class="form-control">
                                    <label class="label" for="first_name">
                                        <span class="label-text"
                                            >First Name</span>
                                    </label>
                                    <input
                                        id="first_name"
                                        type="text"
                                        class="input input-bordered w-full"
                                        bind:value={personEditData.first_name}
                                        required />
                                </div>

                                <div class="form-control">
                                    <label class="label" for="middle_name">
                                        <span class="label-text"
                                            >Middle Name</span>
                                    </label>
                                    <input
                                        id="middle_name"
                                        type="text"
                                        class="input input-bordered w-full"
                                        bind:value={
                                            personEditData.middle_name
                                        } />
                                </div>

                                <div class="form-control">
                                    <label class="label" for="last_name">
                                        <span class="label-text"
                                            >Last Name</span>
                                    </label>
                                    <input
                                        id="last_name"
                                        type="text"
                                        class="input input-bordered w-full"
                                        bind:value={personEditData.last_name}
                                        required />
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
                                        on:click={cancelPersonEdit}
                                        disabled={isSaving}>
                                        <X size={16} />
                                        Cancel
                                    </button>
                                </div>
                            </form>
                        {:else if person}
                            <!-- View Mode -->
                            <div class="space-y-4">
                                <div class="flex items-start gap-3">
                                    <UserIcon
                                        class="text-primary mt-1"
                                        size={20} />
                                    <div>
                                        <p class="text-sm opacity-70">
                                            Full Name
                                        </p>
                                        <p class="text-lg font-semibold">
                                            {person.first_name}
                                            {person.middle_name || ''}
                                            {person.last_name || ''}
                                        </p>
                                    </div>
                                </div>

                                <div class="flex items-start gap-3">
                                    <Calendar
                                        class="text-primary mt-1"
                                        size={20} />
                                    <div>
                                        <p class="text-sm opacity-70">
                                            Member Since
                                        </p>
                                        <p class="text-lg">
                                            {new Date(
                                                person.created_at,
                                            ).toLocaleDateString()}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        {/if}
                    </div>
                </div>

                <!-- Account Information Card -->
                <div class="card bg-base-100 shadow-lg">
                    <div class="card-body">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="card-title text-xl">
                                Account Information
                            </h2>
                            {#if !isEditingUser}
                                <button
                                    class="btn btn-primary btn-sm gap-2"
                                    on:click={enableUserEdit}>
                                    <Edit size={16} />
                                    Edit
                                </button>
                            {/if}
                        </div>

                        {#if isEditingUser}
                            <!-- Edit Form -->
                            <form
                                on:submit|preventDefault={saveUserChanges}
                                class="space-y-4">
                                <div class="form-control">
                                    <label class="label" for="userName">
                                        <span class="label-text">Username</span>
                                    </label>
                                    <input
                                        id="userName"
                                        type="text"
                                        class="input input-bordered w-full"
                                        bind:value={userEditData.userName}
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
                                        bind:value={userEditData.email}
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
                                        bind:value={userEditData.phone}
                                        required />
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
                                        on:click={cancelUserEdit}
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
                                    <UserIcon
                                        class="text-primary mt-1"
                                        size={20} />
                                    <div>
                                        <p class="text-sm opacity-70">
                                            Username
                                        </p>
                                        <p class="text-lg font-bold">
                                            {currentUser.userName}
                                        </p>
                                    </div>
                                </div>

                                <div class="flex items-start gap-3">
                                    <Mail class="text-primary mt-1" size={20} />
                                    <div>
                                        <p class="text-sm opacity-70">Email</p>
                                        <p class="text-lg">
                                            {currentUser.email}
                                        </p>
                                    </div>
                                </div>

                                <div class="flex items-start gap-3">
                                    <Phone
                                        class="text-primary mt-1"
                                        size={20} />
                                    <div>
                                        <p class="text-sm opacity-70">Phone</p>
                                        <p class="text-lg">
                                            {currentUser.phone}
                                        </p>
                                    </div>
                                </div>

                                <div class="flex items-start gap-3">
                                    <Shield
                                        class="text-primary mt-1"
                                        size={20} />
                                    <div>
                                        <p class="text-sm opacity-70">Role</p>
                                        <p class="text-lg">
                                            {#if currentUser.isAdmin}
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
                                            Account Created
                                        </p>
                                        <p class="text-lg">
                                            {new Date(
                                                currentUser.createdAt,
                                            ).toLocaleString()}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        {/if}
                    </div>
                </div>
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
                        <h2 class="card-title">Account Details</h2>
                        <div class="space-y-3">
                            <div>
                                <p class="text-sm opacity-70">Status</p>
                                <span class="badge badge-success">Active</span>
                            </div>
                            <div>
                                <p class="text-sm opacity-70">User ID</p>
                                <p class="text-xs font-mono break-all">
                                    {currentUser.id}
                                </p>
                            </div>
                            {#if person}
                                <div>
                                    <p class="text-sm opacity-70">Person ID</p>
                                    <p class="text-xs font-mono break-all">
                                        {person.id}
                                    </p>
                                </div>
                            {/if}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    {:else}
        <div class="alert alert-warning">
            <span>Unable to load profile information</span>
        </div>
    {/if}
</PageSection>

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
                    <div class="label">
                        <span class="label-text-alt">Minimum 6 characters</span>
                    </div>
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
                            errorMessage = '';
                        }}>Cancel</button>
                    <button type="submit" class="btn btn-primary">
                        Change Password
                    </button>
                </div>
            </form>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button
                type="button"
                on:click={() => {
                    showPasswordModal = false;
                    passwordData = { newPassword: '', confirmPassword: '' };
                    errorMessage = '';
                }}>
                close
            </button>
        </form>
    </dialog>
{/if}
