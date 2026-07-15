.PHONY: build docker-build deb-build deb-clean

BIN := clash-switcher
IMAGE := clash-verge-rev:2.5.1
FIMAGE := clash-verge-rev_v2.5.1.tar
FAKEROOT := package/root

build:
	cargo build --release

docker-build:
	cd docker/ && docker build -t $(IMAGE) .

deb-prepare: build docker-build
	mkdir -p $(FAKEROOT)/usr/bin
	mkdir -p $(FAKEROOT)/opt/$(BIN)
	cp target/release/$(BIN) $(FAKEROOT)/usr/bin
	cp docker/compose.yaml $(FAKEROOT)/opt/$(BIN)
	docker save -o $(FAKEROOT)/opt/$(BIN)/$(FIMAGE) $(IMAGE)

deb-build:
	cd package/ && fakeroot dpkg-deb --build .

deb-clean:
	rm $(FAKEROOT)/usr/bin/* $(FAKEROOT)/opt/$(BIN)/*
