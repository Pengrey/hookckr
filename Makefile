# Cargo commands
CARGO := cargo

# Logger
define log_info
	echo -e "[\033[0;33m*\033[0m] $(1)"
endef

define log_success
	echo -e "[\033[0;32m+\033[0m] Done"
endef

release:
	@ $(call log_info,Compiling...)
	@ $(CARGO) build --release --target x86_64-pc-windows-gnu
	@ $(call log_success)

clean:
	@ $(call log_info,Cleaning compile artifacts)
	@ rm -rf target
	@ $(call log_success)

.PHONY: release debug clean
