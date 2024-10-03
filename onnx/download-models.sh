#!/usr/bin/bash

model_down_onnx() {
  yolo export format=onnx model=$1-seg.pt int8=true simplify=true data=coco8-seg.yaml imgsz=640 nms=true batch=2
  mv $1-seg.onnx $1-seg-int8.onnx
  yolo export format=onnx model=$1-seg.pt half=true simplify=true imgsz=640 nms=true batch=2
  mv $1-seg.onnx $1-seg-half.onnx
  yolo export format=onnx model=$1-seg.pt simplify=true imgsz=640 nms=true batch=2
}

model_down_onnx yolov8m
model_down_onnx yolo11s
model_down_onnx yolo11m
