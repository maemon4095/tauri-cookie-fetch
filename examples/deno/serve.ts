import { Builder, BuilderOptions } from "https://raw.githubusercontent.com/maemon4095/deno-esbuilder/release/v0.2.0/src/mod.ts";
import tailwindcss from "npm:tailwindcss";
import postCssPlugin from "https://raw.githubusercontent.com/maemon4095/deno-esbuilder/release/v0.2.0/plugins/postCssPlugin.ts";
import tailwindConfig from "./tailwind.config.js";

const options: BuilderOptions = {
    documentFilePath: "./index.html",
    sourceRoot: "src",
    denoConfigPath: "./deno.json",
    esbuildOptions: {
        jsxFactory: "h",
        jsxFragment: "Fragment"
    },
    serve: {
        port: 1415,
        watch: ["src"]
    },
    esbuildPlugins: [
        postCssPlugin({
            plugins: [
                tailwindcss(tailwindConfig)
            ]
        })
    ]
};

const builder = new Builder(options);

await builder.serve();