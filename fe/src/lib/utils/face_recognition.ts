import * as faceapi from 'face-api.js';

export class FaceRecognitionService {
    private static instance: FaceRecognitionService;
    public isLoaded = false;

    private constructor() { }

    public static getInstance(): FaceRecognitionService {
        if (!FaceRecognitionService.instance) {
            FaceRecognitionService.instance = new FaceRecognitionService();
        }
        return FaceRecognitionService.instance;
    }

    public async loadModels() {
        if (this.isLoaded) return;

        const MODEL_URL = '/models';
        try {
            await Promise.all([
                faceapi.nets.ssdMobilenetv1.loadFromUri(MODEL_URL),
                faceapi.nets.faceLandmark68Net.loadFromUri(MODEL_URL),
                faceapi.nets.faceRecognitionNet.loadFromUri(MODEL_URL)
            ]);
            this.isLoaded = true;
            console.log("FaceAPI Models Loaded");
        } catch (e) {
            console.error("Failed to load FaceAPI models:", e);
            throw e;
        }
    }

    public async getFaceDescriptor(image: HTMLImageElement | HTMLVideoElement | HTMLCanvasElement): Promise<Float32Array | undefined> {
        if (!this.isLoaded) await this.loadModels();

        const detection = await faceapi.detectSingleFace(image)
            .withFaceLandmarks()
            .withFaceDescriptor();

        return detection?.descriptor;
    }

    public async getAllFaces(image: HTMLImageElement | HTMLVideoElement): Promise<faceapi.WithFaceDescriptor<faceapi.WithFaceLandmarks<{ detection: faceapi.FaceDetection }>>[]> {
        if (!this.isLoaded) await this.loadModels();

        return await faceapi.detectAllFaces(image)
            .withFaceLandmarks()
            .withFaceDescriptors();
    }

    public createMatcher(labeledDescriptors: faceapi.LabeledFaceDescriptors[]): faceapi.FaceMatcher {
        return new faceapi.FaceMatcher(labeledDescriptors, 0.6);
    }
}
