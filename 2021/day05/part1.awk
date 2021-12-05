#!/usr/bin/awk -f
function min(a, b) { return (a < b) ? a : b }
function max(a, b) { return (a > b) ? a : b }

BEGIN { FS="[ ,]" }

$1 == $4 {
  for (y = min($2, $5); y <= max($2, $5); y++)
    map[$1 "," y]++
}

$2 == $5 {
  for (x = min($1, $4); x <= max($1, $4); x++)
    map[x "," $2]++
}

END {
  for (key in map)
    if (map[key] >= 2)
      count++
  print count
}
