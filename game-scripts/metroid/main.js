import { JSONRPCClient } from './json-rpc-2.0.js'
import { reactive, html } from './arrow.js'

const client = new JSONRPCClient((jsonRPCRequest) =>
  fetch("http://localhost:3030", {
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

let state = reactive({
  map: Array(32).fill(Array(32).fill(255)),
  position: {y: 0, x: 0},
  visited: new Set([]),
});

window.state = state;

function buildTr(y) {
  return html`
  <tr id="${() => `row-${y}`}">
    ${() => state.map.map(
      (_, x) => buildTd(y, x)
    )}
  </tr>
  `.key(`row-${y}`)
}

function buildTd(y, x) {
  const value = state.map[y][x].toString(16).padStart(2, '0');
  const display = value;
  const key = `cell-${y}-${x}`;
  const isCurrent = state.position.x == x && state.position.y == y;
  const isVisited = Reflect.getPrototypeOf(state.visited).has(y * 32 + x)

  return html`
  <td
    data-key="${() => key}"
    data-value="${() => value}"
    data-current="${() => isCurrent}"
    data-visited="${() => isVisited}"
  >${() => display}</td>
  `.key(key)
}

function toggleVisited() {
  document.querySelector('table').classList.toggle("hide-unvisited");
}

html`
  <button @click="${toggleVisited}">Show / hide unexplored</button>
  <table class="hide-unvisited">
    ${() => state.map.map(
      (_, y) => buildTr(y)
    )}
  </table>
`(document.querySelector(".container"))

async function setup() {
  const map_addr = Object.fromEntries(Array(1024).keys().map(i => [i, 0x0254E + i]));

  let {
    u8: map,
  } = await client.request("read_rom", {
    u8: map_addr
  });

  for (let y = 0; y < 32; y++) {
    for (let x = 0; x < 32; x++) {
      const cell = map[y * 32 + x];
      state.map[y][x] = cell;
    }
  }

  // TODO: The length is hardcoded for the original game.
  // Needs to be parsed dinamically if we wish to support rom hacks.
  const brinstar_item_data_addr = Object.fromEntries(Array(106).keys().map(i => [i, 0x063E6 + i]));

  let {
    u8: brinstar_item_data,
  } = await client.request("read_rom", {
    u8: brinstar_item_data_addr
  });

  let items = parseItemData(Object.values(brinstar_item_data))

  console.log(items)
}

function parseItemData(data) {
  let items = [];
  let item = {};

  while (data.length > 0) {

    item.y = data.shift();
    data.shift(); data.shift(); // Drop the next y pointer, we don't need it

    let hasNext = true;
    do {
      item.x = data.shift();
      hasNext = data.shift() != 0xFF;
      item.type = data.shift();

      switch (item.type) {
        case 0x1: // Squeept
          // TODO
          break;
        case 0x2: // Power up
          item.data = [data.shift(), data.shift()];
          data.shift(); // 0x00 terminator
          break;
        case 0x3: // Mellows
          data.shift(); // 0x00 terminator
          break;
        case 0x4: // Elevator
          item.data = [data.shift()];
          data.shift(); // 0x00 terminator
          break;
        case 0x5: // Mother brain room cannon
          // TODO
          break;
        case 0x6: // Mother brain
          // TODO
          break;
        case 0x7: // Zeebetite
          // TODO
          break;
        case 0x8: // Rinka
          // TODO
          break;
        case 0x9: // Door
          // TODO
          break;
        case 0xA: // Palette change room
          data.shift(); // 0x00 terminator
          break;

        default:
          break;
      }

      items.push(item);
      item = {y: item.y};

    } while (hasNext);
  }

  return items;
}

async function update() {
  let {
    u8: position,
  } = await client.request("read_memory", {
    u8: {
      y: 0x4F,
      x: 0x50,
    }
  });

  if (state.position.y != position.y || state.position.x != position.x) {
    state.position = position;
    const visited = Reflect.getPrototypeOf(state.visited)
    const index = position.y * 32 + position.x;
    if (!visited.has(index)) {
      visited.add(index);
      state.visited = visited;
    }
  }
}

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

window.run = true;

async function loop() {
  while (window.run) {
    try {
      await update();
    } catch (e) {
      window.run = false
      console.log(e)
    }

    await sleep(250);
  }
}

await setup()
loop()
