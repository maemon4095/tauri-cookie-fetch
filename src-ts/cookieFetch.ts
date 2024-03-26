import TAURI from "./deps.ts";
import { connect } from "./connect.ts";
import { omit } from "./utils.ts";
const { invoke } = TAURI.tauri;

export type FetchOptions = {
    method?: string,
    headers?: HeaderMap;
    cookies?: Record<string, string>;
    redirect?: RedirectPolicy,
    body?: ReadableStream<Uint8Array>;
};

export type RedirectPolicy = "follow" | "manual" | { limit: number; };
export type HeaderMap = { [name: string]: string[]; };

export type Response = {
    url: string,
    status: number,
    headers: HeaderMap;
    cookies: Record<string, string>;
    body: ReadableStream<Uint8Array>;
};

export async function fetch(url: string | URL, options?: FetchOptions) {
    let str;
    if (url instanceof URL) {
        str = url.toString();
    } else {
        str = url;
    }

    const { id, upstream, downstream } = await connect();

    if (options !== undefined) {
        const body = omit(options, "body");
        body?.pipeTo(upstream);
    }

    const response = await invoke("plugin:cookie_fetch|fetch", {
        url: str,
        id,
        options
    }) as Omit<Response, "body">;

    return { ...response, body: downstream } as Response;
}


