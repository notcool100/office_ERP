<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Settings, Building2, Loader2 } from 'lucide-svelte';
    import {
        companySettingsService,
        type CompanySettings,
    } from '$lib/services/company-settings';
    import { buildApiUrl } from '$lib/services/api';
    import 'leaflet/dist/leaflet.css';
    import type * as Leaflet from 'leaflet';
    import markerIcon2x from 'leaflet/dist/images/marker-icon-2x.png';
    import markerIcon from 'leaflet/dist/images/marker-icon.png';
    import markerShadow from 'leaflet/dist/images/marker-shadow.png';

    pageTitle.set({ title: 'Company', desc: 'Manage your company profile, branding and office location.' });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'Company', icon: Building2 },
    ]);

    const DEFAULT_LAT = 27.7172;
    const DEFAULT_LNG = 85.324;

    let settings: CompanySettings | null = null;
    let loading = true;
    let loadError = '';

    let name = '';
    let savingName = false;
    let nameError = '';
    let nameSuccess = '';

    let logoFile: HTMLInputElement | null = null;
    let uploadingLogo = false;
    let logoError = '';

    let faviconFile: HTMLInputElement | null = null;
    let uploadingFavicon = false;
    let faviconError = '';

    let mapContainer: HTMLDivElement | null = null;
    let map: Leaflet.Map | null = null;
    let marker: Leaflet.Marker | null = null;
    let latitude = DEFAULT_LAT;
    let longitude = DEFAULT_LNG;
    let locationName = '';
    let geocoding = false;
    let geocodeError = '';
    let savingLocation = false;
    let locationError = '';
    let locationSuccess = '';

    async function load() {
        loading = true;
        loadError = '';
        try {
            settings = await companySettingsService.getSettings();
            name = settings.name;
            latitude = settings.latitude ?? DEFAULT_LAT;
            longitude = settings.longitude ?? DEFAULT_LNG;
            locationName = settings.location_name ?? '';
        } catch (e: any) {
            loadError = e.message ?? 'Failed to load company settings';
        } finally {
            loading = false;
        }
    }

    async function saveName() {
        if (!settings || !name.trim()) return;
        savingName = true;
        nameError = '';
        nameSuccess = '';
        try {
            settings = await companySettingsService.updateDetails({
                name: name.trim(),
                latitude: settings.latitude,
                longitude: settings.longitude,
                location_name: settings.location_name,
            });
            name = settings.name;
            nameSuccess = 'Company name updated.';
            setTimeout(() => { nameSuccess = ''; }, 3000);
        } catch (e: any) {
            nameError = e.message ?? 'Failed to update company name';
        } finally {
            savingName = false;
        }
    }

    async function handleLogoChange(e: Event) {
        const target = e.target as HTMLInputElement;
        const file = target.files?.[0];
        if (!file) return;
        uploadingLogo = true;
        logoError = '';
        try {
            settings = await companySettingsService.uploadLogo(file);
        } catch (e: any) {
            logoError = e.message ?? 'Failed to upload logo';
        } finally {
            uploadingLogo = false;
            if (logoFile) logoFile.value = '';
        }
    }

    async function handleFaviconChange(e: Event) {
        const target = e.target as HTMLInputElement;
        const file = target.files?.[0];
        if (!file) return;
        uploadingFavicon = true;
        faviconError = '';
        try {
            settings = await companySettingsService.uploadFavicon(file);
        } catch (e: any) {
            faviconError = e.message ?? 'Failed to upload favicon';
        } finally {
            uploadingFavicon = false;
            if (faviconFile) faviconFile.value = '';
        }
    }

    async function reverseGeocode(lat: number, lng: number) {
        geocoding = true;
        geocodeError = '';
        try {
            const res = await fetch(
                `https://nominatim.openstreetmap.org/reverse?format=json&lat=${lat}&lon=${lng}`
            );
            if (!res.ok) throw new Error('Failed to look up address');
            const data = await res.json();
            locationName = data.display_name ?? '';
            if (!locationName) geocodeError = 'No address found for this location.';
        } catch {
            geocodeError = 'Failed to look up address for this location.';
        } finally {
            geocoding = false;
        }
    }

    async function saveLocation() {
        if (!settings) return;
        savingLocation = true;
        locationError = '';
        locationSuccess = '';
        try {
            settings = await companySettingsService.updateDetails({
                name: settings.name,
                latitude,
                longitude,
                location_name: locationName || null,
            });
            locationSuccess = 'Location saved.';
            setTimeout(() => { locationSuccess = ''; }, 3000);
        } catch (e: any) {
            locationError = e.message ?? 'Failed to save location';
        } finally {
            savingLocation = false;
        }
    }

    async function initMap() {
        if (!mapContainer || map) return;

        const { default: L } = await import('leaflet');

        L.Icon.Default.mergeOptions({
            iconRetinaUrl: markerIcon2x,
            iconUrl: markerIcon,
            shadowUrl: markerShadow,
        });

        map = L.map(mapContainer).setView([latitude, longitude], 13);

        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
        }).addTo(map);

        marker = L.marker([latitude, longitude], { draggable: true }).addTo(map);

        marker.on('dragend', () => {
            if (!marker) return;
            const pos = marker.getLatLng();
            latitude = pos.lat;
            longitude = pos.lng;
            reverseGeocode(latitude, longitude);
        });

        if (settings?.latitude != null && settings?.longitude != null) {
            reverseGeocode(latitude, longitude);
        }
    }

    onMount(async () => {
        await load();
        await initMap();
    });

    onDestroy(() => {
        map?.remove();
        map = null;
    });
</script>

{#if loading}
    <div class="flex justify-center p-12">
        <span class="loading loading-spinner loading-lg"></span>
    </div>
{:else if loadError}
    <div class="p-6">
        <div class="alert alert-error text-sm">{loadError}</div>
    </div>
{:else}
    <div class="space-y-6 max-w-2xl p-6">
        <div class="card bg-base-100 border border-base-300 shadow-sm">
            <div class="card-body">
                <h3 class="font-semibold text-base mb-1">Company name</h3>
                <p class="text-sm text-base-content/60 mb-4">
                    This name appears across the app and on the login page.
                </p>

                <div class="form-control mb-3">
                    <label class="label" for="company-name">
                        <span class="label-text">Name</span>
                    </label>
                    <input
                        id="company-name"
                        type="text"
                        class="input input-bordered"
                        bind:value={name} />
                </div>

                <div class="flex items-center justify-between pt-2 border-t border-base-200">
                    {#if nameError}
                        <span class="text-sm text-error">{nameError}</span>
                    {:else if nameSuccess}
                        <span class="text-sm text-success">{nameSuccess}</span>
                    {:else}
                        <span></span>
                    {/if}
                    <button
                        class="btn btn-sm btn-primary"
                        disabled={savingName || !name.trim()}
                        on:click={saveName}>
                        {#if savingName}
                            <Loader2 class="w-4 h-4 animate-spin" />
                        {/if}
                        Save
                    </button>
                </div>
            </div>
        </div>

        <div class="card bg-base-100 border border-base-300 shadow-sm">
            <div class="card-body">
                <h3 class="font-semibold text-base mb-1">Logo</h3>
                <p class="text-sm text-base-content/60 mb-4">
                    Shown on the login page. Uploads immediately.
                </p>

                <div class="flex items-center gap-4 mb-3">
                    {#if settings?.logo_url}
                        <img
                            src={buildApiUrl(settings.logo_url)}
                            alt="Company logo"
                            class="h-16 w-auto rounded-lg border border-base-300" />
                    {:else}
                        <div class="h-16 w-16 rounded-lg border border-dashed border-base-300 flex items-center justify-center text-base-content/30 text-xs">
                            No logo
                        </div>
                    {/if}

                    <div class="flex-1">
                        <input
                            bind:this={logoFile}
                            type="file"
                            accept="image/*"
                            class="file-input file-input-bordered file-input-sm w-full"
                            disabled={uploadingLogo}
                            on:change={handleLogoChange} />
                    </div>

                    {#if uploadingLogo}
                        <Loader2 class="w-4 h-4 animate-spin" />
                    {/if}
                </div>

                {#if logoError}
                    <span class="text-sm text-error">{logoError}</span>
                {/if}
            </div>
        </div>

        <div class="card bg-base-100 border border-base-300 shadow-sm">
            <div class="card-body">
                <h3 class="font-semibold text-base mb-1">Favicon</h3>
                <p class="text-sm text-base-content/60 mb-4">
                    Shown in the browser tab. Uploads immediately.
                </p>

                <div class="flex items-center gap-4 mb-3">
                    {#if settings?.favicon_url}
                        <img
                            src={buildApiUrl(settings.favicon_url)}
                            alt="Company favicon"
                            class="h-8 w-8 rounded border border-base-300" />
                    {:else}
                        <div class="h-8 w-8 rounded border border-dashed border-base-300 flex items-center justify-center text-base-content/30 text-[10px]">
                            None
                        </div>
                    {/if}

                    <div class="flex-1">
                        <input
                            bind:this={faviconFile}
                            type="file"
                            accept="image/*"
                            class="file-input file-input-bordered file-input-sm w-full"
                            disabled={uploadingFavicon}
                            on:change={handleFaviconChange} />
                    </div>

                    {#if uploadingFavicon}
                        <Loader2 class="w-4 h-4 animate-spin" />
                    {/if}
                </div>

                {#if faviconError}
                    <span class="text-sm text-error">{faviconError}</span>
                {/if}
            </div>
        </div>

        <div class="card bg-base-100 border border-base-300 shadow-sm">
            <div class="card-body">
                <h3 class="font-semibold text-base mb-1">Office location</h3>
                <p class="text-sm text-base-content/60 mb-4">
                    Drag the marker to set the office's pinpoint location.
                </p>

                <div bind:this={mapContainer} class="w-full h-[380px] rounded-lg overflow-hidden border border-base-300"></div>

                <div class="mt-3 text-sm">
                    {#if geocoding}
                        <span class="text-base-content/50 flex items-center gap-1.5">
                            <Loader2 class="w-3.5 h-3.5 animate-spin" /> Looking up address...
                        </span>
                    {:else if geocodeError}
                        <span class="text-error">{geocodeError}</span>
                    {:else if locationName}
                        <span class="text-base-content/70">{locationName}</span>
                    {:else}
                        <span class="text-base-content/40">No address available yet.</span>
                    {/if}
                </div>

                <div class="flex items-center justify-between pt-3 mt-2 border-t border-base-200">
                    {#if locationError}
                        <span class="text-sm text-error">{locationError}</span>
                    {:else if locationSuccess}
                        <span class="text-sm text-success">{locationSuccess}</span>
                    {:else}
                        <span></span>
                    {/if}
                    <button
                        class="btn btn-sm btn-primary"
                        disabled={savingLocation}
                        on:click={saveLocation}>
                        {#if savingLocation}
                            <Loader2 class="w-4 h-4 animate-spin" />
                        {/if}
                        Save location
                    </button>
                </div>
            </div>
        </div>
    </div>
{/if}
