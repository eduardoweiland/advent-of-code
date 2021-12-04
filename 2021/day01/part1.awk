#!/usr/bin/awk -f
NR > 1 && $1 > last { count++ }
{ last=$1 }
END { print count }
