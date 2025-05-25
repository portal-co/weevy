export function initChrome(url) {
    globalThis.chrome.webRequest.onBeforeRequest.addListener(function (details) {
        if (details.type !== 'script') {
            return;
        }
        if (!details.url.startsWith(url))
            return { redirectUrl: `${url}extern=${btoa(details.url)}` };
    }, { urls: ["*://*/*"] }, ["blocking"]);
}
