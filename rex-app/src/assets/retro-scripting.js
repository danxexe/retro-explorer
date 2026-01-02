import { JSONRPCClient } from '/assets/json-rpc-2.0.js';

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

async function serverIsUp() {
  return await window.__TAURI__.core.invoke('check_server_status', { 
    address: "127.0.0.1:3030"
  });
}

async function waitForServer() {
  while (true) {
    if (await serverIsUp()) {
      return client();
    }

    await sleep(1000);
  }
}

async function pollUpdate(cb) {
  while (serverIsUp()) {
    try {
      await cb();
    } catch (e) {
      if (e.message !== 'signal timed out') {
        console.log(e);
      }

      break;
    }

    await sleep(1000);
  }
}

function client() {
  const client = new JSONRPCClient((jsonRPCRequest) =>
    fetch("http://localhost:3030", {
      signal: AbortSignal.timeout(500),
      method: "POST",
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify(jsonRPCRequest),
    }).then((response) => {
      if (response.status === 200) {
        return response
          .json()
          .then((jsonRPCResponse) => client.receive(jsonRPCResponse));
      } else if (jsonRPCRequest.id !== undefined) {
        return Promise.reject(new Error(response.statusText));
      }
    })
  );

  return client;
}

export {
  sleep,
  serverIsUp,
  waitForServer,
  pollUpdate,
  client,
};
