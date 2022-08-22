libwelfords.a : ./rust/src/lib.rs
	cd rust;\
	cargo build --release;
	cp ./rust/target/release/libwelfords.a .

welfords.so : libwelfords.a welfords.c
	PKG_LIBS=libwelfords.a R CMD SHLIB welfords.c

.PHONY: all
all : ./rust/src/lib.rs welfords.c
	make libwelfords.a
	make welfords.so
	make test

.PHONY: test
test : test.r welfords.so
	R -e 'source("test.r")'

.PHONY: clean
clean :
	rm *.a
	rm *.so
	rm *.o
