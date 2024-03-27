import { Builder, BuilderOptions } from "https://raw.githubusercontent.com/maemon4095/deno-esbuilder/release/v0.1.0/src/mod.ts";
import "./src-server/index.ts";

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
    }
};

const builder = new Builder(options);

await builder.serve();