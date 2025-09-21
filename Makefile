.PHONY: debug release trace build mac win

debug:
	cargo run --features tracing-off

release:
	cargo run --release --features tracing-off

trace:
	cargo run --release --features tracy

build: win mac

mac:
	@if [ -z "$(v)" ]; then echo "Error: Version parameter is required. Use 'make mac v=x.y.z'"; exit 1; fi
	cargo bundle --target aarch64-apple-darwin --release --features tracing-off
	mkdir -p build/macos/v$(v)
	cp -r "target/aarch64-apple-darwin/release/bundle/osx/Dungeon Breeder.app" "build/macos/v$(v)/Dungeon Breeder v$(v).app"
	codesign --force --deep --sign "https://github.com/Zitronenjoghurt" "build/macos/v$(v)/Dungeon Breeder v$(v).app"
	cd build/macos/v$(v) && zip -r dungeon-breeder-v$(v)-mac.zip "Dungeon Breeder v$(v).app"
	@echo "MacOS app bundle created and signed"

win:
	@if [ -z "$(v)" ]; then echo "Error: Version parameter is required. Use 'make win v=x.y.z'"; exit 1; fi
	cargo build --target x86_64-pc-windows-gnu --release --features tracing-off
	mkdir -p build/windows/v$(v)
	cp target/x86_64-pc-windows-gnu/release/dungeon-breeder-app.exe "build/windows/v$(v)/Dungeon Breeder v$(v).exe"
	cd build/windows/v$(v) && zip -r dungeon-breeder-v$(v)-win.zip "Dungeon Breeder v$(v).exe"
	@echo "Windows executable built and zipped"