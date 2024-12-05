
self.methodCallStates = [];

self.methodCallResults = [];

self.methodCallBuffers = [];
self.methodResponseBuffers = [];

self.start_method_call = function (method, args) {

    const sab = new SharedArrayBuffer(4, { maxByteLength: 1 * 1000 * 1000 }); // 1mb max for now
    self.postMessage({ type: 'call', method: method, args: args/*, index: index*/, sab: sab });
    Atomics.wait(new Int32Array(sab), 0, 0);

    return "ok";

    /*
    
        self.postMessage({ type: 'log', message: method });
        let index = self.methodCallStates.push("0") - 1;
    
        const sab = new SharedArrayBuffer(4);
        self.methodCallBuffers[index] = sab;
        let int32View = new Int32Array(sab);
    
        const sab2 = new SharedArrayBuffer(4);
        self.methodResponseBuffers[index] = sab2;
        let int32View2 = new Int32Array(sab2);
        self.postMessage({ type: 'call', method: method, args: args, index: index, sab: sab, response_sab: sab2 });
        Atomics.wait(int32View, 0, 0);
        self.postMessage({ type: 'log', message: "original call buffer was notified, now waiting on response postMessage" });
        Atomics.wait(int32View2, 0, 0, 1000);
        self.postMessage({ type: 'log', message: index + " " + self.methodCallResults[index] });
        return self.methodCallResults[index];
        return index.toString();*/
};

self.finish_method_call = function (index) {
    self.postMessage({ type: 'finish', index: index });
    let state = self.methodCallStates[parseInt(index)];
    if (state !== "0") {
        self.methodCallStates[index] = null;
        return JSON.stringify(self.methodCallResults[index]);
    }

    let sharedArrayBuffer = self.methodCallBuffers[index];
    let int32View = new Int32Array(sharedArrayBuffer);
    Atomics.wait(int32View, 0, 0, 1000);

    return "0";
};

self.onmessage = function (e) {
    let message = e.data;
    self.postMessage({ type: 'log', message: message });
    if (message.type === 'result') {
        self.methodCallResults[message.index] = message.result;
    }
    Atomics.notify(new Int32Array(message.sab), 0);
};