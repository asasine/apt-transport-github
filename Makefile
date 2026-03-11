METHODS_DIR := /usr/lib/apt/methods
LOG_METHODS := https http
LOG_SYMLINKS := $(addprefix $(METHODS_DIR)/log+,$(LOG_METHODS))

.PHONY: install

install: $(METHODS_DIR)/log $(LOG_SYMLINKS)

$(METHODS_DIR)/log: bin/apt-transport-log
	install -m 755 $< $@

$(LOG_SYMLINKS):
	ln -sf log $@
