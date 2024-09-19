.PHONY: run

run:
	 nix develop .. --command bash -c "nix-shell cuda-shell.nix --command \"nix run github:nix-community/nixGL#nixGLIntel -- poetry run python3 main.py\""

infer:
	nix run github:nix-community/nixGL#nixGLIntel -- yolo predict model=yolov8m-seg.pt source=example-foreground.png
