import { hook } from '@portal-solutions/hooker-core'
import { decompress } from 'brotli-compress/js.mjs'
import { decode } from '@mikeshardmind/base2048'
export class Host {
    static mappers: WeakMap<any, Host> = new WeakMap();
    #obj: any;
    stringify: (() => string) | undefined;
    constructor(obj: any) {
        this.#obj = obj;
    }
    static of(a: any): Host {
        while (true) {
            if (this.mappers.has(a)) {
                return this.mappers.get(a) as Host;
            }
            this.mappers.set(a, new Host(a));
        }
    }
}
export function newSourceDecompressor(x) {
    const a = decompress(decode(x));
    let ress = {};
    return (r, v) => {
        // v = this.wrap(v);
        const s = r.split(";");
        const [c, b] = [parseInt(s[0]), parseInt(s[1])];
        const x = new Uint8Array(a.buffer, c + a.byteOffset, b - c);
        // let res;
        // return v => {
        Host.of(v).stringify = () => (ress[r] || (ress[r] = { $: new TextDecoder().decode(x) })).$;
        return v;
        // }
    }
}
hook(Function.prototype, "toString", Reflect => ({
    apply(a, b, c) {
        var g = Host.of(b).stringify;
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
            Host.of(v).stringify = Host.of(argArray[0]).stringify;
        }
        return v;
    },
}));
