CFLAGS = -std=c11 -Wall -Wextra -O0 -ggdb3

all: silly

silly: src/silly.c
	mkdir -p bin
	$(CC) $(CFLAGS) -o bin/$@ $<

run:
	LD_PRELOAD=./target/debug/libmemstats.so bin/silly

clean:
	@rm -rf bin
