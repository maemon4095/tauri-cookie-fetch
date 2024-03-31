import { Fragment, h } from "preact";
import { useState } from "preact/hooks";

export default function Headers({ headers, onremove, onappend }: {
  headers: [string, string][];
  onremove?: (idx: number) => void;
  onappend?: (name: string, value: string) => void;
}) {
  const [newItemName, setNewItemName] = useState("");
  const [newItemValue, setNewItemValue] = useState("");

  return (
    <div class="flex flex-col items-stretch">
      {headers.map(([name, value], idx) => (
        <Header
          {...{
            name,
            value,
            onremove: (onremove && (() => onremove(idx))),
          }}
        />
      ))}
      {onappend && (
        <div class="flex flex-row items-stretch gap-1 bg-slate-200 p-1">
          <input
            placeholder="header name"
            onInput={(e) => setNewItemName(e.currentTarget.value)}
            class="flex-1"
          >
          </input>
          <input
            class="flex-1"
            placeholder="header value"
            onInput={(e) => setNewItemValue(e.currentTarget.value)}
          >
          </input>
          <button
            onClick={() => onappend(newItemName, newItemValue)}
            class="bg-slate-100 rounded"
          >
            ➕
          </button>
        </div>
      )}
    </div>
  );
}

function Header(
  { name, value, onremove }: {
    name: string;
    value: string;
    onremove?: () => void;
  },
) {
  return (
    <div class="flex flex-row items-stretch p-1 border-slate-400 border-b">
      <span class="flex-1">{name}</span>
      <span class="flex-1">{value}</span>
      {onremove && (
        <button onClick={onremove} class="shadow rounded">
          ❌
        </button>
      )}
    </div>
  );
}
