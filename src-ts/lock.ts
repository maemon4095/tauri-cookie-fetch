import { ListQueue } from "https://raw.githubusercontent.com/maemon4095/ts_components/release/v0.2.0/collections/mod.ts";

export class Lock {
    readonly #waitings = new ListQueue<(v: void) => void>();
    #locked = false;

    async acquire(): Promise<void> {
        if (this.#locked) {
            await new Promise((resolve) => {
                this.#waitings.enqueue(resolve);
            });
        } else {
            this.#locked = true;
        }
    }
    release() {
        if (!this.#locked) {
            return;
        }

        if (this.#waitings.isEmpty) {
            this.#locked = false;
        } else {
            const resolve = this.#waitings.dequeue()!;
            resolve();
        }
    }
}
