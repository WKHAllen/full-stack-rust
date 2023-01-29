.PHONY: all build run clean

all: build

build:
	cd backend && cargo tauri build

run:
	cd backend && cargo tauri dev

clean:
	cargo clean
