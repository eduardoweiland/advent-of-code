<?php

$positions = explode(',', file_get_contents('php://stdin'));
$avg = round(array_sum($positions) / count($positions));
$getCostAt = fn ($targetPos) => array_sum(
    array_map(
        function ($startPos) use ($targetPos) {
            $distance = abs($startPos - $targetPos);
            return (($distance + 1) * $distance) / 2;
        },
        $positions,
    ),
);

$bestCost = $getCostAt($avg);
$distanceToAvg = 0;

do {
    $distanceToAvg++;
    $costLeft = $getCostAt($avg - $distanceToAvg);
    $costRight = $getCostAt($avg + $distanceToAvg);
    $bestCost = min($bestCost, $costLeft, $costRight);
} while ($costLeft <= $bestCost || $costRight <= $bestCost);

echo $bestCost . PHP_EOL;
