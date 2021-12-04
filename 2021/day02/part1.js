#!/usr/bin/node

const initialState = () => ({ hPos: 0, depth: 0 });

const mutations = {
    forward: ({ hPos, depth }, x) => ({ hPos: hPos + x, depth }),
    down:    ({ hPos, depth }, x) => ({ hPos, depth: depth + x }),
    up:      ({ hPos, depth }, x) => ({ hPos, depth: depth - x }),
};

let state = initialState();

const apply = (mutation, ...args) => {
    state = mutations[mutation](state, ...args);
};

require('readline')
    .createInterface({ input: process.stdin })
    .on('close', () => console.log(state.hPos * state.depth))
    .on('line', (line) => {
        const [mutation, x] = line.split(' ');
        apply(mutation, Number.parseInt(x, 10));
    });
