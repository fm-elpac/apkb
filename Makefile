# build apkb (for fdroid)

# (fdroid) set gradlew
ifeq ($(BUILD),fdroid)
BIN_GRADLE := gradle
else
BIN_GRADLE := ./gradlew
endif

# fdroid build
all: azi-aar atool pnpm-setup apkb-setup-zip apkb-apk
.PHONY: all

# 构建: atool (aarch64-linux-android)
atool:
	export PATH=$$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$$PATH && \
	export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android28-clang && \
	export AR_aarch64_linux_android=llvm-ar && \
	export CC_aarch64_linux_android=aarch64-linux-android28-clang && \
	cd atool && \
	cargo build --target aarch64-linux-android --release
.PHONY: atool

# pnpm install
pnpm-setup:
	cd ui-vue && pnpm install
.PHONY: pnpm-setup

# 构建: apbk-ui
apkb-ui:
	cd ui-vue && pnpm build
.PHONY: apkb-ui

# 使用 zip 生成 apkb-setup.azi.zip
apkb-setup-zip: apkb-ui
	- rm -r apk/setup/ui/
	mkdir -p apk/setup/ui/
	cp -r ui-vue/dist/* apk/setup/ui/
	cp atool/target/aarch64-linux-android/release/atool apk/setup/
	cp LICENSE apk/setup/
	cp README.md apk/setup/

	- rm apkb-setup.azi.zip
	cd apk/setup && zip -r ../../apkb-setup.azi.zip *

	mkdir -p apk/app/src/main/assets/
	cp apkb-setup.azi.zip apk/app/src/main/assets/
	cp LICENSE apk/app/src/main/assets/
	cp README.md apk/app/src/main/assets/
.PHONY: apkb-setup-zip

# 构建: apkb.apk
apkb-apk:
	cd apk && $(BIN_GRADLE) assemble
.PHONY: apkb-apk

# TODO
# 构建 (依赖): azi.aar
azi-aar:
	echo TODO
.PHONY: azi-aar
