$root = resolve-path "$PSScriptRoot/.."
$manifest = resolve-path "$root/Cargo.toml"

function ff14run() {
  cargo run --manifest-path $manifest --release --bin @args
}

function recipe() {
  if (-not (test-path $root/scratch/recipes.txt)) {
    ff14run recipes > $root/scratch/recipes.txt
  }

  $recipe = cat $root/scratch/recipes.txt | fzf
  if ($lastexitcode -ne 0) {
    return
  }
  $count = read-host -prompt "item count"
  ff14run specific-recipe -- $recipe $count
}


function mapcompare { ff14run map-compare }
function levecompare { ff14run leve-compare }
function recipecompare { ff14run recipe-compare @args}