#!/usr/bin/env bun

import { Command, Prompt } from "@effect/cli";
import { BunContext, BunRuntime } from "@effect/platform-bun";
import { Effect, Exit, Match } from "effect";

import { red } from "colorette";
import { biomeHandler } from "#/handlers/biome/handler";
import { eslintPrettierHandler } from "#/handlers/eslint_prettier/handler";

const toolPrompt = Prompt.select({
    message: "What tool or utility would you like to configure?",
    choices: [
        { title: "Biome", value: "biome" },
        { title: "ESLint + Prettier", value: "eslint_prettier" },
    ],
    maxPerPage: 5,
});

const handleOptions = (config: string) => {
    return Match.value({ value: config }).pipe(
        Match.when({ value: "biome" }, () => biomeHandler()),
        Match.when({ value: "eslint_prettier" }, () => eslintPrettierHandler()),
        Match.orElse(() => Effect.fail("Invalid tool selected")),
    );
};

const command = Command.prompt("configure", toolPrompt, handleOptions);

const cli = Command.run(command, {
    name: "voidsong",
    version: "1.0.0",
});

const program = Effect.gen(function* (_) {
    yield* Effect.suspend(() => cli(process.argv));
});

program.pipe(
    Effect.provide(BunContext.layer),
    Effect.exit,
    Effect.map(
        Exit.match({
            onSuccess: () => process.exit(0),
            onFailure: (cause) => {
                console.error(red("Program failed:"), cause);
                process.exit(1);
            },
        }),
    ),
    BunRuntime.runMain,
);
