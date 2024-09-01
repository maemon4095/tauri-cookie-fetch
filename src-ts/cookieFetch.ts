import { invoke } from "https://raw.githubusercontent.com/maemon4095/tauri-plugin-bin-ipc/release/v0.2.0/src-ts/mod.ts";

export type FetchOptions = {
    method?: string;
    headers?: HeaderMap;
    cookies?: Record<string, string>;
    redirect?: RedirectPolicy;
    body?: Uint8Array;
};

export type RedirectPolicy = "follow" | "manual" | { limit: number };
export type HeaderMap = { [name: string]: string[] };

export type Response = {
    url: string;
    status: number;
    headers: HeaderMap;
    cookies: Record<string, string>;
    body: Uint8Array;
};

export async function cookieFetch(
    url: string | URL,
    options?: FetchOptions,
): Promise<Response> {
    return await invoke("cookie-fetch", "fetch", { url, options }) as Response;
}