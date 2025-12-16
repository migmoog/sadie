DEBUG_EXE=target/debug/sadie
SOURCE_FILES=$(wildcard src/*.c)

debug: $(DEBUG_EXE)
	rust-gdb $^ -tui

$(DEBUG_EXE): clean $(SOURCE_FILES)
	cargo build

.PHONY: clean
clean:
	cargo clean
