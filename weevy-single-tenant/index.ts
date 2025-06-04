import { hook, _Proxy, _Reflect } from '@portal-solutions/hooker-core'

let push = Array.prototype.push.call.bind(Array.prototype.push);
let pop = Array.prototype.pop.call.bind(Array.prototype.pop);
let startsWith = String.prototype.startsWith.call.bind(String.prototype.startsWith);

import { decode } from '@mikeshardmind/base2048'
import { Host as _Host, newSourceDecompressor } from '@portal-solutions/weevy-src-packager';
import { symWeevyMain } from '@portal-solutions/weevy-common';
import { PropRewriter } from '../weevy-camo-wasm/pkg/weevy_camo_wasm_bg';

let hostCodeActive = true
export function hostCode<T>(f: () => T): T {
    let old = hostCodeActive;
    hostCodeActive = true;
    try {
        return f()
    } finally {
        hostCodeActive = old;
    }
}
export function guestCode<T>(f: () => T): T {
    let old = hostCodeActive;
    hostCodeActive = false
    try {
        return f()
    } finally {
        hostCodeActive = old;
    }
}
export let camo = new PropRewriter("TheTenant");
Object.defineProperty(globalThis, symWeevyMain, {
    enumerable: false,
    configurable: false,
    writable: false,
    value: Object.seal({
        newSourceDecompressor,
        camo,
    }),
});
let guestSyms: { [a: string]: symbol } = Object.create(null);
// @ts-expect-error
let guestSymsRev: WeakMap<symbol, string> = new WeakMap();
hook(Symbol, 'for', Reflect => ({
    apply(target, thisArg, argArray) {
        if (hostCodeActive) return Reflect.apply(target, thisArg, argArray);
        let v = argArray[0];
        let s = guestSyms[v] ??= Symbol(v);
        guestSymsRev.set(s, v);
        return s;
    },
}));
hook(Symbol, 'keyFor', Reflect => ({
    apply(target, thisArg, argArray) {
        if (hostCodeActive) return Reflect.apply(target, thisArg, argArray);

        return guestSymsRev.get(argArray[0])
    },
}));
export let loc: URL = new URL(location.href);
export function setLoc(n: URL) {
    loc = n;
    dispatchEvent(Object.assign(new Event('locationchange'), {
        newLoc: loc,
    }))
}
for (var obj of [globalThis, globalThis?.document]) {
    if (!obj) {
        continue;
    }
    Object.defineProperty(obj, camo.rewrite("location") as string, {
        get() {
            return new Proxy(Object.assign(loc, {
                assign(a) {
                    setLoc(new URL(a));
                },
                reload() {

                },
                replace(a) {
                    this.assign(a);
                }
            }), {
                getPrototypeOf(target) {
                    return _Reflect.getPrototypeOf(location);
                },
                set(target, p, newValue, receiver) {
                    let a = _Reflect.set(target, p, newValue, receiver);
                    setLoc(target);
                    return a;
                },
                get(target, p, receiver) {
                    let a = _Reflect.get(target, p, receiver);
                    if (typeof a === "function") {
                        _Host.of(a).stringify = () => `function ${String(p)}{ [native code] }`;
                    }
                    return a;
                },
            });
        },
        set(a) {
            this.get().href = a;
        }
    });
}