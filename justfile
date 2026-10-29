set default-list := true
develop issue:
    git fetch --all
    gh issue develop {{issue}} --checkout --base main

test *args:
    cargo nextest run --workspace {{args}}

test-crate crate *args:
    cargo nextest run -p {{crate}} {{args}}

build *args:
    cargo build {{args}}
