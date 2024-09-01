import { useState } from "preact/hooks";
import { cookieFetch } from "@cookie-fetch";

export default function App() {
  async function greet() {
    const res = await cookieFetch("https://ssl.dlsite.com/home/mypage", {});
    console.log(res);
  }

  greet();

  return (
    <div className="container">
    </div>
  );
}
