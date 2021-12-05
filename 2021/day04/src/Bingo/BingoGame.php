<?php

declare (strict_types=1);

namespace EW\AoC\Bingo;

use function Functional\each;
use function Functional\first;
use function Functional\some;

class BingoGame
{
    private array $drawnNumbers = [];

    /** @param Board[] $boards */
    public function __construct(private NumberDrawer $numberDrawer, private array $boards)
    {
    }

    public function hasWinner(): bool
    {
        return some($this->boards, fn ($board) => $board->isWinner());
    }

    public function getWinner(): ?Board
    {
        if (!$this->hasWinner()) {
            return null;
        }

        return first($this->boards, fn ($board) => $board->isWinner());
    }

    public function playRound(): void
    {
        if (!$this->hasWinner()) {
            $number = $this->numberDrawer->draw();
            $this->drawnNumbers[] = $number;
            each($this->boards, fn ($board) => $board->mark($number));
        }
    }

    public function getDrawnNumbers(): array
    {
        return $this->drawnNumbers;
    }
}
