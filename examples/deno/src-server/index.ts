const port = 8080;

Deno.serve({ port }, (): Response => {
    const body = new ReadableStream<Uint8Array>({
        async pull(controller) {
            await sleep(100);
            const buf = fillRandomASCII();
            controller.enqueue(buf);
        },
    });
    return new Response(body);
});

function fillRandomASCII() {
    const buffer = new Uint8Array(1024);
    crypto.getRandomValues(buffer);
    for (let i = 0; i < buffer.length; ++i) {
        const c = buffer[i];
        buffer[i] = (c % (0x7F - 0x20)) + 0x20;
    }
    return buffer;
}

async function sleep(ms: number) {
    await new Promise((resolve) => {
        setTimeout(resolve, ms);
    });
}