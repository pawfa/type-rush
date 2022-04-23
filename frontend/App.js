import React, {useState} from "react";
import * as wasm from "../pkg";

wasm.run(`const test = 1 + 2;
const variable = test + 4;

function add(first: number, second: number) {
    const result = first + second;
    return result;
}

add(3,2)`);

const placeholder = `const test = 1 + 2;
const variable = test + 4;

function add(first: number, second: number) {
    const result = first + second;
    return result;
}

add(3,2)`

export function App(){
    const [code, setCode] = useState(placeholder)

    const handleTextAreaChange = (event) => {
        console.log(event.target.value)
        setCode(event.target.value)
    }

    const handleCompileButtonClick = () => {
        console.log(code);
        wasm.run(code);
    }

    return <div>
        <textarea onChange={handleTextAreaChange} cols={40} rows={15} defaultValue={placeholder}/>
        <button onClick={handleCompileButtonClick}>Compile</button>
    </div>
}