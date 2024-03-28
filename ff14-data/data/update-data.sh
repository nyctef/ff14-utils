#/bin/bash

set -euo pipefail

root=$(dirname $0)

# cache data from xivapi to avoid unnecessary load
# https://xivapi.com/docs/Game-Data

curl 'https://xivapi.com/Leve?pretty=1&limit=2000&columns=ID,Name,GilReward,ClassJobLevel,CraftLeve.Item0.Name,CraftLeve.Item0.ID,CraftLeve.ItemCount0' -o "$root/Leve.json"

curl 'https://xivapi.com/search?pretty=1&filters=Bonuses.CP.Max%3E0&columns=ID,Name,Bonuses,ItemUICategory.Name' -o "$root/CraftingFoodItems.json"