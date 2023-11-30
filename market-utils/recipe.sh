#/bin/bash

set -euo pipefail

recipe=$(cargo run --quiet --release --bin list-recipes | fzf)
cargo run --quiet --release --bin specific-recipe -- "$recipe" "1"