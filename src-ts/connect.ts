import TAURI from "./deps.ts";
import { customSchemeLocalHost } from "./utils.ts";

const { listen } = TAURI.event;
const { invoke } = TAURI.tauri;
type UnlistenFn = TAURI.event.UnlistenFn;

const cookieFetchLocalHost = await customSchemeLocalHost("cookie-fetch-ipc");
export async function connect() {
    const id = await invoke("plugin:cookie_fetch|connect") as number;
    const channel = `${cookieFetchLocalHost}/${id}`;
    const popURL = `${channel}/pop`;
    const pushURL = `${channel}/push`;
    const closeUpstreamURL = `${channel}/close/upstream`;
    const closeDownstreamURL = `${channel}/close/downstream`;
    const upstream = new WritableStream({
        async write(chunk) {
            await fetch(pushURL, {
                method: "POST",
                body: chunk,
            });
        },
        async close() {
            await fetch(closeUpstreamURL, {
                method: "POST",
            });
        }
    });

    let unlisten: UnlistenFn;
    const downstream = new ReadableStream({
        type: "bytes",
        async start(controller) {
            unlisten = await listen<number>("cookie-fetch-ipc:ready-to-pop", async (e) => {
                if (e.payload !== id) {
                    return;
                }

                const res = await fetch(popURL, {
                    method: "POST",
                });

                switch (res.status) {
                    case 100: {
                        break;
                    }
                    case 200: {
                        const buf = await res.arrayBuffer();
                        controller.enqueue(new Uint8Array(buf));
                        break;
                    }
                    case 204: {
                        controller.close();
                        break;
                    }
                }
            });
        },
        async cancel() {
            unlisten();
            await fetch(closeDownstreamURL, {
                method: "POST",
            });
        },
    });

    return { id, upstream, downstream };
}

