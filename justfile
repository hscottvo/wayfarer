develop issue:
    git fetch --all
    gh issue develop {{issue}} --checkout --base main

test crate *args:
    cargo nextest run -p wayfarer-{{crate}} {{args}}

