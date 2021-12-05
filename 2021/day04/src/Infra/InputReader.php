<?php

declare (strict_types=1);

namespace EW\AoC\Infra;

use function explode;
use function fgets;
use function intval;
use function Functional\map;

class InputReader
{
    public function __construct(private $stream)
    {
    }

    public function readNumbersToDraw(): array
    {
        $line = fgets($this->stream);

        if ($line === false) {
            return [];
        }

        return $this->convertStringToArrayOfInts($line, separator: ',');
    }

    public function readEmptyLine(): void
    {
        fgets($this->stream);
    }

    public function readBoardNumbers(): array
    {
        $rows = [];

        for (;;) {
            $line = fgets($this->stream);

            if ($line === false || trim($line) === '') {
                break;
            }

            $rows[] = $this->convertStringToArrayOfInts($line, separator: ' ');
        }

        return $rows;
    }

    public function readAllBoardNumbers(): array
    {
        $boards = [];

        for (;;) {
            $board = $this->readBoardNumbers();

            if (count($board) === 0) {
                break;
            }

            $boards[] = $board;
        }

        return $boards;
    }

    private function convertStringToArrayOfInts(string $line, string $separator): array
    {
        return map(
            preg_split("/$separator/", trim($line), -1, PREG_SPLIT_NO_EMPTY),
            fn ($number) => intval($number),
        );
    }
}
