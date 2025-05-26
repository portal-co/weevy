export declare class Host {
    #private;
    static mappers: WeakMap<any, Host>;
    stringify: () => string | undefined;
    constructor(obj: any);
    static of(a: any): Host;
}
export declare function newSourceDecompressor(x: any): (r: any, v: any) => any;
