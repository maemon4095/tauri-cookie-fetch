import { useState } from "preact/hooks";
import { cookieFetch } from "@cookie-fetch";

export default function App() {
  const [url, setUrl] = useState("https://example.com");
  const [response, setResponse] = useState("");

  return (
    <div class="size-full p-2 flex gap-1 flex-col items-stretch">
      <div class="flex-1 flex flex-col gap-1 items-stretch bg-slate-200 rounded">
        <div class="px-1">
          Response
        </div>
        <textarea
          value={response}
          class="flex-1 resize-none m-1 mt-0 p-1 font-mono text-sm leading-4 "
          readonly
        />
      </div>

      <div class="flex flex-row items-stretch gap-1 bg-slate-200 p-2 rounded">
        <button
          onClick={async () => {
            const res = await cookieFetch(url);
            setResponse(
              JSON.stringify(res, (k, v) => {
                if (k === "body") {
                  const buf = v as Uint8Array;
                  return { byteLength: buf.byteLength };
                }
                return v;
              }, 2),
            );
          }}
          class="rounded bg-slate-400 text-gray-100 px-1"
        >
          SEND
        </button>
        <label class="flex-1 flex flex-row gap-1 bg-slate-300 p-1 rounded">
          URL:
          <input
            value={url}
            onInput={(e) => {
              setUrl(e.currentTarget.value);
            }}
            class="flex-1 px-1"
          />
        </label>
      </div>
    </div>
  );
}
