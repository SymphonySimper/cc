set quiet := true

test FILE:
    cargo test {{ FILE }}

testv FILE:
    cargo test {{ FILE }} -- --nocapture

format:
  cargo fmt

run FILE:
    ./.scripts/run.sh {{ FILE }}

runc FILE:
    ./.scripts/run.sh -c {{ FILE }}

cp:
    cat ./template.txt | wl-copy

new PATH:
    ./.scripts/new.sh {{ PATH }}
