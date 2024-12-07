test FILE:
    cargo test {{ FILE }}

run FILE:
    ./run.sh {{ FILE }}

runc FILE:
    ./run.sh -c {{ FILE }}
