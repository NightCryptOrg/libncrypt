# Installation vars
PREFIX ?= /usr
DATADIR ?= $(PREFIX)/share

# Build targets
ncrypt=target/release/libncrypt.a
ncrypt_so=target/release/libncrypt.so

debug:
	cargo build
.PHONY: debug

release:
	cargo build --release
.PHONY: release

install: $(ncrypt) $(ncrypt_so) include/ncrypt.h README.md LICENSE
	@# Object files
	install -d $(DESTDIR)$(PREFIX)/lib
	install $(ncrypt) $(DESTDIR)$(PREFIX)/lib/libncrypt.a
	install $(ncrypt_so) $(DESTDIR)$(PREFIX)/lib/libncrypt.so
	@# Headers
	install -d $(DESTDIR)$(PREFIX)/include
	install -m644 include/ncrypt.h $(DESTDIR)$(PREFIX)/include/ncrypt.h
	@# Docs + license
	install -d $(DESTDIR)$(DATADIR)/doc/ncrypt
	install -m644 README.md $(DESTDIR)$(DATADIR)/doc/ncrypt/README.md
	install -d $(DESTDIR)$(DATADIR)/licenses/ncrypt
	install -m644 LICENSE $(DESTDIR)$(DATADIR)/licenses/ncrypt/LICENSE
.PHONY: install

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/lib/libncrypt.a
	rm -f $(DESTDIR)$(PREFIX)/lib/libncrypt.so
	rm -f $(DESTDIR)$(PREFIX)/include/include/ncrypt.h
	rm -rf $(DESTDIR)$(DATADIR)/doc/ncrypt
	rm -rf $(DESTDIR)$(DATADIR)/licenses/ncrypt
.PHONY: uninstall

clean:
	cargo clean
	rm -f $(ncrypt) libncrypt.so
.PHONY: clean
