# Installation vars
PREFIX ?= /usr
DATADIR ?= $(PREFIX)/share

lib-files=libncrypt.a libncrypt.so

debug:
	cargo build
.PHONY: debug

release:
	cargo build --release
	cp target/$@/libncrypt.so target/$@/libncrypt.a ./
.PHONY: release

install: $(lib-files) README.md LICENSE
	install -d $(DESTDIR)$(PREFIX)/lib
	install $(lib-files) $(DESTDIR)$(PREFIX)/lib/
	install -d $(DESTDIR)$(DATADIR)/doc/ncrypt
	install -m644 README.md $(DESTDIR)$(DATADIR)/doc/ncrypt/
	install -d $(DESTDIR)$(DATADIR)/licenses/ncrypt
	install -m644 LICENSE $(DESTDIR)$(DATADIR)/licenses/ncrypt/LICENSE
.PHONY: install

clean:
	cargo clean
	rm -f $(lib-files)
.PHONY: clean
