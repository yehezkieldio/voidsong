import { $ } from "bun";
import { Layer } from "effect";
import * as Context from "effect/Context";
import * as Effect from "effect/Effect";

export interface RuntimeManagerInterface {
    readonly write: (path: string, content: string) => Effect.Effect<void, Error, never>;
    readonly execute: (command: string) => Effect.Effect<void, Error, never>;
}

export class RuntimeManager extends Context.Tag("RuntimeManager")<
    RuntimeManager,
    {
        readonly write: (path: string, content: string) => Effect.Effect<unknown, unknown, never>;
        readonly execute: (command: string, expected: string) => Effect.Effect<void, Error, never>;
    }
>() {}

export const RuntimeManagerLive = Layer.succeed(
    RuntimeManager,
    RuntimeManager.of({
        write: (path: string, content: string) =>
            Effect.promise(() =>
                Bun.write(path, content).then(() => {
                    return Effect.succeed(true);
                }),
            ),
        execute: (command: string, expected: string) =>
            Effect.promise(() =>
                $`${command}`.then((output) => {
                    if (output.stdout.includes(expected)) {
                        return Effect.succeed(true);
                    }

                    return Effect.fail(false);
                }),
            ),
    }),
);
