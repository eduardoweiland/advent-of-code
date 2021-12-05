<?php

declare (strict_types=1);

namespace EW\AoC\Tests\Infra;

use EW\AoC\Infra\InputReader;
use PHPUnit\Framework\TestCase;

class InputReaderTest extends TestCase
{
    public function testReadNumbersToDrawReturnsEmptyArrayWithEmptyStream(): void
    {
        $reader = $this->buildInputReaderWithData('');
        $numbersToDraw = $reader->readNumbersToDraw();

        $this->assertIsArray($numbersToDraw);
        $this->assertCount(0, $numbersToDraw);
    }

    public function testReadOneNumberToDraw(): void
    {
        $reader = $this->buildInputReaderWithData('11');
        $numbersToDraw = $reader->readNumbersToDraw();

        $this->assertIsArray($numbersToDraw);
        $this->assertCount(1, $numbersToDraw);
        $this->assertEquals(11, $numbersToDraw[0]);
    }

    public function testReadThreeNumbersToDraw(): void
    {
        $reader = $this->buildInputReaderWithData('11,42,69');
        $numbersToDraw = $reader->readNumbersToDraw();

        $this->assertIsArray($numbersToDraw);
        $this->assertCount(3, $numbersToDraw);
        $this->assertEquals([11, 42, 69], $numbersToDraw);
    }

    public function testReadBoardNumbersStopOnABlankLine(): void
    {
        $reader = $this->buildInputReaderWithData("1 2 3\n4 5 6\n\n7 8 9\n");
        $boardNumbers = $reader->readBoardNumbers();

        $this->assertIsArray($boardNumbers);
        $this->assertCount(2, $boardNumbers);
    }

    public function testReadBoardNumbersConvertsValuesToInt(): void
    {
        $reader = $this->buildInputReaderWithData("1 2 3\n");
        $boardNumbers = $reader->readBoardNumbers();

        $this->assertIsArray($boardNumbers);
        $this->assertIsArray($boardNumbers[0]);
        $this->assertEquals([1, 2, 3], $boardNumbers[0]);
    }

    public function testReadBoardNumbersIgnoresMultipleSpacesBetweenColumns(): void
    {
        $reader = $this->buildInputReaderWithData("   1   2   3   \n");
        $boardNumbers = $reader->readBoardNumbers();

        $this->assertIsArray($boardNumbers);
        $this->assertIsArray($boardNumbers[0]);
        $this->assertEquals([1, 2, 3], $boardNumbers[0]);
    }

    public function testReadAllBoardNumbers(): void
    {
        $reader = $this->buildInputReaderWithData("1 2\n3 4\n\n5 6\n7 8\n");

        $allBoardNumbers = $reader->readAllBoardNumbers();

        $this->assertIsArray($allBoardNumbers);
        $this->assertEquals([[1, 2], [3, 4]], $allBoardNumbers[0]);
        $this->assertEquals([[5, 6], [7, 8]], $allBoardNumbers[1]);
    }

    private function buildInputReaderWithData(string $data)
    {
        $stream = fopen('php://memory', 'r+');
        fwrite($stream, $data);
        rewind($stream);
        return new InputReader($stream);
    }
}
