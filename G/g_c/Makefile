CC = emcc
CFLAGS = -O3 -s WASM=1 -s MODULARIZE=1 -s EXPORTED_FUNCTIONS='["_func"]' -s EXPORTED_RUNTIME_METHODS='["cwrap"]'

build-c: clean
	$(CC) $(CFLAGS) -o module.cjs main.c

build-cpp: clean
	$(CC) $(CFLAGS) -o module.cjs main.cpp

test:
	cd .. && npm run submit-test

clean:
	rm -f module.cjs module.wasm
