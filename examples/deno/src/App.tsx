import { h } from "preact";
import { useEffect, useState } from "preact/hooks";
import { fetch } from "../../../src-ts/mod.ts";
export default function App() {
  const [received, setReceived] = useState("");
  useEffect(() => {
    (async () => {
      const res = await fetch("http://localhost:8080");
      const reader = res.body.getReader();
      const decoder = new TextDecoder("utf-8");
      while (true) {
        const { done, value } = await reader.read();
        console.log(done);
        if (done) {
          break;
        }
        const seg = decoder.decode(value);
        setReceived((t) => seg);
      }
      reader.releaseLock();
    })();
  }, []);

  return (
    <div>
      {received}
    </div>
  );
}
