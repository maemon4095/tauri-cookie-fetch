import TAURI from "./deps.ts";
import { customSchemeLocalHost } from "./utils.ts";
import { Lock } from "./lock.ts";

const { listen } = TAURI.event;
const { invoke } = TAURI.tauri;

const readyToPopListeners = {} as { [id: number]: undefined | (() => void) | (() => Promise<unknown>); };
await listen<number>("cookie-fetch-ipc:ready-to-pop", async (e) => await readyToPopListeners[e.payload]?.());

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

    const downstreamLock = new Lock();
    const downstream = new ReadableStream({
        type: "bytes",
        start(controller) {
            readyToPopListeners[id] = async () => {
                await downstreamLock.acquire();
                try {
                    const res = await fetch(popURL, {
                        method: "POST",
                    });

                    switch (res.status) {
                        case 100: {
                            break;
                        }
                        case 200: {
                            for await (const chunk of res.body!) {
                                controller.enqueue(chunk);
                            }
                            break;
                        }
                        case 204: {
                            controller.close();
                            break;
                        }
                    }
                } finally {
                    downstreamLock.release();
                }
            };
        },
        async cancel() {
            delete readyToPopListeners[id];
            await fetch(closeDownstreamURL, {
                method: "POST",
            });
        },
    });

    return { id, upstream, downstream };
}

