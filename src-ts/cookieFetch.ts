import { invoke } from "npm:@tauri-apps/api/tauri";

export type FetchOptions = {
    method?: string,
    headers?: HeaderMap;
    cookies?: Record<string, string>;
    redirect?: RedirectPolicy,
    body?: Uint8Array;
};

export type RedirectPolicy = "follow" | "manual" | { limit: number; };
export type HeaderMap = { [name: string]: string[]; };

export type Response = {
    url: string,
    status: number,
    headers: HeaderMap;
    cookies: Record<string, string>;
    body: Array<number>;
};

export async function cookieFetch(url: string | URL, options?: FetchOptions): Promise<Response> {
    return await invoke<Response>("plugin:cookie_fetch|fetch", { url, options });
}


