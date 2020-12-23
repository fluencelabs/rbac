(
cd provider
fce build
cp target/wasm32-wasi/debug/rbac_provider.wasm ../artifacts/
)

(
cd verifier
fce build
cp target/wasm32-wasi/debug/rbac_verifier.wasm ../artifacts/
)