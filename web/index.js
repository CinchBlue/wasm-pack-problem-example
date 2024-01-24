import { MyApi } from "learn_rust_wasm";

export async function debug_all() {
    console.log('----------------- debug_csb() -----------------');

    // console.log('----------------- debug_wasm() -----------------');
    // await debug_wasm();
}

export function getApi() {
    let api = MyApi.new("lol");
    debugger;
    return api;
}

let x = getApi();

window.api = x;
console.log(await window.api.get_name());
console.log(await window.api.do_stuff());