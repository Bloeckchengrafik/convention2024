```shell
pip install -U ultralytics

yolo export model=yolov8m-seg.pt format=onnx  simplify dynamic
```

```shell
nix run github:nix-community/nixGL#nixGLIntel --  /home/chris/.local/bin/yolo export model=yolov8m-seg.pt format=onnx  simplify dynamic
```