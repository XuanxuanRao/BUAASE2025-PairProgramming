@echo off
rmdir /S /Q ..\t3-snake-1-dqr\pkg
cargo build
wasm-pack build --target nodejs
xcopy /E /I pkg ..\t3-snake-1-dqr\pkg
npm run submit-test