import { ComponentChildren, h } from "preact";

export default function Card(
  props: {
    title: ComponentChildren;
    children: ComponentChildren;
  } & h.JSX.HTMLAttributes<HTMLDivElement>,
) {
  const { title, children, ...others } = props;

  return (
    <div {...others} class="flex flex-col items-stretch rounded bg-slate-300">
      <span class="self-start mx-1 mt-1 px-1 rounded-t bg-slate-50">
        {title}
      </span>
      <div class="mx-1 mb-1 bg-slate-50">
        {children}
      </div>
    </div>
  );
}
