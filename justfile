test FILE:
    cargo test {{ FILE }}

testv FILE:
    cargo test {{ FILE }} -- --nocapture

run FILE:
    ./run.sh {{ FILE }}

runc FILE:
    ./run.sh -c {{ FILE }}
