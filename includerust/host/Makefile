RUST_TARGET ?= debug
RUST_BUILD_ARGS ?= 

host: main.c  libscaledown.so
	gcc -o host -g -DVERSION='"1.0"' -I../scaledown/target/include main.c -Wl,-rpath='$$ORIGIN' -L. -lscaledown

libscaledown.so: ../scaledown/target/$(RUST_TARGET)/libscaledown.so
	cp $< $@

../scaledown/target/$(RUST_TARGET)/libscaledown.so:
	cd ../scaledown/ && cargo build $(RUST_BUILD_ARGS)

clean:
	rm -rf libscaledown.so host
