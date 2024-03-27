import { bundle } from "https://deno.land/x/emit@0.38.2/mod.ts";

const result = await bundle(
    "./src-ts/mod.ts",
    {
        minify: true,
        compilerOptions: {
            sourceMap: true
        }
    }
);

await Deno.mkdir("./dist", { recursive: true });
await Deno.writeTextFile("./dist/mod.js", result.code);
await Deno.writeTextFile("./dist/mod.js.map", result.map!);