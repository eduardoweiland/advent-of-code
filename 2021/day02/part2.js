#!/usr/bin/node

const initialState = () => ({ hPos: 0, depth: 0, aim: 0 });

const mutations = {
    forward: ({ hPos, depth, aim }, x) => ({ hPos: hPos + x, depth: depth + aim * x, aim: aim }),
    down:    ({ hPos, depth, aim }, x) => ({ hPos, depth, aim: aim + x }),
    up:      ({ hPos, depth, aim }, x) => ({ hPos, depth, aim: aim - x }),
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
