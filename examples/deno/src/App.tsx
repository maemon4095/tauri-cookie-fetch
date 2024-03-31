import { h } from "preact";
import { useRef, useState } from "preact/hooks";
import { fetch, HeaderMap } from "../../../src-ts/mod.ts";
import Headers from "./Headers.tsx";
import Card from "./Card.tsx";
export default function App() {
  const input = useRef<HTMLInputElement | null>(null);
  const [url, setURL] = useState("");
  const [requestHeaders, setRequestHeaders] = useState(
    [] as [string, string][],
  );
  const [requestMethod, setRequestMethod] = useState("GET");
  const [requestBody, setRequestBody] = useState(null as string | null);

  const [responseBody, setResponseBody] = useState("");
  const [responseHeaders, setResponseHeaders] = useState(
    [] as [string, string][],
  );

  const onClickFetch = async () => {
    setResponseBody("");
    input.current!.setCustomValidity("");
    try {
      const res = await fetch(url, {
        method: requestMethod,
        body: iterableStream([requestBody]).pipeThrough(
          new TextEncoderStream(),
        ),
        headers: arrayToHeaderMap(requestHeaders),
      });
      setResponseHeaders(headerMapToArray(res.headers));

      const reader = res.body.getReader();
      const decoder = new TextDecoder("utf-8");
      while (true) {
        const { done, value } = await reader.read();
        if (done) {
          break;
        }
        const seg = decoder.decode(value);
        setResponseBody((t) => t + seg);
      }
      reader.releaseLock();
    } catch (e) {
      if (!("message" in e)) {
        throw e;
      }
      const current = input.current!;
      current.setCustomValidity(`${e.message}`);
      current.reportValidity();
      console.log(e);
    }
  };

  return (
    <div class="w-full m-0 p-4 flex flex-col items-stretch gap-4">
      <div class="flex flex-col items-stretch gap-2">
        <h1 class="font-bold border-b text-xl">Request</h1>
        <Card title="headers">
          <Headers
            headers={requestHeaders}
            onremove={(idx) => setRequestHeaders((h) => h.toSpliced(idx, 1))}
            onappend={(name, value) =>
              setRequestHeaders((h) => h.toSpliced(-1, 0, [name, value]))}
          />
        </Card>
        <Card title="body">
          <textarea
            onInput={(e) => setRequestBody(e.currentTarget.value)}
            class="resize-y size-full align-top"
          >
          </textarea>
        </Card>
        <div class="flex flex-row items-stretch gap-1 rounded p-1 bg-slate-300">
          <select
            onChange={(e) => setRequestMethod(e.currentTarget.value)}
          >
            <option>GET</option>
            <option>POST</option>
            <option>PUT</option>
            <option>DELETE</option>
          </select>
          <input
            ref={input}
            name="url"
            type="url"
            onInput={(e) => setURL(e.currentTarget.value)}
            class="flex-1"
          >
          </input>
          <button
            onClick={onClickFetch}
            class="rounded bg-slate-100 p-1"
          >
            fetch
          </button>
        </div>
      </div>
      <div class="flex flex-col items-stretch gap-2">
        <h1 class="font-bold border-b text-xl">Response</h1>
        <Card title="headers">
          <Headers headers={responseHeaders} />
        </Card>
        <Card title="body">
          <div class="w-full overflow-x-auto">
            <pre class="w-full font-mono">
            {responseBody}
            </pre>
          </div>
        </Card>
      </div>
    </div>
  );
}

function iterableStream<T>(iterable: Iterable<T>): ReadableStream<T> {
  const iter = iterable[Symbol.iterator]();
  return new ReadableStream({
    pull(controller) {
      const { done, value } = iter.next();
      if (done) {
        controller.close();
      } else {
        controller.enqueue(value);
      }
    },
  });
}

function arrayToHeaderMap(array: [string, string][]): HeaderMap {
  const map: HeaderMap = {};
  for (const [name, value] of array) {
    if (map[name] === undefined) {
      map[name] = [value];
    } else {
      map[name].push(value);
    }
  }
  return map;
}

function headerMapToArray(headermap: HeaderMap): [string, string][] {
  return Object.entries(headermap).flatMap(([name, values]) => {
    return values.map((v) => [name, v] as [string, string]);
  });
}
