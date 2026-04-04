METHODS_DIR := /usr/lib/apt/methods
LOG_METHODS := https http github
LOG_SYMLINKS := $(addprefix $(METHODS_DIR)/log+,$(LOG_METHODS))

.PHONY: install

install: $(METHODS_DIR)/log $(METHODS_DIR)/github $(LOG_SYMLINKS)

$(METHODS_DIR)/log: bin/apt-transport-log
	install -m 755 $< $@

# must `cargo build --release` beforehand
$(METHODS_DIR)/github: target/release/apt-transport-github
	install -m 755 $< $@

$(LOG_SYMLINKS):
	ln -sf log $@
