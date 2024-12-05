import * as hc_workspace from "./lib/hc_workspace/hc_workspace.js";
async function run() {
await hc_workspace.default();
hc_workspace.start();
}
run();
