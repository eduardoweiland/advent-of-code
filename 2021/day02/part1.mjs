#!/usr/bin/node

import { runProgram } from './common.mjs';

const initialState = () => ({ hPos: 0, depth: 0 });

const mutations = {
    forward: ({ hPos },  x) => ({ hPos: hPos + x }),
    down: ({ depth }, x) => ({ depth: depth + x }),
    up: ({ depth }, x) => ({ depth: depth - x }),
};

runProgram(initialState(), mutations);
