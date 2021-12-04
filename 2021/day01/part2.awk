#!/usr/bin/awk -f
(NR - 1) % 3 == 0 { B+=$1; C+=$1; if (NR > 1 && B > A) count++; A=$1 }
(NR - 2) % 3 == 0 { A+=$1; C+=$1; if (NR > 2 && C > B) count++; B=$1 }
(NR - 3) % 3 == 0 { A+=$1; B+=$1; if (NR > 3 && A > C) count++; C=$1 }
END { print count }
