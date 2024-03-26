import TAURI from "./deps.ts";
const { platform } = TAURI.os;

export async function customSchemeLocalHost(scheme: string) {
    const name = await platform();
    if (name === "win32") {
        return `https://${scheme}.localhost`;
    }

    return `${scheme}://localhost`;
}

export function omit<T, K extends keyof T>(value: T, key: K): T[K] {
    const prop = value[key];
    delete value[key];
    return prop;
}