#!/usr/bin/node

import { runProgram } from './common.mjs';

const initialState = () => ({ hPos: 0, depth: 0, aim: 0 });

const mutations = {
    forward: ({ hPos, depth, aim }, x) => ({ hPos: hPos + x, depth: depth + aim * x }),
    down: ({ aim }, x) => ({ aim: aim + x }),
    up: ({ aim }, x) => ({ aim: aim - x }),
};

runProgram(initialState(), mutations);
