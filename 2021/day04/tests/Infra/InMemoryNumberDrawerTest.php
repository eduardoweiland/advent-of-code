<?php

declare (strict_types=1);

namespace EW\AoC\Tests\Infra;

use EW\AoC\Infra\InMemoryNumberDrawer;
use PHPUnit\Framework\TestCase;
use UnderflowException;

class InMemoryNumberDrawerTest extends TestCase
{
    public function testItDrawsNumbersInOrder(): void
    {
        $drawer = new InMemoryNumberDrawer([42, 6, 17]);
        $this->assertEquals(42, $drawer->draw());
        $this->assertEquals(6, $drawer->draw());
        $this->assertEquals(17, $drawer->draw());
    }

    public function testDrawThrowsExceptionWhenNoNumbersAreLeft(): void
    {
        $this->expectException(UnderflowException::class);
        $this->expectExceptionMessage('No more numbers left to draw');
        $drawer = new InMemoryNumberDrawer([]);
        $drawer->draw();
    }
}