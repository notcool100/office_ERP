<script lang="ts">
    import { MessageCircle } from 'lucide-svelte';
</script>

<div
    class="flex-1 flex flex-col items-center justify-center text-base-content/50 p-8 text-center">
    <div class="bg-base-300 p-6 rounded-full mb-4">
        <MessageCircle class="w-12 h-12" />
    </div>
    <h1 class="text-xl font-bold text-base-content">Direct Messages</h1>
    <p class="max-w-xs mt-2">
        Select a direct message from the sidebar to start conversing privately
        with your team members.
    </p>
</div>
