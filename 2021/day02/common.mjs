import readline from 'readline';

export async function runProgram(initialState, mutations) {
    let state = initialState;

    for await (const line of stdin()) {
        const [command, x] = parseLine(line);
        state = applyMutation(state, mutations[command], x);
    }

    showResult(state);
}

function stdin() {
    return readline.createInterface({ input: process.stdin });
}

function showResult({ hPos, depth }) {
    console.log(hPos * depth);
}

function parseLine(line) {
    const [mutation, x] = line.split(' ');
    return [mutation, Number.parseInt(x, 10)];
}

function applyMutation(state, mutation, ...args) {
    return Object.assign({}, state, mutation(state, ...args));
}
