<?php

declare (strict_types=1);

namespace EW\AoC\Bingo;

use function Functional\contains;
use function Functional\each;
use function Functional\last;
use function Functional\some;

class BingoGame
{
    private array $drawnNumbers = [];
    private array $allWinners = [];

    /** @param Board[] $boards */
    public function __construct(private NumberDrawer $numberDrawer, private array $boards)
    {
    }

    public function hasWinner(): bool
    {
        return some($this->boards, fn ($board) => $board->isWinner());
    }

    public function getLastWinner(): ?Board
    {
        return last($this->allWinners);
    }

    public function playRound(): void
    {
        $number = $this->numberDrawer->draw();
        $this->drawnNumbers[] = $number;
        each($this->boards, fn ($board) => $board->mark($number));
        $this->checkWinners();
    }

    public function getDrawnNumbers(): array
    {
        return $this->drawnNumbers;
    }

    public function areThereBoardsStillPlaying(): bool
    {
        return some($this->boards, fn ($board) => !$board->isWinner());
    }

    private function checkWinners(): void
    {
        foreach ($this->boards as $board) {
            if ($board->isWinner() && !contains($this->allWinners, $board, strict: true)) {
                $this->allWinners[] = $board;
            }
        }
    }
}
