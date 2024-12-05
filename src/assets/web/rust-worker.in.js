self.methodCallStates = [];

self.methodCallResults = [];

self.start_method_call = function (method, args) {
    let index = self.methodCallStates.push("0") - 1;
    self.postMessage({ type: 'call', method: method, args: args, index: index });
    return index.toString();
};

self.finish_method_call = function (index) {
    let state = self.methodCallStates[parseInt(index)];
    if (state !== "0") {
        self.methodCallStates[index] = null;
        return JSON.stringify(self.methodCallResults[index]);
    }

    return "0";
};

self.onmessage = function (e) {
    let message = e.data;
    if (message.type === 'result') {
        self.methodCallResults[message.index] = message.result;
    }
};