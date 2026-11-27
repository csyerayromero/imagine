BINARY = imagine
INSTALL_DIR = /usr/local/bin
TARGET = target/release/$(BINARY)

.PHONY: all build install uninstall clean

all: build

build:
	cargo build --release

install: build
	sudo install -Dm755 $(TARGET) $(INSTALL_DIR)/$(BINARY)
	@echo "Installed $(BINARY) successfully!"

uninstall:
	sudo rm -f $(INSTALL_DIR)/$(BINARY)
	@echo "Uninstalled $(BINARY)."

clean:
	cargo clean

