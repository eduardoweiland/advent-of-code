<?php

declare (strict_types=1);

namespace EW\AoC\Tests\Bingo;

use EW\AoC\Bingo\BingoGame;
use EW\AoC\Bingo\Board;
use EW\AoC\Bingo\NumberDrawer;
use PHPUnit\Framework\TestCase;

class BingoGameTest extends TestCase
{
    private NumberDrawer $numberDrawer;
    private Board $board1;
    private Board $board2;
    private BingoGame $bingoGame;

    protected function setUp(): void
    {
        $this->numberDrawer = $this->createMock(NumberDrawer::class);
        $this->board1 = $this->createMock(Board::class);
        $this->board2 = $this->createMock(Board::class);

        $boards = [$this->board1, $this->board2];
        $this->bingoGame = new BingoGame($this->numberDrawer, $boards);
    }

    public function testNewBingoGameDoesNotHaveWinner(): void
    {
        $this->assertFalse($this->bingoGame->hasWinner());
        $this->assertNull($this->bingoGame->getLastWinner());
    }

    public function testPlayRoundDrawsANumber(): void
    {
        $this->numberDrawer->expects($this->once())->method('draw');
        $this->bingoGame->playRound();
    }

    public function testPlayRoundMarksNumberOnAllBoardsAfterDraw(): void
    {
        $this->numberDrawer->method('draw')->will($this->returnValue(42));
        $this->board1->expects($this->once())->method('mark')->with($this->equalTo(42));
        $this->board2->expects($this->once())->method('mark')->with($this->equalTo(42));
        $this->bingoGame->playRound();
    }

    public function testGetDrawnNumbersAfterTwoRounds(): void
    {
        $this->numberDrawer->method('draw')->will($this->onConsecutiveCalls(42, 13));
        $this->bingoGame->playRound();
        $this->bingoGame->playRound();
        $this->assertEquals([42, 13], $this->bingoGame->getDrawnNumbers());
    }

    public function testBingoHasWinnerWhenSomeBoardIsWinner(): void
    {
        $this->board1->method('isWinner')->will($this->returnValue(false));
        $this->board2->method('isWinner')->will($this->returnValue(true));
        $this->bingoGame->playRound();
        $this->assertTrue($this->bingoGame->hasWinner());
    }

    public function testGetLastWinner(): void
    {
        $this->board1->method('isWinner')->will($this->returnValue(false));
        $this->board2->method('isWinner')->will($this->returnValue(true));
        $this->bingoGame->playRound();
        $this->assertSame($this->board2, $this->bingoGame->getLastWinner());
    }

    public function testGetLastWinnerAfterSecondBoardWins(): void
    {
        $this->board1->method('isWinner')->will($this->onConsecutiveCalls(true, true));
        $this->board2->method('isWinner')->will($this->onConsecutiveCalls(false, true));

        $this->bingoGame->playRound();
        $this->bingoGame->playRound();

        $this->assertSame($this->board2, $this->bingoGame->getLastWinner());
    }

    public function testAreThereBoardsStillPlayingOnNewGame(): void
    {
        $this->board1->method('isWinner')->will($this->returnValue(false));
        $this->board2->method('isWinner')->will($this->returnValue(false));
        $this->assertTrue($this->bingoGame->areThereBoardsStillPlaying());
    }

    public function testAreThereBoardsStillPlayingAfterAllBoardsWon(): void
    {
        $this->board1->method('isWinner')->will($this->returnValue(true));
        $this->board2->method('isWinner')->will($this->returnValue(true));
        $this->assertFalse($this->bingoGame->areThereBoardsStillPlaying());
    }
}
