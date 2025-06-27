import { hook, _Proxy, _Reflect } from '@portal-solutions/hooker-core'
import { camo } from '@portal-solutions/weevy-single-tenant'
import { decode, encode } from '@stablelib/base64'
import { hash256 } from "@stablelib/sha3"
import { ChaCha20Poly1305 } from '@stablelib/chacha20poly1305'
let endpoint = `${location.origin}/h`;
let [, a,  , c] = location.pathname.split('/');
let key = decode(a);
let p = new XMLHttpRequest();
p.open('GET', endpoint, false, 'google', encode(hash256(key)));
p.send(null);
// let x = p.responseText;
let [n, b] = p.responseText.split(',');
let x = new ChaCha20Poly1305(key);
let { assignment: fetchedPath } = JSON.parse(new TextDecoder().decode(x.open(decode(n), decode(b))!));
let logUrl = new URL(`${location.origin}${fetchedPath}`);
function sync(a: URL) {
    let a2 = a.toString();
    if (a2.startsWith(location.origin)) {
        a2 = a2.substring(location.origin.length);
        let nonce = new ArrayBuffer(12);
        crypto.getRandomValues(new Uint8Array(nonce));
        let newPath = encode(x.seal(new Uint8Array(nonce), new TextEncoder().encode(JSON.stringify({
            "assignment": a2,
        })))!);
        let newNonce = encode(new Uint8Array(nonce));
        let p = new XMLHttpRequest();
        p.open('POST', endpoint, false, 'google', encode(hash256(key)));
        p.send(`${newNonce},${newPath}`);
    } else {
        location.href = a2;
    }
}
_Reflect.defineProperty(globalThis, camo.rewrite('location'), {
    get() {
        return new _Proxy(logUrl, {
            getPrototypeOf(target) {
                return _Reflect.getPrototypeOf(location)
            },
            set(a, b, c) {
                let d = _Reflect.set(a, b, c);
                sync(logUrl);
                return d;
            }
        });
    },
    set(val) {
        this.get().href = val;
    }
});