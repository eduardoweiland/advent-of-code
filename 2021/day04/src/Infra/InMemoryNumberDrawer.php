<?php

declare (strict_types=1);

namespace EW\AoC\Infra;

use EW\AoC\Bingo\NumberDrawer;
use UnderflowException;
use function array_shift;
use function count;
use function intval;

class InMemoryNumberDrawer implements NumberDrawer
{
    public function __construct(private array $numbers)
    {
    }

    public function draw(): int
    {
        if (count($this->numbers) === 0) {
            throw new UnderflowException('No more numbers left to draw');
        }

        return intval(array_shift($this->numbers));
    }
}
