reset

set term png
set output "output.png"

set xlabel "Occupation probability p"
set ylabel "Burn time t_b(p)"

set logscale y

set xrange [0.1:1.0]

plot for [n=5:10] sprintf("results/n%d.dat", 2**n) u 1:2 with linespoints t sprintf("N = %d", 2**n)