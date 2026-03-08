.PHONY: build release install uninstall clean

# Default build
build:
	cargo build

# Binary name
BIN_NAME = aether
ifeq ($(OS),Windows_NT)
	BIN_NAME := aether.exe
endif

# Optimized release build
release:
	cargo build --release

# Install to user directory (no sudo needed)
install: release
	@echo "Installing Aether..."
	@mkdir -p $(HOME)/.local/bin 2>/dev/null || true
	@cp target/release/$(BIN_NAME) $(HOME)/.local/bin/$(BIN_NAME)
	@chmod +x $(HOME)/.local/bin/$(BIN_NAME) 2>/dev/null || true
	@echo "✅ Binary installed to ~/.local/bin/$(BIN_NAME)"
ifeq ($(OS),Windows_NT)
	@echo "Creating Windows Start Menu shortcut..."
	@powershell -Command "$$ws = New-Object -ComObject WScript.Shell; $$s = $$ws.CreateShortcut(\"$$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Aether.lnk\"); $$s.TargetPath = \"$$env:USERPROFILE\.local\bin\aether.exe\"; $$s.Save()"
	@echo "✅ Start Menu shortcut created"
else ifeq ($(shell uname),Darwin)
	@echo "Creating macOS App Bundle..."
	@mkdir -p $(HOME)/Applications/Aether.app/Contents/MacOS
	@mkdir -p $(HOME)/Applications/Aether.app/Contents/Resources
	@cp target/release/aether $(HOME)/Applications/Aether.app/Contents/MacOS/aether
	@cp "aether logos/aether_icon_256.png" $(HOME)/Applications/Aether.app/Contents/Resources/aether.png
	@cp assets/Info.plist $(HOME)/Applications/Aether.app/Contents/Info.plist
	@echo "✅ Aether.app created in ~/Applications"
else
	@echo "Installing Linux Desktop Entry..."
	@mkdir -p $(HOME)/.local/share/icons/hicolor/256x256/apps
	@cp "aether logos/aether_icon_256.png" $(HOME)/.local/share/icons/hicolor/256x256/apps/aether.png
	@mkdir -p $(HOME)/.local/share/applications
	@cp assets/aether.desktop $(HOME)/.local/share/applications/aether.desktop
	@chmod +x $(HOME)/.local/share/applications/aether.desktop
	@echo "✅ Desktop entry installed to ~/.local/share/applications/aether.desktop"
endif
	@echo ""
	@echo "Run 'aether' to get started!"

# Uninstall
uninstall:
	@rm -f $(HOME)/.local/bin/$(BIN_NAME)
ifeq ($(OS),Windows_NT)
	@rm -f "$(APPDATA)/Microsoft/Windows/Start Menu/Programs/Aether.lnk"
else ifeq ($(shell uname),Darwin)
	@rm -rf $(HOME)/Applications/Aether.app
else
	@rm -f $(HOME)/.local/share/applications/aether.desktop
	@rm -f $(HOME)/.local/share/icons/hicolor/256x256/apps/aether.png
endif
	@echo "✅ Aether uninstalled"

# Clean build artifacts
clean:
	cargo clean

# Run in debug mode
run:
	cargo run

# Run with args
run-file:
	cargo run -- $(FILE)

# Run setup
run-setup:
	cargo run -- --setup
