import * as wasm from "../pkg/index";

wasm.run(`const test = 1 + 2;
const variable = test + 4;

function add(first: number, second: number) {
    const result = first + second;
    return result;
}

add(3,2)`);
console.log('asdasdasd')
