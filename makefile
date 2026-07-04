# Root Makefile
KERNEL_SRC = ./kernel-src
DISTRO_DIR = ./distro
OUT_DIR    = ./build

.PHONY: all kernel userspace image

all: image

# Prepare and compile the kernel
kernel:
	@echo "Binding distro config to Linux 7.1 source..."
	# Copy Rusted-Linux config into the raw kernel source directory
	cp $(DISTRO_DIR)/kernel-config $(KERNEL_SRC)/.config
	# Apply any custom patches (comment because there are none yet)
	# cd $(KERNEL_SRC) && git reset --hard && git am ../$(DISTRO_DIR)/patches/*
	# Compile the kernel
	$(MAKE) -C $(KERNEL_SRC) -j$(shell nproc) bzImage

# Build the Rust userspace
userspace:
	cd user-space && cargo build --release --target x86_64-unknown-linux-gnu

# Bind them together into a bootable image/ISO
image: kernel userspace
	mkdir -p $(OUT_DIR)
	# Script/Tooling to combine bzImage and Rust binary into an initramfs
	./scripts/build_initramfs.sh $(OUT_DIR)