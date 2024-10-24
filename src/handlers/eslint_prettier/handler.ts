import { Effect } from "effect";

export const eslintPrettierHandler = () => {
    return Effect.log("Prettier + ESLint handler called");
};
