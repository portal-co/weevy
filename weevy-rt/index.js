import brotliPromise from 'brotli-wasm';
const brotli = await brotliPromise;
import { decode } from '@mikeshardmind/base2048';
export function urlRewriter(base) {
    return data => {
        let a = new XMLHttpRequest();
        a.open('GET', `${base}code=${btoa(data)}`, false);
        a.send();
        return a.responseText;
    };
}
const globalName = '__WeevyMain';
// const symMarkPrivate = Symbol.for("weevy private");
// const symSpecialStringify = Symbol.for("weevy string marker");
// const marSpecialStringify: WeakMap<any, () => string> = new WeakMap();
export class Host {
    static mappers = new WeakMap();
    #obj;
    stringify;
    constructor(obj) {
        this.#obj = obj;
    }
    static of(a) {
        while (true) {
            if (this.mappers.has(a)) {
                return this.mappers.get(a);
            }
            this.mappers.set(a, new Host(a));
        }
    }
}
export class Guest {
    // #globalThis: typeof globalThis;
    #of_ = new WeakMap();
    #expose_ = new WeakMap();
    #rewriter;
    rewrite(a) {
        return this.#rewriter(a);
    }
    of(t) {
        if (typeof t !== "object" && typeof t !== "function") {
            return t;
        }
        var a = this.#of_.get(t);
        if (a === undefined) {
            return t;
        }
        return this.of(a);
    }
    ofThis(this_, key) {
        let t = this.of(this_[key]);
        if (typeof t !== "object" && typeof t !== "function") {
            return t;
        }
        return new Proxy(t, {
            apply(target, thisArg, argArray) {
                thisArg = this_;
                return Reflect.apply(target, thisArg, argArray);
            },
        });
    }
    #set(a, p, create) {
        let crate = (a) => new Proxy(Object.create(a), {
            getPrototypeOf(target) {
                return Reflect.getPrototypeOf(Reflect.getPrototypeOf(target));
            },
        });
        let v = new Proxy(create ? crate(a) : a, p);
        // if (create) {
        //     v = Object.create(v);
        // }
        this.#of_.set(a, v);
        this.#expose_.set(v, a);
        return v;
    }
    #expose(t) {
        if (typeof t !== "object" && typeof t !== "function") {
            return t;
        }
        var a = this.#expose_.get(t);
        if (a === undefined) {
            return t;
        }
        return this.#expose(a);
    }
    get globalThis() {
        return this.of(globalThis);
    }
    constructor(id, rewriter) {
        this.#rewriter = rewriter;
        let rewrite_prop = (p) => {
            if (typeof p === 'string') {
                if (p.startsWith(globalName)) {
                    return p + '_';
                }
            }
            return p;
        };
        // let set = (a, p) => ;
        this.#set(globalThis, {
            has(target, p) {
                p = rewrite_prop(p);
                return Reflect.has(target, p);
            },
            get(target, p, receiver) {
                p = rewrite_prop(p);
                return Reflect.get(target, p, receiver);
            },
            deleteProperty(target, p) {
                p = rewrite_prop(p);
                return Reflect.deleteProperty(target, p);
            },
            defineProperty(target, property, attributes) {
                property = rewrite_prop(property);
                return Reflect.defineProperty(target, property, attributes);
            },
            getOwnPropertyDescriptor(target, p) {
                p = rewrite_prop(p);
                return Reflect.getOwnPropertyDescriptor(target, p);
            },
            set(target, p, newValue, receiver) {
                p = rewrite_prop(p);
                return Reflect.set(target, p, newValue, receiver);
            },
            ownKeys(target) {
                return Reflect.ownKeys(target).filter(x => x !== globalName).map(p => {
                    if (typeof p === 'string') {
                        if (p.startsWith(globalName) && p.endsWith("_")) {
                            return p.slice(0, -1);
                        }
                    }
                    return p;
                });
            },
        }, true);
    }
}
export const WeevyMain = {
    // [symMarkPrivate]: { replaceWith: undefined },
    newSourceDecompressor(x) {
        const a = brotli.decompress(decode(x));
        let ress = {};
        return (r, v) => {
            const s = r.split(";");
            const [c, b] = [parseInt(s[0]), parseInt(s[1])];
            const x = new Uint8Array(a.buffer, c + a.byteOffset, b - c);
            // let res;
            // return v => {
            Host.of(v).stringify = () => (ress[r] || (ress[r] = { $: new TextDecoder().decode(x) })).$;
            return v;
            // }
        };
    },
    guests: {},
    withProxy(a) {
        return new Proxy(a, {
            has(target, a) {
                return a !== globalName && Reflect.has(target, a);
            }
        });
    }
};
Object.defineProperty(globalThis, globalName, {
    value: WeevyMain,
    configurable: false,
    enumerable: false,
    writable: false
});
Function.prototype.toString = new Proxy(Function.prototype.toString, {
    apply: (a, b, c) => {
        var g = Host.of(b).stringify;
        if (g !== undefined) {
            return g();
        }
        return Reflect.apply(a, b, c);
    }
});
globalThis.Proxy = new Proxy(globalThis.Proxy, {
    construct(target, argArray, newTarget) {
        var v = Reflect.construct(target, argArray, newTarget);
        if (argArray.length > 0) {
            Host.of(v).stringify = Host.of(argArray[0]).stringify;
        }
        return v;
    },
});
