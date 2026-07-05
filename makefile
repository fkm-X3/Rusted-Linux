# Root Makefile
KERNEL_SRC   = ./kernel-src
DISTRO_DIR   = ./distro
OUT_DIR      = ./build
SYSROOT      = $(OUT_DIR)/rootfs
SYSTEMD_DIR  = ./systemd

# WSL build support (for building Linux binaries from Windows)
WSL_DISTRO  ?= Ubuntu-24.04

.PHONY: all kernel userspace image vt-install vt-uninstall clean \
        wsl wsl-kernel wsl-userspace wsl-image wsl-vt-install wsl-clean

all: image

# Prepare and compile the kernel
kernel:
	@echo "Binding distro config to Linux 7.1 source..."
	cp $(DISTRO_DIR)/kernel-config $(KERNEL_SRC)/.config
	$(MAKE) -C $(KERNEL_SRC) olddefconfig
	$(MAKE) -C $(KERNEL_SRC) -j$$(nproc) bzImage

# Build the Rust userspace (VT core, tools, and manager)
userspace:
	cd user-space && cargo build --release --target x86_64-unknown-linux-gnu

# Bind them together into a bootable image/ISO
image: kernel userspace
	mkdir -p $(OUT_DIR)
	./scripts/build_initramfs.sh $(OUT_DIR)

# Install Rust VT binaries into a sysroot
vt-install: userspace
	@echo "Installing Rust VT system to $(SYSROOT)..."
	install -d $(SYSROOT)/usr/bin
	install -d $(SYSROOT)/usr/lib/systemd/system

	# VT manager daemon
	install -m 755 user-space/target/x86_64-unknown-linux-gnu/release/rusted-vt-manager \
		$(SYSROOT)/usr/bin/rusted-vt-manager

	# Console setup tool
	install -m 755 user-space/target/x86_64-unknown-linux-gnu/release/rusted-vconsole-setup \
		$(SYSROOT)/usr/bin/rusted-vconsole-setup

	# VT CLI tools
	install -m 755 user-space/target/x86_64-unknown-linux-gnu/release/chvt \
		$(SYSROOT)/usr/bin/chvt
	install -m 755 user-space/target/x86_64-unknown-linux-gnu/release/fgconsole \
		$(SYSROOT)/usr/bin/fgconsole
	install -m 755 user-space/target/x86_64-unknown-linux-gnu/release/deallocvt \
		$(SYSROOT)/usr/bin/deallocvt
	install -m 755 user-space/target/x86_64-unknown-linux-gnu/release/openvt \
		$(SYSROOT)/usr/bin/openvt
	install -m 755 user-space/target/x86_64-unknown-linux-gnu/release/setfont \
		$(SYSROOT)/usr/bin/setfont
	install -m 755 user-space/target/x86_64-unknown-linux-gnu/release/loadkeys \
		$(SYSROOT)/usr/bin/loadkeys

	# Systemd units
	install -m 644 $(SYSTEMD_DIR)/rusted-console-setup.service \
		$(SYSROOT)/usr/lib/systemd/system/rusted-console-setup.service
	install -m 644 $(SYSTEMD_DIR)/rusted-vt-manager.service \
		$(SYSROOT)/usr/lib/systemd/system/rusted-vt-manager.service

	@echo "Rust VT system installed to $(SYSROOT)"

vt-uninstall:
	rm -f $(SYSROOT)/usr/bin/rusted-vt-manager
	rm -f $(SYSROOT)/usr/bin/rusted-vconsole-setup
	rm -f $(SYSROOT)/usr/bin/chvt
	rm -f $(SYSROOT)/usr/bin/fgconsole
	rm -f $(SYSROOT)/usr/bin/deallocvt
	rm -f $(SYSROOT)/usr/bin/openvt
	rm -f $(SYSROOT)/usr/bin/setfont
	rm -f $(SYSROOT)/usr/bin/loadkeys
	rm -f $(SYSROOT)/usr/lib/systemd/system/rusted-console-setup.service
	rm -f $(SYSROOT)/usr/lib/systemd/system/rusted-vt-manager.service

clean:
	cd user-space && cargo clean
	rm -rf $(OUT_DIR)


# WSL alt builds

wsl-deps:
	wsl.exe -d $(WSL_DISTRO) --cd "$(CURDIR)" bash scripts/install-deps.sh

wsl:
	wsl.exe -d $(WSL_DISTRO) --cd "$(CURDIR)" make

wsl-kernel:
	wsl.exe -d $(WSL_DISTRO) --cd "$(CURDIR)" make kernel

wsl-userspace:
	wsl.exe -d $(WSL_DISTRO) --cd "$(CURDIR)" make userspace

wsl-image:
	wsl.exe -d $(WSL_DISTRO) --cd "$(CURDIR)" make image

wsl-vt-install:
	wsl.exe -d $(WSL_DISTRO) --cd "$(CURDIR)" make vt-install

wsl-clean:
	wsl.exe -d $(WSL_DISTRO) --cd "$(CURDIR)" make clean