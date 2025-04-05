import { main } from "../pkg/il_compiler.js";
const arg = process.argv.slice(2);
if (arg.length === 0) {
    console.log("Please provide a name as an argument.");
    process.exit(254);
}
main(arg[0]);
