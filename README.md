# monoglu
A file storage with a version control feature and a CLI/GUI client.

for local tests:
```
cargo install basic-http-server
```

## Usage (Windows Only for Now)
1. Run the following commands on the prompt:
```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo update -p wasm-bindgen

./build.bat
```

2. Run localhost:8080 on the browser.
