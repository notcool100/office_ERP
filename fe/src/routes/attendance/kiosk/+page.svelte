<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { FaceRecognitionService } from '$lib/utils/face_recognition';
    import { getAllFaceDescriptors } from '$lib/services/employee';
    import { listEmployees } from '$lib/services/employee';
    import { checkIn } from '$lib/services/attendance';
    import type { Employee } from '$lib/types/employee';
    import { Camera, MapPin, CheckCircle, XCircle } from 'lucide-svelte';
    import * as faceapi from 'face-api.js';

    let videoEl: HTMLVideoElement;
    let canvasEl: HTMLCanvasElement;
    let status = 'Loading System...';
    let isProcessing = false;
    let lastCheckInTime = 0;

    let faceService = FaceRecognitionService.getInstance();
    let faceMatcher: faceapi.FaceMatcher | null = null;
    let employees: Employee[] = [];
    let stream: MediaStream | null = null;
    let interval: any;

    // UI State
    let flashMessage = '';
    let flashType: 'success' | 'error' = 'success';

    onMount(async () => {
        try {
            await Promise.all([faceService.loadModels(), loadData()]);

            await startCamera();
            status = 'Ready. Please look at the camera.';
        } catch (e) {
            status = 'System Error: ' + e;
        }
    });

    onDestroy(() => {
        stopCamera();
        if (interval) clearInterval(interval);
    });

    async function loadData() {
        // Load Employees for display names
        const empRes = await listEmployees({
            pageSize: 1000,
            status: 'active',
        });
        employees = empRes.employees;

        // Load Descriptors
        const rawDescriptors = await getAllFaceDescriptors();
        if (rawDescriptors.length > 0) {
            const labeled = rawDescriptors.map(([id, descStr]) => {
                const arr = JSON.parse(descStr); // array of numbers
                const f32 = new Float32Array(arr);
                return new faceapi.LabeledFaceDescriptors(id, [f32]);
            });
            faceMatcher = faceService.createMatcher(labeled);
        }
    }

    async function startCamera() {
        stream = await navigator.mediaDevices.getUserMedia({ video: {} });
        videoEl.srcObject = stream;
        startLoop();
    }

    function stopCamera() {
        if (stream) {
            stream.getTracks().forEach((t) => t.stop());
            stream = null;
        }
    }

    function startLoop() {
        interval = setInterval(async () => {
            if (
                !videoEl ||
                videoEl.paused ||
                videoEl.ended ||
                !faceMatcher ||
                isProcessing
            )
                return;

            // Cooldown check (5 seconds)
            if (Date.now() - lastCheckInTime < 5000) return;

            const detections = await faceService.getAllFaces(videoEl);

            if (detections.length > 0) {
                // Find best match
                for (const d of detections) {
                    const match = faceMatcher.findBestMatch(d.descriptor);

                    // Match found (label is UUID)
                    if (match.label !== 'unknown' && match.distance < 0.5) {
                        // Strict threshold
                        await handleCheckIn(match.label);
                        break; // Only one check-in at a time
                    }
                }
            }
        }, 1000); // Check every 1s
    }

    async function handleCheckIn(employeeId: string) {
        if (isProcessing) return;
        isProcessing = true;

        const emp = employees.find((e) => e.id === employeeId);
        const name = emp ? `${emp.firstName} ${emp.lastName}` : 'Employee';

        try {
            status = `Identifying ${name}...`;

            // Get Location
            let lat, long;
            try {
                const pos = await getCurrentPosition();
                lat = pos.coords.latitude;
                long = pos.coords.longitude;
            } catch (e) {
                console.warn('Location failed', e);
            }

            // Capture Image Snapshot (optional, for logs)
            const snapshot = captureImage();

            await checkIn({
                employeeId: emp ? emp.employeeId : employeeId, // API expects String Code or UUID? checkIn expects 'employeeId' string code usually?
                // Wait, backend 'req.employee_id' matches 'employees.employee_id'.
                // But the face label is 'employees.id' (UUID).
                // So I need to map UUID -> employeeId (Code).
                // existing logic: `employees` map has both.

                // Let's verify CheckInRequest expects employee_id string column value.
                // Backend: "SELECT id FROM employees WHERE employee_id = $1"
                // So yes, it expects the String Code (e.g. EMP001).

                method: 'FACE',
                image: snapshot,
                latitude: lat,
                longitude: long,
            });

            flashMessage = `Welcome, ${name}! Checked In.`;
            flashType = 'success';
            lastCheckInTime = Date.now();
            status = 'Checked In Successfully.';
        } catch (e: any) {
            console.error(e);
            flashMessage = `Check-in Failed: ${e.message || 'Unknown Error'}`;
            flashType = 'error';
        } finally {
            isProcessing = false;
            setTimeout(() => {
                flashMessage = '';
                status = 'Ready. Please look at the camera.';
            }, 3000); // Clear message after 3s
        }
    }

    function getCurrentPosition() {
        return new Promise<GeolocationPosition>((resolve, reject) => {
            navigator.geolocation.getCurrentPosition(resolve, reject, {
                timeout: 5000,
            });
        });
    }

    function captureImage() {
        const canvas = document.createElement('canvas');
        canvas.width = videoEl.videoWidth;
        canvas.height = videoEl.videoHeight;
        canvas.getContext('2d')?.drawImage(videoEl, 0, 0);
        return canvas.toDataURL('image/jpeg', 0.8);
    }
</script>

<div
    class="min-h-screen bg-neutral text-neutral-content flex flex-col items-center justify-center relative overflow-hidden">
    <!-- Camera Feed -->
    <video
        bind:this={videoEl}
        autoplay
        muted
        playsinline
        class="absolute inset-0 w-full h-full object-cover opacity-60"></video>

    <!-- Overlay UI -->
    <div class="relative z-10 p-8 w-full max-w-2xl text-center space-y-8">
        <div
            class="card bg-base-100/90 backdrop-blur shadow-2xl p-8 border-t-8 border-primary">
            <h1 class="text-4xl font-black text-base-content mb-2">
                Office Attendance
            </h1>
            <p class="text-xl text-base-content/70">{status}</p>
        </div>

        {#if flashMessage}
            <div
                class="alert {flashType === 'success'
                    ? 'alert-success'
                    : 'alert-error'} shadow-lg animate-bounce">
                {#if flashType === 'success'}
                    <CheckCircle />
                {:else}
                    <XCircle />
                {/if}
                <span class="text-xl font-bold">{flashMessage}</span>
            </div>
        {/if}
    </div>

    <!-- Powered By -->
    <div
        class="absolute bottom-4 left-0 right-0 text-center text-sm opacity-50">
        Secured by FaceID
    </div>
</div>
