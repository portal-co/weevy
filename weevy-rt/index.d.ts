import { Host as _Host } from '@portal-solutions/weevy-src-packager';
export declare function urlRewriter(base: string): (a: string) => string;
export declare let Host: typeof _Host;
export declare class Guest {
    #private;
    rewrite(a: string): string;
    of<T>(t: T): T;
    ofThis<T extends object>(this_: T, key: keyof T): T[typeof key];
    get globalThis(): typeof globalThis;
    constructor(id: string, rewriter: (a: string) => string);
}
interface GuestMap {
    [a: string]: Guest;
}
export declare const WeevyMain: {
    newSourceDecompressor(x: any): (r: any, v: any) => any;
    guests: GuestMap;
    withProxy<T extends object>(a: T): T;
    contextStack: string[];
    readonly context: string | undefined;
    wrap<T>(t: T): T;
    readonly contextGuest: Guest | undefined;
};
export {};
