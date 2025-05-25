export declare function urlRewriter(base: string): (a: string) => string;
export declare class Host {
    #private;
    static mappers: WeakMap<any, Host>;
    stringify: () => string | undefined;
    constructor(obj: any);
    static of(a: any): Host;
}
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
};
export {};
