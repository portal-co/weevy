import brotliPromise from 'brotli-wasm'
const brotli = await brotliPromise;
import { decode } from '@mikeshardmind/base2048'
export function urlRewriter(base: string): (a: string) => string {
    return data => {
        let a = new XMLHttpRequest();
        a.open('GET', `${base}code=${btoa(data)}`, false);
        a.send();
        return a.responseText;
    }
}
const globalName = '__WeevyMain';
// const symMarkPrivate = Symbol.for("weevy private");
// const symSpecialStringify = Symbol.for("weevy string marker");
// const marSpecialStringify: WeakMap<any, () => string> = new WeakMap();
export class Host {
    static mappers: WeakMap<any, Host> = new WeakMap();
    #obj: any;
    stringify: () => string | undefined;
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
export class Guest {
    // #globalThis: typeof globalThis;
    #of_: WeakMap<any, any> = new WeakMap();
    #expose_: WeakMap<any, any> = new WeakMap();
    #rewriter: (a: string) => string;
    rewrite(a: string): string {
        return this.#rewriter(a)
    }
    of<T>(t: T): T {
        if (typeof t !== "object" && typeof t !== "function") {
            return t;
        }
        var a = this.#of_.get(t);
        if (a === undefined) {
            return t;
        }
        return this.of(a);
    }
    ofThis<T extends object>(this_: T, key: keyof T): T[typeof key] {
        let t = this.of(this_[key]);
        if (typeof t !== "object" && typeof t !== "function") {
            return t;
        }
        return new Proxy(t as any, {
            apply(target, thisArg, argArray) {
                thisArg = this_;
                return Reflect.apply(target, thisArg, argArray);
            },
        }) as T[typeof key];
    }
    #set<T extends object>(a: T, p: ProxyHandler<T>, create: boolean): T {
        let crate = (a) => new Proxy(Object.create(a), {
            getPrototypeOf(target) {
                return Reflect.getPrototypeOf(Reflect.getPrototypeOf(target)!);
            },
        });
        let v: T = new Proxy(create ? crate(a) : a, p);
        // if (create) {
        //     v = Object.create(v);
        // }
        this.#of_.set(a, v);
        this.#expose_.set(v, a);
        return v;
    }
    #expose<T>(t: T): T {
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
    constructor(id: string, rewriter: (a: string) => string) {
        this.#rewriter = rewriter;
        let rewrite_prop = (p: string | symbol): string | symbol => {
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
                return Reflect.has(target, p)
            },
            get(target, p, receiver) {
                p = rewrite_prop(p);
                return Reflect.get(target, p, receiver);
            },
            deleteProperty(target, p) {
                p = rewrite_prop(p);
                return Reflect.deleteProperty(target, p)
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
                return Reflect.set(target, p, newValue, receiver)
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
interface GuestMap {
    [a: string]: Guest
}
export const WeevyMain = {
    // [symMarkPrivate]: { replaceWith: undefined },
    newSourceDecompressor(x) {
        const a = brotli.decompress(decode(x));
        let ress = {};
        return (r, v) => {
            const s = r.split(";");
            const [c, b] = [parseInt(s[0]), parseInt(s[1])];
            const x = new Uint8Array(a, c, b - c);
            // let res;
            // return v => {
            Host.of(v).stringify = () => (ress[r] || (ress[r] = { $: new TextDecoder().decode(x) })).$;
            return v;
            // }
        }
    },
    guests: {} as GuestMap,
    withProxy<T extends object>(a: T): T {
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
})