export function initChrome(url: string, prefix: string) {
    (globalThis as any).chrome.webRequest.onBeforeRequest.addListener(
        function (details) {
            if (details.type !== 'script') {
                return;
            }
            if (!details.url.startsWith(url)) {
                let url2 = new URL(details.url);
                let a = ``;
                for (var [s, v] of [...url2.searchParams]) {
                    if (s.startsWith(prefix)) {
                        if (a) {
                            a = `${a}&`
                        }
                        a = `${a}${v.substring(prefix.length)}=${encodeURIComponent(v)}`
                        url2.searchParams.delete(s);
                    }
                }
                return { redirectUrl: `${url}extern=${btoa(url2.href)}${a}` }
            };
        },
        { urls: ["*://*/*"] },
        ["blocking"]
    )
}