<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { FaceRecognitionService } from '$lib/utils/face_recognition';
    import {
        listEmployees,
        updateFaceDescriptor,
    } from '$lib/services/employee';
    import type { Employee } from '$lib/types/employee';
    import {
        Camera,
        User,
        CheckCircle,
        AlertCircle,
        RefreshCw,
    } from 'lucide-svelte';
    import PageSection from '../../../../../components/PageSection.svelte';

    let videoEl: HTMLVideoElement;
    let canvasEl: HTMLCanvasElement;
    let employees: Employee[] = [];
    let selectedEmployeeId: string = '';
    let filteredEmployees: Employee[] = [];
    let search = '';
    let isLoading = true;
    let isCameraReady = false;
    let detectionStatus = 'Initializing...';
    let detectedDescriptor: Float32Array | undefined;

    let faceService = FaceRecognitionService.getInstance();
    let stream: MediaStream | null = null;
    let interval: any;

    onMount(async () => {
        try {
            const res = await listEmployees({
                pageSize: 1000,
                status: 'active',
            });
            employees = res.employees;
            filteredEmployees = employees;

            await faceService.loadModels();
            detectionStatus = 'Models Loaded. Starting Camera...';
            await startCamera();
        } catch (e) {
            detectionStatus = 'Error initializing: ' + e;
        } finally {
            isLoading = false;
        }
    });

    onDestroy(() => {
        stopCamera();
        if (interval) clearInterval(interval);
    });

    $: filteredEmployees = employees.filter((e) =>
        (e.firstName + ' ' + e.lastName + e.employeeId)
            .toLowerCase()
            .includes(search.toLowerCase()),
    );

    async function startCamera() {
        try {
            stream = await navigator.mediaDevices.getUserMedia({ video: {} });
            videoEl.srcObject = stream;
            isCameraReady = true;
            detectionStatus = 'Camera Active. Select an employee to register.';
            startDetectionLoop();
        } catch (e) {
            detectionStatus = 'Camera access denied or failed.';
            console.error(e);
        }
    }

    function stopCamera() {
        if (stream) {
            stream.getTracks().forEach((t) => t.stop());
            stream = null;
        }
    }

    function startDetectionLoop() {
        interval = setInterval(async () => {
            if (
                !videoEl ||
                videoEl.paused ||
                videoEl.ended ||
                !faceService.isLoaded
            )
                return;

            // Detect face
            const descriptor = await faceService.getFaceDescriptor(videoEl);
            if (descriptor) {
                detectedDescriptor = descriptor;
                detectionStatus = 'Face Detected! Ready to Capture.';
            } else {
                detectedDescriptor = undefined;
                detectionStatus = 'No face detected. Look closer.';
            }
        }, 500);
    }

    async function handleRegister() {
        if (!detectedDescriptor || !selectedEmployeeId) return;

        try {
            detectionStatus = 'Saving Face ID...';
            // We use the Employee UUID (id) not the string code (employeeId)
            const emp = employees.find((e) => e.id === selectedEmployeeId); // employees returned by API have 'id' as UUID??
            // wait, listEmployees returns Employee objects. Let's verify 'id' vs 'employeeId'.
            // Helper: Employee type has `id` (UUID) and `employeeId` (String Code).

            if (emp) {
                await updateFaceDescriptor(emp.id, detectedDescriptor);
                alert(`Face ID registered for ${emp.firstName}!`);
                detectionStatus = 'Registration Success!';
            }
        } catch (e) {
            console.error(e);
            alert('Failed to register face.');
            detectionStatus = 'Registration Failed.';
        }
    }
</script>

<PageSection title="Face Registration (Admin)">
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <!-- Sidebar: Employee List -->
        <div class="card bg-base-100 shadow-xl max-h-[600px] flex flex-col">
            <div class="p-4 border-b">
                <input
                    type="text"
                    placeholder="Search Employee..."
                    class="input input-bordered w-full"
                    bind:value={search} />
            </div>
            <div class="overflow-y-auto flex-1 p-2 space-y-2">
                {#each filteredEmployees as emp}
                    <button
                        class="w-full text-left p-3 rounded-lg flex items-center gap-3 transition-colors {selectedEmployeeId ===
                        emp.id
                            ? 'bg-primary text-primary-content'
                            : 'hover:bg-base-200'}"
                        on:click={() => (selectedEmployeeId = emp.id)}>
                        <User size={20} />
                        <div>
                            <div class="font-bold">
                                {emp.firstName}
                                {emp.lastName}
                            </div>
                            <div class="text-xs opacity-70">
                                {emp.employeeId} - {emp.status}
                            </div>
                        </div>
                    </button>
                {/each}
            </div>
        </div>

        <!-- Main Area: Camera -->
        <div class="md:col-span-2 space-y-4">
            <div
                class="card bg-black text-white shadow-xl overflow-hidden relative min-h-[400px]">
                <video
                    bind:this={videoEl}
                    autoplay
                    muted
                    playsinline
                    class="w-full h-full object-cover"></video>
                <div
                    class="absolute bottom-0 left-0 right-0 p-4 bg-black/50 backdrop-blur-sm flex justify-between items-center">
                    <div class="flex items-center gap-2">
                        {#if detectedDescriptor}
                            <CheckCircle class="text-success" />
                        {:else}
                            <AlertCircle class="text-warning" />
                        {/if}
                        <span>{detectionStatus}</span>
                    </div>
                </div>
            </div>

            <div class="card bg-base-100 shadow-xl p-6">
                <h3 class="font-bold text-lg mb-4">Registration Controls</h3>

                <div class="flex items-center justify-between">
                    <div>
                        <div class="text-sm opacity-70">Selected Employee</div>
                        <div class="text-xl font-bold">
                            {#if selectedEmployeeId}
                                {employees.find(
                                    (e) => e.id === selectedEmployeeId,
                                )?.firstName}
                                {employees.find(
                                    (e) => e.id === selectedEmployeeId,
                                )?.lastName}
                            {:else}
                                None Selected
                            {/if}
                        </div>
                    </div>

                    <button
                        class="btn btn-primary btn-lg gap-2"
                        disabled={!selectedEmployeeId || !detectedDescriptor}
                        on:click={handleRegister}>
                        <Camera />
                        Register Face
                    </button>
                </div>

                <p class="mt-4 text-sm text-base-content/60">
                    Instructions: Select an employee from the list. Ask them to
                    look at the camera. Ensure good lighting. When "Face
                    Detected" appears, click Register.
                </p>
            </div>
        </div>
    </div>
</PageSection>
