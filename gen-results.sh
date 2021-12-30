#!/bin/zsh

cargo build --release

for n in {5..9}
do
    ./target/release/sim-5 $((2**$n)) >> "results/n$((2**$n))".dat
done

./target/release/sim-5 1024 -s 1000 >> "results/n1024".dat