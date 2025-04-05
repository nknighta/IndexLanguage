import init, { greet } from "./pkg/il_compiler.js";
init().then(() => {
    greet("Holy fuck Microsoft!");
});