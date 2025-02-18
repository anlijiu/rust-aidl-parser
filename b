# wasm-pack build --target=web
export CC=gcc
export CXX=g++
export CXXFLAGS=" -fexceptions"
# export CXXFLAGS='--sysroot=/home/an/workspace/wasm/wasi-sysroot-25.0 -DRUST_CXX_NO_EXCEPTIONS'
wasm-pack build --target=web --out-dir=webpkg
wasm-pack build --target=nodejs  --out-dir=nodepkg
