clippy:
  cargo clippy -- -D warnings -A clippy::option-map-unit-fn -A clippy::module_inception -A clippy::single-match

build:
  cargo build
  cd fframes-editor && yarn rescript:build