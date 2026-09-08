<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { FaceRecognitionService } from '$lib/utils/face_recognition';
    import { getAllFaceDescriptors, listEmployees } from '$lib/services/employee';
    import { checkIn, checkOut } from '$lib/services/attendance';
    import { companySettingsService } from '$lib/services/company-settings';
    import type { Employee } from '$lib/types/employee';
    import {
        Camera,
        MapPin,
        CheckCircle,
        XCircle,
        LogIn,
        LogOut,
        Loader2,
        AlertTriangle,
        ScanFace,
    } from 'lucide-svelte';
    import * as faceapi from 'face-api.js';

    const GEOFENCE_METERS = 300;
    const MATCH_DISTANCE_THRESHOLD = 0.5;
    const RECOGNITION_HOLD_MS = 2000;
    const LOCATION_POLL_MS = 10000;

    let videoEl: HTMLVideoElement;
    let stream: MediaStream | null = null;

    let faceService = FaceRecognitionService.getInstance();
    let faceMatcher: faceapi.FaceMatcher | null = null;
    let employees: Employee[] = [];

    let detectInterval: ReturnType<typeof setInterval> | undefined;
    let locationInterval: ReturnType<typeof setInterval> | undefined;
    let clockInterval: ReturnType<typeof setInterval> | undefined;

    // Boot / system state
    let booting = true;
    let systemError = '';

    // Live recognition state
    let recognized: Employee | null = null;
    let lastMatchAt = 0;
    let scanHint = 'Loading camera…';

    // Location state
    interface OfficeLocation {
        latitude: number | null;
        longitude: number | null;
        locationName: string | null;
    }
    let officeLocation: OfficeLocation | null = null;
    let officeLocationLoaded = false;
    let locationStatus: 'idle' | 'locating' | 'ok' | 'error' = 'idle';
    let distanceToOffice: number | null = null;

    $: officeConfigured =
        officeLocationLoaded &&
        officeLocation?.latitude != null &&
        officeLocation?.longitude != null;
    $: withinRange = distanceToOffice != null && distanceToOffice <= GEOFENCE_METERS;

    // Action state
    let isProcessing = false;
    let actionLabel = '';
    let flashMessage = '';
    let flashType: 'success' | 'error' = 'success';
    let flashTimer: ReturnType<typeof setTimeout> | undefined;

    // Clock
    let now = new Date();

    $: canAct = !booting && !isProcessing && !!recognized && officeConfigured;

    $: ringClass = systemError
        ? 'ring-error'
        : isProcessing
          ? 'ring-info animate-pulse'
          : recognized
            ? 'ring-success'
            : 'ring-base-content/10';

    onMount(async () => {
        try {
            await Promise.all([faceService.loadModels(), loadEmployeeData(), loadOfficeLocation()]);
            await startCamera();
            startDetectionLoop();
            await pollLocation();
            locationInterval = setInterval(pollLocation, LOCATION_POLL_MS);
            clockInterval = setInterval(() => (now = new Date()), 1000);
            booting = false;
            scanHint = 'Please look at the camera';
        } catch (e: any) {
            console.error('Kiosk init failed:', e);
            systemError = e?.message || 'Failed to start the attendance kiosk.';
        }
    });

    onDestroy(() => {
        stopCamera();
        if (detectInterval) clearInterval(detectInterval);
        if (locationInterval) clearInterval(locationInterval);
        if (clockInterval) clearInterval(clockInterval);
        if (flashTimer) clearTimeout(flashTimer);
    });

    async function loadEmployeeData() {
        const empRes = await listEmployees({ pageSize: 1000, status: 'active' });
        employees = empRes.employees;

        const rawDescriptors = await getAllFaceDescriptors();
        if (rawDescriptors.length > 0) {
            const labeled = rawDescriptors.map(([id, descStr]) => {
                const arr = JSON.parse(descStr);
                const f32 = new Float32Array(arr);
                return new faceapi.LabeledFaceDescriptors(id, [f32]);
            });
            faceMatcher = faceService.createMatcher(labeled);
        }
    }

    async function loadOfficeLocation() {
        try {
            const settings = await companySettingsService.getSettings();
            officeLocation = {
                latitude: settings.latitude,
                longitude: settings.longitude,
                locationName: settings.location_name,
            };
        } catch (e) {
            console.warn('Failed to load company settings:', e);
            officeLocation = { latitude: null, longitude: null, locationName: null };
        } finally {
            officeLocationLoaded = true;
        }
    }

    async function startCamera() {
        stream = await navigator.mediaDevices.getUserMedia({
            video: { facingMode: 'user', width: { ideal: 1280 }, height: { ideal: 720 } },
        });
        videoEl.srcObject = stream;
    }

    function stopCamera() {
        if (stream) {
            stream.getTracks().forEach((t) => t.stop());
            stream = null;
        }
    }

    function startDetectionLoop() {
        detectInterval = setInterval(async () => {
            if (!videoEl || videoEl.paused || videoEl.ended || !faceMatcher || isProcessing) return;

            const detections = await faceService.getAllFaces(videoEl);

            let matchedEmployee: Employee | null = null;
            for (const d of detections) {
                const match = faceMatcher.findBestMatch(d.descriptor);
                if (match.label === 'unknown' || match.distance >= MATCH_DISTANCE_THRESHOLD) continue;

                const emp = employees.find((e) => e.id === match.label);
                if (emp) {
                    matchedEmployee = emp;
                    break;
                }
            }

            if (matchedEmployee) {
                recognized = matchedEmployee;
                lastMatchAt = Date.now();
                scanHint = '';
            } else if (recognized && Date.now() - lastMatchAt > RECOGNITION_HOLD_MS) {
                recognized = null;
            }

            if (!recognized) {
                scanHint = detections.length > 0 ? 'Face not recognized' : 'Please look at the camera';
            }
        }, 900);
    }

    async function pollLocation() {
        locationStatus = 'locating';
        try {
            const pos = await getCurrentPosition();
            locationStatus = 'ok';
            if (officeLocation?.latitude != null && officeLocation?.longitude != null) {
                distanceToOffice = haversineMeters(
                    pos.coords.latitude,
                    pos.coords.longitude,
                    officeLocation.latitude,
                    officeLocation.longitude,
                );
            }
        } catch (e) {
            locationStatus = 'error';
            distanceToOffice = null;
        }
    }

    function haversineMeters(lat1: number, lon1: number, lat2: number, lon2: number): number {
        const R = 6371000;
        const toRad = (v: number) => (v * Math.PI) / 180;
        const dLat = toRad(lat2 - lat1);
        const dLon = toRad(lon2 - lon1);
        const a =
            Math.sin(dLat / 2) ** 2 +
            Math.cos(toRad(lat1)) * Math.cos(toRad(lat2)) * Math.sin(dLon / 2) ** 2;
        return R * 2 * Math.asin(Math.sqrt(a));
    }

    function formatDistance(meters: number): string {
        return meters < 1000 ? `${Math.round(meters)}m` : `${(meters / 1000).toFixed(1)}km`;
    }

    function greeting(): string {
        const h = new Date().getHours();
        if (h < 12) return 'Good morning';
        if (h < 17) return 'Good afternoon';
        return 'Good evening';
    }

    function initials(emp: Employee): string {
        return `${emp.firstName?.charAt(0) ?? ''}${emp.lastName?.charAt(0) ?? ''}`.toUpperCase();
    }

    function flash(message: string, type: 'success' | 'error') {
        flashMessage = message;
        flashType = type;
        if (flashTimer) clearTimeout(flashTimer);
        flashTimer = setTimeout(() => (flashMessage = ''), 4500);
    }

    async function handleAction(kind: 'in' | 'out') {
        if (isProcessing) return;

        if (!recognized) {
            flash('No face recognized. Please look directly at the camera.', 'error');
            return;
        }

        if (!officeConfigured) {
            flash('Office location is not configured. Please contact an administrator.', 'error');
            return;
        }

        if (distanceToOffice != null && distanceToOffice > GEOFENCE_METERS) {
            flash(
                `You're ${formatDistance(distanceToOffice)} from the office. Move within ${GEOFENCE_METERS}m to ${kind === 'in' ? 'check in' : 'check out'}.`,
                'error',
            );
            return;
        }

        const emp = recognized;
        const name = `${emp.firstName} ${emp.lastName}`;

        isProcessing = true;
        actionLabel = kind === 'in' ? 'Checking in…' : 'Checking out…';

        try {
            let latitude: number | undefined;
            let longitude: number | undefined;
            try {
                const pos = await getCurrentPosition();
                latitude = pos.coords.latitude;
                longitude = pos.coords.longitude;
            } catch (e) {
                console.warn('Location unavailable at action time', e);
            }

            const snapshot = captureImage();

            if (kind === 'in') {
                await checkIn({
                    employeeId: emp.employeeId,
                    method: 'FACE',
                    image: snapshot,
                    latitude,
                    longitude,
                });
                flash(`${greeting()}, ${name}! You're checked in.`, 'success');
            } else {
                await checkOut(emp.employeeId, {
                    method: 'FACE',
                    latitude,
                    longitude,
                });
                flash(`Goodbye, ${name}! You're checked out.`, 'success');
            }

            // Prevent an instant duplicate action on the same face.
            recognized = null;
        } catch (e: any) {
            let message: string = e?.message || 'Something went wrong. Please try again.';
            if (/check in before you can check out/i.test(message)) {
                message = `${name}, you haven't checked in yet today — please check in first.`;
            } else if (/already checked in today/i.test(message)) {
                message = `${name}, you've already checked in today.`;
            }
            flash(message, 'error');
        } finally {
            isProcessing = false;
            actionLabel = '';
        }
    }

    async function getCurrentPosition(): Promise<GeolocationPosition> {
        try {
            return await new Promise<GeolocationPosition>((resolve, reject) => {
                navigator.geolocation.getCurrentPosition(resolve, reject, {
                    enableHighAccuracy: true,
                    timeout: 6000,
                    maximumAge: 5000,
                });
            });
        } catch (e) {
            console.warn('GPS failed, falling back to IP location…', e);
            try {
                const res = await fetch('https://ipapi.co/json/');
                if (!res.ok) throw new Error('IP Geo failed');
                const data = await res.json();
                return {
                    coords: {
                        latitude: data.latitude,
                        longitude: data.longitude,
                        accuracy: 5000,
                        altitude: null,
                        altitudeAccuracy: null,
                        heading: null,
                        speed: null,
                    },
                    timestamp: Date.now(),
                } as GeolocationPosition;
            } catch (ipError) {
                console.error('IP fallback failed:', ipError);
                throw e;
            }
        }
    }

    function captureImage(): string {
        const canvas = document.createElement('canvas');
        canvas.width = videoEl.videoWidth;
        canvas.height = videoEl.videoHeight;
        canvas.getContext('2d')?.drawImage(videoEl, 0, 0);
        return canvas.toDataURL('image/jpeg', 0.8);
    }
</script>

<div
    class="min-h-screen bg-gradient-to-br from-neutral via-neutral to-base-300 flex items-center justify-center p-4 md:p-8">
    {#if systemError}
        <div class="card bg-base-100 shadow-2xl p-8 max-w-md text-center space-y-4">
            <AlertTriangle class="mx-auto text-error" size={48} />
            <h1 class="text-xl font-bold">Kiosk Unavailable</h1>
            <p class="opacity-70">{systemError}</p>
        </div>
    {:else}
        <div class="w-full max-w-5xl grid md:grid-cols-[1.15fr_1fr] gap-6">
            <!-- Camera Panel -->
            <div
                class="relative rounded-3xl overflow-hidden shadow-2xl bg-black aspect-[4/3] md:aspect-auto md:h-[600px] ring-4 transition-colors duration-500 {ringClass}">
                <video
                    bind:this={videoEl}
                    autoplay
                    muted
                    playsinline
                    class="w-full h-full object-cover -scale-x-100"></video>

                {#if booting}
                    <div
                        class="absolute inset-0 flex flex-col items-center justify-center gap-3 bg-black/70 text-white">
                        <Loader2 class="animate-spin" size={40} />
                        <p class="font-medium">Starting kiosk…</p>
                    </div>
                {/if}

                <!-- Top bar -->
                <div
                    class="absolute top-0 inset-x-0 p-4 flex justify-between items-start bg-gradient-to-b from-black/70 to-transparent text-white pointer-events-none">
                    <div class="flex items-center gap-2 font-semibold text-sm md:text-base">
                        <Camera size={18} />
                        Face Attendance Kiosk
                    </div>
                    <div class="text-right text-xs md:text-sm opacity-90 leading-tight">
                        <div class="font-semibold">
                            {now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                        </div>
                        <div class="opacity-70">
                            {now.toLocaleDateString([], {
                                weekday: 'short',
                                month: 'short',
                                day: 'numeric',
                            })}
                        </div>
                    </div>
                </div>

                <!-- Processing overlay -->
                {#if isProcessing}
                    <div
                        class="absolute inset-0 flex flex-col items-center justify-center gap-3 bg-black/60 text-white">
                        <Loader2 class="animate-spin" size={36} />
                        <p class="font-semibold">{actionLabel}</p>
                    </div>
                {/if}

                <!-- Bottom recognition banner -->
                <div
                    class="absolute bottom-0 inset-x-0 p-4 bg-gradient-to-t from-black/80 to-transparent text-white">
                    {#if recognized}
                        <div class="flex items-center gap-3">
                            <div class="avatar avatar-placeholder">
                                <div
                                    class="bg-success text-success-content rounded-full w-12 ring-2 ring-white/40">
                                    <span class="text-lg font-bold">{initials(recognized)}</span>
                                </div>
                            </div>
                            <div class="min-w-0">
                                <p class="font-bold text-lg leading-tight truncate">
                                    {recognized.firstName} {recognized.lastName}
                                </p>
                                <p class="text-xs opacity-80 truncate">
                                    {recognized.employeeId}{recognized.department
                                        ? ` · ${recognized.department}`
                                        : ''}
                                </p>
                            </div>
                            <CheckCircle class="ml-auto shrink-0 text-success" size={22} />
                        </div>
                    {:else if !booting}
                        <p class="text-sm opacity-80 flex items-center gap-2">
                            <ScanFace size={16} />
                            {scanHint}
                        </p>
                    {/if}
                </div>
            </div>

            <!-- Right Panel -->
            <div class="flex flex-col gap-4">
                <div class="card bg-base-100 shadow-xl p-6 flex-1 flex flex-col">
                    <h1 class="text-2xl font-black text-base-content">Office Attendance</h1>
                    <p class="text-sm text-base-content/60 mt-1">
                        Look at the camera, then tap Check In or Check Out.
                    </p>

                    <!-- Location status -->
                    <div
                        class="mt-5 flex items-start gap-3 rounded-xl p-3 border {officeConfigured
                            ? withinRange
                                ? 'bg-success/10 border-success/30 text-success'
                                : 'bg-error/10 border-error/30 text-error'
                            : 'bg-warning/10 border-warning/30 text-warning'}">
                        <MapPin size={18} class="mt-0.5 shrink-0" />
                        <div class="text-sm leading-snug">
                            {#if !officeLocationLoaded}
                                <span>Checking office location…</span>
                            {:else if !officeConfigured}
                                <span class="font-medium"
                                    >Office location not configured. Contact an administrator.</span>
                            {:else if locationStatus === 'locating' && distanceToOffice === null}
                                <span>Getting your location…</span>
                            {:else if locationStatus === 'error' && distanceToOffice === null}
                                <span class="font-medium"
                                    >Can't get your location. Enable location services to continue.</span>
                            {:else if distanceToOffice != null}
                                <span class="font-medium">
                                    {#if withinRange}
                                        You're within office range
                                    {:else}
                                        {formatDistance(distanceToOffice)} from office — too far
                                    {/if}
                                </span>
                                {#if officeLocation?.locationName}
                                    <div class="opacity-70 text-xs mt-0.5">{officeLocation.locationName}</div>
                                {/if}
                            {/if}
                        </div>
                    </div>

                    <!-- Action buttons -->
                    <div class="grid grid-cols-2 gap-3 mt-6">
                        <button
                            class="btn btn-success btn-lg h-auto py-4 flex-col gap-1"
                            disabled={!canAct}
                            on:click={() => handleAction('in')}>
                            <LogIn size={22} />
                            <span>Check In</span>
                        </button>
                        <button
                            class="btn btn-error btn-lg h-auto py-4 flex-col gap-1"
                            disabled={!canAct}
                            on:click={() => handleAction('out')}>
                            <LogOut size={22} />
                            <span>Check Out</span>
                        </button>
                    </div>

                    {#if !recognized && !booting}
                        <p class="text-xs text-center text-base-content/50 mt-3">
                            Buttons unlock once your face is recognized.
                        </p>
                    {/if}

                    <div class="mt-auto pt-6">
                        {#if flashMessage}
                            <div
                                class="alert {flashType === 'success'
                                    ? 'alert-success'
                                    : 'alert-error'} shadow-lg">
                                {#if flashType === 'success'}
                                    <CheckCircle size={20} />
                                {:else}
                                    <XCircle size={20} />
                                {/if}
                                <span class="font-semibold text-sm">{flashMessage}</span>
                            </div>
                        {/if}
                    </div>
                </div>

                <div class="text-center text-xs text-base-content/40">Secured by FaceID</div>
            </div>
        </div>
    {/if}
</div>
