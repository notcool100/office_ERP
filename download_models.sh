#!/bin/bash
BASE_URL="https://raw.githubusercontent.com/justadudewhohacks/face-api.js/master/weights"
TARGET_DIR="fe/static/models"

mkdir -p $TARGET_DIR

files=(
    "ssd_mobilenet_v1_model-weights_manifest.json"
    "ssd_mobilenet_v1_model-shard1"
    "ssd_mobilenet_v1_model-shard2"
    "face_landmark_68_model-weights_manifest.json"
    "face_landmark_68_model-shard1"
    "face_recognition_model-weights_manifest.json"
    "face_recognition_model-shard1"
    "face_recognition_model-shard2"
)

for file in "${files[@]}"; do
    echo "Downloading $file..."
    curl -s -o "$TARGET_DIR/$file" "$BASE_URL/$file"
done

echo "Download complete."
