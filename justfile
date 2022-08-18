clippy:
  cargo clippy -- -D warnings -A clippy::option-map-unit-fn -A clippy::module_inception -A clippy::single-match

build:
  cargo build
  cd fframes-editor && yarn rescript:build

init-repo:
  ffmpeg -version
  rustc --version
  yarn --version

  cargo build
  yarn install
  cd fframes-editor && yarn rescript:build && yarn bundle:dev

watch-editor:
  cd fframes-editor && yarn dev

run example:
  cd examples/{{example}}/editor && yarn dev

render example:
  cd examples/{{example}} && cargo run --release && just play {{example}}

play example:
  cd examples/{{example}} && ffplay out.mp4

 