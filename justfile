echo:
    cargo build && ./extra/maelstrom/maelstrom/maelstrom test -w echo --bin ./target/debug/maelstrom-echo --node-count 1 --time-limit 10 --log-stderr

unique-ids:
    cargo build && ./extra/maelstrom/maelstrom/maelstrom test -w unique-ids --bin ./target/debug/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition --log-stderr


compile:
    cargo build --release && ./extra/maelstrom/maelstrom/maelstrom test -w echo --bin ./target/release/maelstrom-echo --node-count 1 --time-limit 10 --log-stderr

serve:
    ./extra/maelstrom/maelstrom/maelstrom serve