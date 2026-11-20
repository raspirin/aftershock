(() => {
    const fetch = window.fetch.bind(window);
    let readDestination = () => window.location.href;
    let redirecting = false;

    window.aftershockSetAnubisDestination = (readUrl) => {
        readDestination = readUrl;
    };

    const destinationUrl = () => new URL(readDestination(), window.location.href);

    window.fetch = async (input, init) => {
        const request = new URL(input instanceof Request ? input.url : input, document.baseURI);
        const isServerFunction = request.pathname.startsWith("/api/");
        const isWasm = request.pathname.startsWith("/pkg/") && request.pathname.endsWith(".wasm");
        if (request.origin !== window.location.origin || (!isServerFunction && !isWasm)) {
            return fetch(input, init);
        }

        const destination = destinationUrl();
        const response = await fetch(input, init);
        const contentType = response.headers.get("content-type") ?? "";
        if (
            redirecting ||
            destination.origin !== window.location.origin ||
            !/^text\/html(?:\s*;|$)/i.test(contentType)
        ) {
            return response;
        }

        const html = await response.clone().text().catch(() => "");
        if (
            !redirecting &&
            destination.href === destinationUrl().href &&
            html.includes('<script id="anubis_challenge" type="application/json">')
        ) {
            redirecting = true;
            const current = new URL(window.location.href);
            if (destination.pathname === current.pathname && destination.search === current.search) {
                if (destination.hash !== current.hash) {
                    window.history.replaceState(window.history.state, "", destination.href);
                }
                window.location.reload();
            } else {
                window.location.assign(destination.href);
            }
        }

        return response;
    };
})();
