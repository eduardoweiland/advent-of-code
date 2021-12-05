<?php

declare (strict_types=1);

namespace EW\AoC\Bingo;

use function array_column;
use function array_fill;
use function array_map;
use function array_search;
use function array_sum;
use function count;
use function Functional\map;
use function Functional\some;
use function Functional\true;
use function max;
use function range;

class Board
{
    private int $numberOfMarkings = 0;
    private array $rowMarkings = [];
    private int $maxColumns = 0;

    public function __construct(private array $rows)
    {
        if (array_sum(array_map('count', $rows)) === 0) {
            throw new BoardException('Board must not be empty');
        }

        foreach ($rows as $cols) {
            $this->rowMarkings[] = array_fill(0, count($cols), false);
            $this->maxColumns = max($this->maxColumns, count($cols));
        }
    }

    public function isWinner(): bool
    {
        $isAllMarked = fn ($rowOrCol) => true($rowOrCol);
        $columnMarkings = map(range(0, $this->maxColumns - 1), fn ($col) => array_column($this->rowMarkings, $col));
        return some($this->rowMarkings, $isAllMarked) || some($columnMarkings, $isAllMarked);
    }

    public function mark(int $number): void
    {
        $index = $this->findIndexesOfNumber($number);

        if ($index !== null) {
            [$row, $col] = $index;
            $this->numberOfMarkings++;
            $this->rowMarkings[$row][$col] = true;
        }
    }

    public function getNumberOfMarkings(): int
    {
        return $this->numberOfMarkings;
    }

    public function getUnmarkedNumbers(): array
    {
        $numbers = [];

        foreach ($this->rows as $rowIndex => $cols) {
            foreach ($cols as $colIndex => $number) {
                if (!$this->rowMarkings[$rowIndex][$colIndex]) {
                    $numbers[] = $number;
                }
            }
        }

        return $numbers;
    }

    private function findIndexesOfNumber(int $number): ?array
    {
        foreach ($this->rows as $rowIndex => $row) {
            $colIndex = array_search($number, $row, true);

            if ($colIndex !== false) {
                return [$rowIndex, $colIndex];
            }
        }

        return null;
    }
}
