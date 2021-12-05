#!/usr/bin/awk -f
function min(a, b) { return (a < b) ? a : b }
function max(a, b) { return (a > b) ? a : b }
function between(x, a, b) { return (x >= min(a, b)) && (x <= max(a, b)) }

BEGIN { FS="[ ,]" }

$1 == $4 {
  for (y = min($2, $5); y <= max($2, $5); y++)
    map[$1 "," y]++
}

$2 == $5 {
  for (x = min($1, $4); x <= max($1, $4); x++)
    map[x "," $2]++
}

$1 != $4 && $2 != $5 {
  incx = $1 < $4 ? +1 : -1
  incy = $2 < $5 ? +1 : -1

  x = $1
  y = $2

  do {
    map[x "," y]++
    x += incx
    y += incy
  } while (between(x, $1, $4) && between(y, $2, $5))
}

END {
  for (key in map)
    if (map[key] >= 2)
      count++
  print count
}
