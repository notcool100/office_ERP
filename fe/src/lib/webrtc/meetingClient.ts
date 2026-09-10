import { writable, get, type Writable } from 'svelte/store';
import { buildWsUrl } from '../services/api';
import { meetingService } from '../services/meeting';

// Public STUN only — no TURN server is configured on the backend. Peers
// behind symmetric NATs / strict corporate firewalls may fail to connect
// directly; that's a known limitation of a mesh-with-STUN-only setup, not a
// bug in this client.
const ICE_SERVERS: RTCIceServer[] = [{ urls: 'stun:stun.l.google.com:19302' }];

export interface RemoteParticipant {
    userId: string;
    name: string;
    stream: MediaStream | null;
    micOn: boolean;
    cameraOn: boolean;
}

export interface RosterEntry {
    userId: string;
    name: string;
    role: 'host' | 'participant';
}

export interface ChatMessage {
    text: string;
    fromUserId: string;
    fromName: string;
    sentAt: string;
}

export type ConnectionState = 'idle' | 'connecting' | 'connected' | 'disconnected' | 'error';

export interface MeetingClientState {
    localStream: MediaStream | null;
    micOn: boolean;
    cameraOn: boolean;
    isScreenSharing: boolean;
    isRecording: boolean;
    connectionState: ConnectionState;
    // WebRTC mesh tiles, keyed by user_id.
    participants: Record<string, RemoteParticipant>;
    // "Who's in the room" roster (from participant_joined/left), independent
    // of the WebRTC peer-connection map.
    roster: Record<string, RosterEntry>;
    chatMessages: ChatMessage[];
    meetingEnded: boolean;
    error: string | null;
}

interface RecordingSession {
    canvas: HTMLCanvasElement;
    ctx: CanvasRenderingContext2D;
    rafId: number;
    videoEls: Map<string, HTMLVideoElement>;
    audioCtx: AudioContext;
    audioSources: MediaStreamAudioSourceNode[];
    mediaRecorder: MediaRecorder;
    chunks: Blob[];
}

function initialState(): MeetingClientState {
    return {
        localStream: null,
        micOn: true,
        cameraOn: true,
        isScreenSharing: false,
        isRecording: false,
        connectionState: 'idle',
        participants: {},
        roster: {},
        chatMessages: [],
        meetingEnded: false,
        error: null
    };
}

/**
 * Plain TypeScript WebRTC mesh client for the Meetings feature. Exposes a
 * Svelte store (`store`) for reactive UI consumption — instantiate this
 * from a `.svelte` component's script and subscribe with `$client.store`.
 */
export class MeetingClient {
    readonly store: Writable<MeetingClientState> = writable(initialState());

    private meetingId: string;
    private myUserId: string;
    private myName: string;

    private ws: WebSocket | null = null;
    private pcs: Map<string, RTCPeerConnection> = new Map();
    private pendingCandidates: Map<string, RTCIceCandidateInit[]> = new Map();

    private cameraStream: MediaStream | null = null;
    private screenStream: MediaStream | null = null;

    private recording: RecordingSession | null = null;

    constructor(meetingId: string, myUserId: string, myName: string) {
        this.meetingId = meetingId;
        this.myUserId = myUserId;
        this.myName = myName;
    }

    private get state(): MeetingClientState {
        return get(this.store);
    }

    private patch(partial: Partial<MeetingClientState>) {
        this.store.update((s) => ({ ...s, ...partial }));
    }

    private upsertParticipant(userId: string, partial: Partial<RemoteParticipant>) {
        this.store.update((s) => {
            const existing = s.participants[userId] ?? {
                userId,
                name: s.roster[userId]?.name || 'Participant',
                stream: null,
                micOn: true,
                cameraOn: true
            };
            return {
                ...s,
                participants: {
                    ...s.participants,
                    [userId]: { ...existing, ...partial }
                }
            };
        });
    }

    private removeParticipant(userId: string) {
        this.store.update((s) => {
            const next = { ...s.participants };
            delete next[userId];
            return { ...s, participants: next };
        });
    }

    // ---- Join / connect -------------------------------------------------

    async join(): Promise<void> {
        this.patch({ connectionState: 'connecting', error: null });
        try {
            this.cameraStream = await navigator.mediaDevices.getUserMedia({ video: true, audio: true });
            this.patch({ localStream: this.cameraStream, micOn: true, cameraOn: true });

            const activeParticipants = await meetingService.joinMeeting(this.meetingId);
            const roster: Record<string, RosterEntry> = {};
            for (const p of activeParticipants) {
                if (p.user_id === this.myUserId) continue;
                roster[p.user_id] = { userId: p.user_id, name: p.display_name, role: p.role };
            }
            this.patch({ roster });

            await this.connectSocket();
            this.patch({ connectionState: 'connected' });
        } catch (e: any) {
            console.error('Failed to join meeting:', e);
            this.patch({ connectionState: 'error', error: e?.message || 'Failed to join meeting' });
            throw e;
        }
    }

    private connectSocket(): Promise<void> {
        return new Promise((resolve, reject) => {
            const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
            const url = `${buildWsUrl(`/ws/meetings/${this.meetingId}`)}?token=${token}`;
            const ws = new WebSocket(url);
            this.ws = ws;

            let settled = false;
            ws.onopen = () => {
                settled = true;
                resolve();
            };
            ws.onmessage = (event) => this.handleMessage(event);
            ws.onclose = () => {
                this.patch({ connectionState: 'disconnected' });
            };
            ws.onerror = (err) => {
                console.error('Meeting WS error:', err);
                if (!settled) {
                    settled = true;
                    reject(err);
                }
            };
        });
    }

    private sendRaw(obj: Record<string, any>) {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(obj));
        }
    }

    // ---- Inbound message routing ----------------------------------------

    private handleMessage(event: MessageEvent) {
        let data: any;
        try {
            data = JSON.parse(event.data);
        } catch {
            return;
        }

        switch (data.message_type) {
            case 'signal':
                this.handleSignal(data.payload);
                break;
            case 'peer_online':
                this.handlePeerOnline(data.payload);
                break;
            case 'peer_offline':
                this.handlePeerOffline(data.payload);
                break;
            case 'participant_joined':
                this.handleParticipantJoined(data.payload);
                break;
            case 'participant_left':
                this.handleParticipantLeft(data.payload);
                break;
            case 'meeting_ended':
                this.handleMeetingEnded();
                break;
        }
    }

    private handleSignal(payload: any) {
        if (!payload || payload.from_user_id === this.myUserId) return;
        if (payload.to_user_id && payload.to_user_id !== this.myUserId) return;

        if (payload.kind === 'meeting_chat') {
            this.store.update((s) => ({
                ...s,
                chatMessages: [
                    ...s.chatMessages,
                    {
                        text: payload.text,
                        fromUserId: payload.from_user_id,
                        fromName: payload.from_name || s.roster[payload.from_user_id]?.name || 'Unknown',
                        sentAt: payload.sent_at || new Date().toISOString()
                    }
                ]
            }));
            return;
        }

        if (payload.kind === 'media-state') {
            this.upsertParticipant(payload.from_user_id, {
                micOn: !!payload.micOn,
                cameraOn: !!payload.cameraOn
            });
            return;
        }

        switch (payload.kind) {
            case 'offer':
                this.handleOffer(payload).catch((e) => console.error('Failed to handle offer:', e));
                break;
            case 'answer':
                this.handleAnswer(payload).catch((e) => console.error('Failed to handle answer:', e));
                break;
            case 'ice-candidate':
                this.handleIceCandidate(payload).catch((e) =>
                    console.error('Failed to handle ICE candidate:', e)
                );
                break;
        }
    }

    private handlePeerOnline(payload: { user_id: string }) {
        if (!payload || payload.user_id === this.myUserId) return;
        // Only already-connected peers initiate toward a newcomer, avoiding
        // signaling glare — the newcomer never self-initiates.
        this.initiateOfferTo(payload.user_id).catch((e) =>
            console.error('Failed to initiate offer:', e)
        );
        this.sendMediaState();
    }

    private handlePeerOffline(payload: { user_id: string }) {
        if (!payload) return;
        const pc = this.pcs.get(payload.user_id);
        if (pc) {
            pc.close();
            this.pcs.delete(payload.user_id);
        }
        this.pendingCandidates.delete(payload.user_id);
        this.removeParticipant(payload.user_id);
    }

    private handleParticipantJoined(payload: {
        user_id: string;
        display_name: string;
        role: 'host' | 'participant';
    }) {
        if (!payload) return;
        this.store.update((s) => ({
            ...s,
            roster: {
                ...s.roster,
                [payload.user_id]: {
                    userId: payload.user_id,
                    name: payload.display_name,
                    role: payload.role
                }
            }
        }));
        if (payload.user_id !== this.myUserId && this.state.participants[payload.user_id]) {
            this.upsertParticipant(payload.user_id, { name: payload.display_name });
        }
    }

    private handleParticipantLeft(payload: { user_id: string }) {
        if (!payload) return;
        this.store.update((s) => {
            const next = { ...s.roster };
            delete next[payload.user_id];
            return { ...s, roster: next };
        });
    }

    private handleMeetingEnded() {
        this.patch({ meetingEnded: true });
    }

    // ---- WebRTC plumbing --------------------------------------------------

    private ensurePeerConnection(userId: string): RTCPeerConnection {
        let pc = this.pcs.get(userId);
        if (pc) return pc;

        pc = new RTCPeerConnection({ iceServers: ICE_SERVERS });
        this.pcs.set(userId, pc);

        const videoTrack = this.screenStream?.getVideoTracks()[0] ?? this.cameraStream?.getVideoTracks()[0];
        const audioTrack = this.cameraStream?.getAudioTracks()[0];
        if (videoTrack) pc.addTrack(videoTrack);
        if (audioTrack) pc.addTrack(audioTrack);

        pc.onicecandidate = (event) => {
            if (event.candidate) {
                this.sendRaw({
                    kind: 'ice-candidate',
                    to_user_id: userId,
                    candidate: event.candidate.toJSON()
                });
            }
        };

        pc.ontrack = (event) => {
            const existing = this.state.participants[userId]?.stream;
            const stream = existing ?? new MediaStream();
            if (!stream.getTracks().includes(event.track)) {
                stream.addTrack(event.track);
            }
            this.upsertParticipant(userId, { stream });
        };

        pc.onconnectionstatechange = () => {
            if (pc && (pc.connectionState === 'failed' || pc.connectionState === 'closed')) {
                this.pcs.delete(userId);
            }
        };

        this.upsertParticipant(userId, {});
        return pc;
    }

    private async initiateOfferTo(userId: string) {
        const pc = this.ensurePeerConnection(userId);
        const offer = await pc.createOffer();
        await pc.setLocalDescription(offer);
        this.sendRaw({ kind: 'offer', to_user_id: userId, sdp: offer });
    }

    private async handleOffer(payload: { from_user_id: string; sdp: RTCSessionDescriptionInit }) {
        const pc = this.ensurePeerConnection(payload.from_user_id);
        await pc.setRemoteDescription(payload.sdp);
        await this.flushPendingCandidates(payload.from_user_id);
        const answer = await pc.createAnswer();
        await pc.setLocalDescription(answer);
        this.sendRaw({ kind: 'answer', to_user_id: payload.from_user_id, sdp: answer });
        this.sendMediaState();
    }

    private async handleAnswer(payload: { from_user_id: string; sdp: RTCSessionDescriptionInit }) {
        const pc = this.pcs.get(payload.from_user_id);
        if (!pc) return;
        await pc.setRemoteDescription(payload.sdp);
        await this.flushPendingCandidates(payload.from_user_id);
    }

    private async handleIceCandidate(payload: { from_user_id: string; candidate: RTCIceCandidateInit }) {
        const pc = this.pcs.get(payload.from_user_id);
        if (!pc || !payload.candidate) return;
        if (!pc.remoteDescription) {
            const queue = this.pendingCandidates.get(payload.from_user_id) ?? [];
            queue.push(payload.candidate);
            this.pendingCandidates.set(payload.from_user_id, queue);
            return;
        }
        await pc.addIceCandidate(payload.candidate);
    }

    private async flushPendingCandidates(userId: string) {
        const pc = this.pcs.get(userId);
        const queue = this.pendingCandidates.get(userId);
        if (!pc || !queue) return;
        for (const candidate of queue) {
            try {
                await pc.addIceCandidate(candidate);
            } catch (e) {
                console.warn('Failed to add queued ICE candidate:', e);
            }
        }
        this.pendingCandidates.delete(userId);
    }

    private sendMediaState() {
        this.sendRaw({
            kind: 'media-state',
            micOn: this.state.micOn,
            cameraOn: this.state.cameraOn
        });
    }

    // ---- Local media controls ---------------------------------------------

    toggleMic() {
        if (!this.cameraStream) return;
        const next = !this.state.micOn;
        this.cameraStream.getAudioTracks().forEach((t) => (t.enabled = next));
        this.patch({ micOn: next });
        this.sendMediaState();
    }

    toggleCamera() {
        if (!this.cameraStream) return;
        const next = !this.state.cameraOn;
        this.cameraStream.getVideoTracks().forEach((t) => (t.enabled = next));
        this.patch({ cameraOn: next });
        this.sendMediaState();
    }

    async startScreenShare(): Promise<void> {
        if (this.state.isScreenSharing) return;
        const screenStream = await navigator.mediaDevices.getDisplayMedia({ video: true });
        this.screenStream = screenStream;
        const screenTrack = screenStream.getVideoTracks()[0];
        screenTrack.onended = () => {
            this.stopScreenShare().catch((e) => console.error('Failed to stop screen share:', e));
        };

        for (const pc of this.pcs.values()) {
            const sender = pc.getSenders().find((s) => s.track && s.track.kind === 'video');
            if (sender) await sender.replaceTrack(screenTrack);
        }

        const displayStream = new MediaStream([
            screenTrack,
            ...(this.cameraStream?.getAudioTracks() ?? [])
        ]);
        this.patch({ isScreenSharing: true, localStream: displayStream });
    }

    async stopScreenShare(): Promise<void> {
        if (!this.state.isScreenSharing || !this.screenStream) return;
        this.screenStream.getTracks().forEach((t) => t.stop());
        this.screenStream = null;

        const camTrack = this.cameraStream?.getVideoTracks()[0];
        for (const pc of this.pcs.values()) {
            const sender = pc.getSenders().find((s) => s.track && s.track.kind === 'video');
            if (sender && camTrack) await sender.replaceTrack(camTrack);
        }

        this.patch({ isScreenSharing: false, localStream: this.cameraStream });
    }

    // ---- In-meeting chat ----------------------------------------------------

    sendChatMessage(text: string) {
        const trimmed = text.trim();
        if (!trimmed) return;
        const sentAt = new Date().toISOString();
        this.sendRaw({ kind: 'meeting_chat', text: trimmed, from_name: this.myName, sent_at: sentAt });
        // Show our own message immediately — the server won't echo it back
        // to us (we filter out our own from_user_id on receipt).
        this.store.update((s) => ({
            ...s,
            chatMessages: [
                ...s.chatMessages,
                { text: trimmed, fromUserId: this.myUserId, fromName: this.myName, sentAt }
            ]
        }));
    }

    // ---- Local recording (client-side only, never touches the server) -----

    async startRecording(): Promise<void> {
        if (this.recording) return;

        const canvas = document.createElement('canvas');
        canvas.width = 1280;
        canvas.height = 720;
        const ctx = canvas.getContext('2d');
        if (!ctx) throw new Error('Canvas 2D context unavailable');

        const videoEls = new Map<string, HTMLVideoElement>();
        const makeVideoEl = (stream: MediaStream) => {
            const v = document.createElement('video');
            v.srcObject = stream;
            v.muted = true;
            v.playsInline = true;
            v.play().catch(() => {});
            return v;
        };

        if (this.state.localStream) videoEls.set('local', makeVideoEl(this.state.localStream));
        for (const p of Object.values(this.state.participants)) {
            if (p.stream) videoEls.set(p.userId, makeVideoEl(p.stream));
        }

        const audioCtx = new AudioContext();
        const dest = audioCtx.createMediaStreamDestination();
        const audioSources: MediaStreamAudioSourceNode[] = [];
        const connectAudio = (stream: MediaStream | null) => {
            if (!stream || stream.getAudioTracks().length === 0) return;
            const src = audioCtx.createMediaStreamSource(new MediaStream(stream.getAudioTracks()));
            src.connect(dest);
            audioSources.push(src);
        };
        connectAudio(this.cameraStream);
        for (const p of Object.values(this.state.participants)) connectAudio(p.stream);

        const draw = () => {
            const session = this.recording;
            if (!session) return;
            ctx.fillStyle = '#111';
            ctx.fillRect(0, 0, canvas.width, canvas.height);

            const entries = Array.from(videoEls.entries());
            const count = Math.max(entries.length, 1);
            const cols = Math.ceil(Math.sqrt(count));
            const rows = Math.ceil(count / cols);
            const cellW = canvas.width / cols;
            const cellH = canvas.height / rows;

            entries.forEach(([, v], i) => {
                const x = (i % cols) * cellW;
                const y = Math.floor(i / cols) * cellH;
                if (v.readyState >= 2) {
                    ctx.drawImage(v, x, y, cellW, cellH);
                }
            });

            session.rafId = requestAnimationFrame(draw);
        };

        const canvasStream = (canvas as HTMLCanvasElement & { captureStream: (fps?: number) => MediaStream }).captureStream(30);
        const mixedAudioTrack = dest.stream.getAudioTracks()[0];
        const combined = new MediaStream([
            ...canvasStream.getVideoTracks(),
            ...(mixedAudioTrack ? [mixedAudioTrack] : [])
        ]);

        const mimeType = MediaRecorder.isTypeSupported('video/webm;codecs=vp9,opus')
            ? 'video/webm;codecs=vp9,opus'
            : MediaRecorder.isTypeSupported('video/webm;codecs=vp8,opus')
              ? 'video/webm;codecs=vp8,opus'
              : 'video/webm';

        const mediaRecorder = new MediaRecorder(combined, { mimeType });
        const chunks: Blob[] = [];
        mediaRecorder.ondataavailable = (e) => {
            if (e.data.size > 0) chunks.push(e.data);
        };
        mediaRecorder.onstop = () => {
            const blob = new Blob(chunks, { type: mimeType });
            const fileName = `meeting-recording-${Date.now()}.webm`;

            // Always offer the local download first — it's instant and free,
            // and doesn't depend on the upload succeeding.
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = fileName;
            document.body.appendChild(a);
            a.click();
            a.remove();
            setTimeout(() => URL.revokeObjectURL(url), 1000);

            // Additionally upload the recording so it shows up in the
            // meeting's Attachments list for every participant. A failed
            // upload (e.g. network issue) must never break the local
            // download the user already has.
            meetingService
                .uploadAttachment(this.meetingId, new File([blob], fileName, { type: mimeType }), 'recording')
                .catch((e) => console.error('Failed to upload recording to meeting attachments:', e));
        };

        mediaRecorder.start(1000);

        this.recording = {
            canvas,
            ctx,
            rafId: 0,
            videoEls,
            audioCtx,
            audioSources,
            mediaRecorder,
            chunks
        };
        this.recording.rafId = requestAnimationFrame(draw);
        this.patch({ isRecording: true });
    }

    stopRecording() {
        const session = this.recording;
        if (!session) return;
        cancelAnimationFrame(session.rafId);
        if (session.mediaRecorder.state !== 'inactive') {
            session.mediaRecorder.stop();
        }
        session.videoEls.forEach((v) => {
            v.pause();
            v.srcObject = null;
        });
        session.audioSources.forEach((s) => s.disconnect());
        session.audioCtx.close().catch(() => {});
        this.recording = null;
        this.patch({ isRecording: false });
    }

    // ---- Teardown -----------------------------------------------------------

    async leave(): Promise<void> {
        this.stopRecording();

        if (this.state.isScreenSharing) {
            await this.stopScreenShare().catch(() => {});
        }

        for (const pc of this.pcs.values()) pc.close();
        this.pcs.clear();
        this.pendingCandidates.clear();

        if (this.cameraStream) {
            this.cameraStream.getTracks().forEach((t) => t.stop());
            this.cameraStream = null;
        }

        if (this.ws) {
            this.ws.onclose = null;
            this.ws.onmessage = null;
            this.ws.onerror = null;
            this.ws.close();
            this.ws = null;
        }

        try {
            await meetingService.leaveMeeting(this.meetingId);
        } catch (e) {
            console.warn('leaveMeeting request failed (meeting may have already ended):', e);
        }

        this.patch({ localStream: null, participants: {}, connectionState: 'disconnected' });
    }
}
