import { writable } from 'svelte/store';

interface MeetingStoreState {
    activeMeetingId: string | null;
}

function createMeetingStore() {
    const { subscribe, set } = writable<MeetingStoreState>({ activeMeetingId: null });

    return {
        subscribe,
        set: (meetingId: string) => set({ activeMeetingId: meetingId }),
        clear: () => set({ activeMeetingId: null })
    };
}

export const meetingStore = createMeetingStore();
