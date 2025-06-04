import { hook, _Proxy, _Reflect } from '@portal-solutions/hooker-core'
export const symWeevyMain = Symbol.for("weevy main");
export const globalName = '__WeevyMain';
Object.defineProperty(globalThis, globalName, {
    value: symWeevyMain,
    writable: false,
    configurable: false,
    enumerable: false
});