root_folder := './src/web-ui'

build:
  wasm-pack build \
    --target web \
    --out-dir "{{root_folder}}/pkg"

serve:
  static-web-server \
    --port 8787 \
    --root "{{root_folder}}"
