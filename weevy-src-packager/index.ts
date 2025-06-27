import { hook } from '@portal-solutions/hooker-core'
import { decompress } from 'brotli-compress/js.mjs'
import { decode } from '@mikeshardmind/base2048'
export let _Uint8Array: typeof Uint8Array = Uint8Array;
export let Host: WeakMap<Function,() => string> = new WeakMap();
let theDecoder = new TextDecoder();
export function newSourceDecompressor(x) {
    const a = decompress(decode(x));
    let ress = {};
    return (r, v) => {
        // v = this.wrap(v);
        const s = r.split(";");
        const [c, b] = [parseInt(s[0]), parseInt(s[1])];
        const x = new _Uint8Array(a.buffer, c + a.byteOffset, b - c);
        // let res;
        // return v => {
        Host.set(v,() => (ress[r] || (ress[r] = { $: theDecoder.decode(x) })).$);
        return v;
        // }
    }
}
hook(Function.prototype, "toString", Reflect => ({
    apply(a, b, c) {
        var g = Host.get(b);
        if (g !== undefined) {
            return g();
        }
        return Reflect.apply(a, b, c);
    }
}));
hook(globalThis, "Proxy", Reflect => ({
    construct(target, argArray, newTarget) {
        var v = Reflect.construct(target, argArray, newTarget);
        if (argArray.length > 0) {
            var g = Host.get(argArray[0]);
            if(g !== undefined)Host.set(v,g);
        }
        return v;
    },
}));
export let function_name = Reflect.getOwnPropertyDescriptor(Function.prototype, 'name')!.get!.call.bind(Reflect.getOwnPropertyDescriptor(Function.prototype, 'name')!.get!);
export function native<T extends Function>(a: T): T {
    Host.set(a, () => `function ${function_name(a)}{ [native code] }`);
    return a;
}