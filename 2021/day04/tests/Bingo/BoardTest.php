<?php

declare (strict_types=1);

namespace EW\AoC\Tests\Bingo;

use EW\AoC\Bingo\Board;
use EW\AoC\Bingo\BoardException;
use PHPUnit\Framework\TestCase;

class BoardTest extends TestCase
{
    private Board $board;

    protected function setUp(): void
    {
        $this->board = new Board([
            [1, 2],
            [3, 4],
        ]);
    }

    public function testNewBoardIsNotWinner(): void
    {
        $this->assertFalse($this->board->isWinner());
    }

    /**
     * @testWith [[]]
     *           [[[]]]
     *           [[[], [], []]]
     */
    public function testEmptyBoardIsNotAllowed(array $boardDefinition): void
    {
        $this->expectException(BoardException::class);
        $this->expectExceptionMessage('Board must not be empty');
        new Board($boardDefinition);
    }

    public function testMarkNumber(): void
    {
        $this->board->mark(2);
        $this->assertEquals(1, $this->board->getNumberOfMarkings());
    }

    public function testMarkNonExistingNumberDoesNothing(): void
    {
        $this->board->mark(666);
        $this->assertEquals(0, $this->board->getNumberOfMarkings());
    }

    public function testAfterOneMarkIsNotWinner(): void
    {
        $this->board->mark(2);
        $this->assertFalse($this->board->isWinner());
    }

    public function testAfterMarkEntireRowIsWinner(): void
    {
        $this->board->mark(1);
        $this->board->mark(2);
        $this->assertTrue($this->board->isWinner());
    }

    public function testAfterMarkEntireColumnIsWinner(): void
    {
        $this->board->mark(1);
        $this->board->mark(3);
        $this->assertTrue($this->board->isWinner());
    }

    public function testGetUnmarkedNumbers(): void
    {
        $this->board->mark(1);
        $this->board->mark(4);
        $this->assertEquals([2, 3], $this->board->getUnmarkedNumbers());
    }
}
