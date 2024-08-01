run:
    cargo build && ./extra/maelstrom/maelstrom/maelstrom test -w echo --bin ./target/debug/maelstrom-echo --node-count 1 --time-limit 10 --log-stderr

compile:
    cargo build --release && ./extra/maelstrom/maelstrom/maelstrom test -w echo --bin ./target/release/maelstrom-echo --node-count 1 --time-limit 10 --log-stderr

serve:
    ./extra/maelstrom/maelstrom/maelstrom serve