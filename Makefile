export DATA_PATH := $(shell pwd)/data

venv:
	python3 -m venv venv
	./venv/bin/python -m pip install --upgrade pip
	./venv/bin/python -m pip install -r requirements.txt

traffic: venv dataloader/src/lib.rs dataloader/src/go.rs dataloader/src/go.* *.go
	(cd dataloader; cargo build --no-default-features --features go)
	go build -ldflags="-r ./dataloader/target/debug" .

.PHONY: run test_py clean

test_py: venv
	(cd dataloader; ../venv/bin/maturin develop)
	./venv/bin/python ./dataloader/test.py

run: traffic
	./traffic

clean:
	rm -rf venv traffic dataloader/target .ipynb_checkpoints
