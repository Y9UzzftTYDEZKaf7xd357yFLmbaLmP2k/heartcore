import * as hc_workspace from "./lib/hc_workspace/hc_workspace.js";
import {readMessage, uuidv4} from "./vendor/sync-message/index.js";
import {
    RTCPeerConnection,
    RTCSessionDescription,
    RTCIceCandidate
} from './vendor/worker-webrtc/worker.js';

self.RTCPeerConnection = RTCPeerConnection;
self.RTCSessionDescription = RTCSessionDescription;
self.RTCIceCandidate = RTCIceCandidate;
  
async function run() {
    await hc_workspace.default();
    postMessage( { type: 'ready' } );
}
run();

self.channel = null;

self.call_method = function (method, args) {
    const messageId = uuidv4();

    postMessage({ id: messageId, type: 'call', method: method, args: args });

    const message = readMessage(self.channel, messageId);

    return message;
};

self.log = function(...args) {
    postMessage({type: 'log', arguments: args});
}

self.onmessage = function (e) {
    let message = e.data;
    if (message.type === 'registerChannel') {
        self.channel = message.channel;
    } else if (message.type === 'callFromJs') {
        let result = hc_workspace[message.method](...message.args);
        if (result instanceof Promise) {
            result = '';
        }
        postMessage({ type: 'result', result: result });
    }
};