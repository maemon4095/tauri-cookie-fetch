import { invoke } from "https://raw.githubusercontent.com/maemon4095/tauri-plugin-bin-ipc/release/v0.3.0/src-ts/mod.ts";

export type SameSite = "Strict" | "Lax" | "None";

export type CookieProps = {
    value: string;
    domain?: string;
    path?: string;
    httpOnly?: boolean;
    secure?: boolean;
    maxAge?: number;
    expires?: string;
    sameSite?: SameSite;
};

export type Cookies = Record<string, Record<string, CookieProps>>;

export type FetchOptions = {
    method?: string;
    headers?: HeaderMap;
    cookies?: Cookies;
    redirect?: RedirectPolicy;
    body?: Uint8Array;
};

export type RedirectPolicy = "follow" | "manual" | { limit: number };
export type HeaderMap = { [name: string]: string[] };

export type Response = {
    url: string;
    status: number;
    headers: HeaderMap;
    cookies: Cookies;
    body: Uint8Array;
};

export async function cookieFetch(
    url: string,
    options?: FetchOptions,
): Promise<Response> {
    if (options !== undefined) {
        const entries = Object.entries(options).filter(([, v]) =>
            v !== undefined
        );
        options = Object.fromEntries(entries);
    }
    return await invoke("cookie-fetch", "fetch", {
        url,
        options,
    }) as Response;
}
