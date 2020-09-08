ifeq ($(shell uname),Darwin)
# MacOS
    LDFLAGS := -Wl,-dead_strip
else
ifeq ($(shell uname -o),Msys)
# Windows (Msys)
    LDFLAGS := -ladvapi32 -lws2_32 -luserenv -lshell32 -lmsvcrt
    EXEC_PFX := PATH="${PATH}:target/debug"
else
# Linux or other Unix
    LDFLAGS := -Wl,--gc-sections -lpthread -ldl
endif
endif

CFLAGS := -I target/debug

all: target/double target/double_dyn
	target/double
	$(EXEC_PFX) target/double_dyn

target:
	mkdir -p $@

target/double: target/main.o target/debug/libdouble_input.a
	$(CC) -o $@ $^ $(LDFLAGS)

target/double_dyn: target/main.o target/debug/libdouble_input.so
	$(CC) -o $@ $^ $(LDFLAGS)

target/debug/libdouble_input.a: src/lib.rs Cargo.toml
	cargo build

target/debug/my_library.h:
	cargo build

target/debug/libdouble_input.so: target/debug/libdouble_input.a
	$(CC) -shared -o $@ -u free_string $^ $(LDFLAGS)

target/main.o: src/main.c target/debug/my_library.h | target
	$(CC) -o $@ -c $< $(CFLAGS)

clean:
	rm -rf target
