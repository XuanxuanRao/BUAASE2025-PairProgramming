@echo off
rmdir /S /Q ..\t3-snake-1-Rust\pkg
rmdir /S /Q ..\t3-snake-2-Rust\pkg
rmdir /S /Q ..\t3-snake-3-Rust\pkg
rmdir /S /Q ..\t3-snake-4-Rust\pkg
cargo build
wasm-pack build --target nodejs
xcopy /E /I pkg ..\t3-snake-1-Rust\pkg
xcopy /E /I pkg ..\t3-snake-2-Rust\pkg
xcopy /E /I pkg ..\t3-snake-3-Rust\pkg
xcopy /E /I pkg ..\t3-snake-4-Rust\pkg
npm run submit-test