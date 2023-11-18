import { invoke } from "https://esm.sh/@tauri-apps/api@1.5.0/tauri";

export type FetchOptions<T extends PayloadType> = {
    responseType: T,
    method?: string,
    headers?: HeaderMap;
    cookies?: Record<string, string>;
    redirect?: RedirectPolicy,
    body?: Body<PayloadType>;
};

export type Body<T> =
    T extends "binary" ? { type: T; payload: Array<number>; } :
    T extends "text" ? { type: T; payload: string; } :
    T extends "discard" ? null :
    never;

export type RedirectPolicy = { type: "none"; } | { type: "limited"; max: number; };

export type PayloadType = "binary" | "text" | "discard";

export type HeaderMap = { [name: string]: string[]; };

export type Response<T extends PayloadType> = {
    url: string,
    status: number,
    headers: HeaderMap;
    cookies: Record<string, string>;
    body: Body<T>;
};

export async function fetch<T extends PayloadType>(url: string | URL, options?: FetchOptions<T>) {
    let str;
    if (url instanceof URL) {
        str = url.toString();
    } else {
        str = url;
    }

    const response = await invoke("plugin:cookie_fetch|fetch", { url: str, options });
    return response as Response<T>;
}