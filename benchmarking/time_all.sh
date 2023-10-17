#!/usr/bin/env bash
set -e

benchmarks=(
    "bench_modulegraph.py"
    "bench_pydeps.py"
    "bench_fastdep.py"
    "bench_python_read.py"
)

if [ -z "$1" ]; then
    echo "Error: Must provide module name to trace!"
    exit 1
fi

ITERS=3
if [ ! -z "$2"]; then
    ITERS=$2
fi

# For any methods which can't trace
./generate_file_list.py "$1"

for file in "${benchmarks[@]}"; do
    echo ""
    echo "Executing: $file"
    for ((i=1; i<=ITERS; i++)); do
        python "$file" "$1"
    done
done