import { JSONRPCClient } from './json-rpc-2.0.js'
import { reactive, html } from './arrow.js'

const mapNames = new Map([
  [0x0000, {name: "School Class 5A"}],
  [0x0100, {name: "School Class 5B"}],
  [0x0200, {name: "School Library"}],
  [0x0300, {name: "School 2F Hallway"}],
  [0x0500, {name: "School Class 1A"}],
  [0x0600, {name: "School Class 1B"}],
  [0x0700, {name: "School AV Room"}],
  [0x0800, {name: "School Infirmary"}],
  [0x0900, {name: "School 1F Hallway"}],
  [0x0B00, {name: "School Cross Hallway"}],
  [0x0C00, {name: "School Storage"}],
  [0x0D00, {name: "School Staff Lounge"}],
  [0x0E00, {name: "School Staff Lounge Hallway"}],
  [0x0001, {name: "ACDC Town"}],
  [0x0101, {name: "ACDC School Gate"}],
  [0x0201, {name: "ACDC Lan Living Room"}],
  [0x0301, {name: "ACDC Lan Room"}],
  [0x0501, {name: "ACDC Mayl Living Room"}],
  [0x0601, {name: "ACDC Mayl Room"}],
  [0x0701, {name: "ACDC Dex Room"}],
  [0x0901, {name: "ACDC Yai Room"}],
  [0x0B01, {name: "ACDC Higsbys"}],
  [0x0C01, {name: "ACDC Station"}],
  [0x0D01, {name: "ACDC Secret Station"}],
  [0x0002, {name: "Govt Complex Front"}],
  [0x0102, {name: "Govt Complex Station"}],
  [0x0202, {name: "Govt Waterworks Lobby"}],
  [0x0302, {name: "Govt SciLab Lobby"}],
  [0x0402, {name: "Govt Complex Hallway"}],
  [0x0502, {name: "Govt Yuichiro Lab"}],
  [0x0602, {name: "Govt Waterworks Office"}],
  [0x0702, {name: "Govt Waterworks Control Room"}],
  [0x0902, {name: "Govt Waterworks Pump Room"}],
  [0x0B02, {name: "Govt Waterworks Purification Room"}],
  [0x0003, {name: "DenTown Center"}],
  [0x0103, {name: "DenTown Station"}],
  [0x0203, {name: "DenTown Block 1"}],
  [0x0303, {name: "DenTown Block 2"}],
  [0x0403, {name: "DenTown Block 3"}],
  [0x0503, {name: "DenTown Block 4"}],
  [0x0603, {name: "DenTown Miyu Antiques"}],
  [0x0703, {name: "DenTown Summer School"}],
  [0x0004, {name: "SciLab Restaurant Hallway"}],
  [0x0104, {name: "SciLab Restaurant"}],
  [0x0204, {name: "SciLab Power Plant Hallway"}],
  [0x0304, {name: "SciLab Power Plant"}],
  [0x0404, {name: "SciLab Power Plant Control Room"}],
  [0x0504, {name: "SciLab Generator Room"}],
  [0x0005, {name: "WWW Base"}],
  [0x0105, {name: "WWW Wily Lab"}],
  [0x0205, {name: "WWW Rocket Hangar"}],
  [0x0305, {name: "WWW Passage 1"}],
  [0x0405, {name: "WWW Passage 2"}],
  [0x0505, {name: "WWW Passage 3"}],
  [0x0080, {name: "School Comp 1", img: "maps/School-1.png"}],
  [0x0180, {name: "School Comp 2", img: "maps/School-2.png"}],
  [0x0280, {name: "School Comp 3", img: "maps/School-3.png"}],
  [0x0380, {name: "School Comp 4", img: "maps/School-4.png"}],
  [0x0480, {name: "School Comp 5", img: "maps/School-5.png"}],
  [0x0081, {name: "Oven Comp 1", img: "maps/Oven-1.png"}],
  [0x0181, {name: "Oven Comp 2", img: "maps/Oven-2.png"}],
  [0x0082, {name: "Waterworks Comp 1", img: "maps/WaterWorks-1.png"}],
  [0x0182, {name: "Waterworks Comp 2", img: "maps/WaterWorks-2.png"}],
  [0x0282, {name: "Waterworks Comp 3", img: "maps/WaterWorks-3.png"}],
  [0x0382, {name: "Waterworks Comp 4", img: "maps/WaterWorks-4.png"}],
  [0x0482, {name: "Waterworks Comp 5", img: "maps/WaterWorks-5.png"}],
  [0x0582, {name: "Waterworks Comp 6", img: "maps/WaterWorks-6.png"}],
  [0x0083, {name: "Traffic Light Comp 1", img: "maps/Traffic-1.png"}],
  [0x0183, {name: "Traffic Light Comp 2", img: "maps/Traffic-2.png"}],
  [0x0283, {name: "Traffic Light Comp 3", img: "maps/Traffic-3.png"}],
  [0x0383, {name: "Traffic Light Comp 4", img: "maps/Traffic-4.png"}],
  [0x0483, {name: "Traffic Light Comp 5", img: "maps/Traffic-5.png"}],
  [0x0084, {name: "Power Plant Comp 1", img: "maps/PowerPlant-1.png"}],
  [0x0184, {name: "Power Plant Comp 2", img: "maps/PowerPlant-2.png"}],
  [0x0284, {name: "Power Plant Comp 3", img: "maps/PowerPlant-3.png"}],
  [0x0384, {name: "Power Plant Comp 4", img: "maps/PowerPlant-4.png"}],
  [0x0085, {name: "WWW Comp 1", img: "maps/WWW-1.png"}],
  [0x0185, {name: "WWW Comp 2", img: "maps/WWW-2.png"}],
  [0x0285, {name: "WWW Comp 3", img: "maps/WWW-3.png"}],
  [0x0385, {name: "WWW Comp 4", img: "maps/WWW-4.png"}],
  [0x0485, {name: "WWW Comp 5", img: "maps/WWW-5.png"}],
  [0x0585, {name: "Rocket Comp"}],
  [0x0088, {name: "ACDC Lan PC"}],
  [0x0188, {name: "ACDC Mayl PC"}],
  [0x0288, {name: "ACDC Yai PC"}],
  [0x0388, {name: "ACDC Dex PC"}],
  [0x0089, {name: "Govt Yuichiro PC"}],
  [0x0189, {name: "Govt Lunch Cart Comp"}],
  [0x008A, {name: "DenTown Antique Comp"}],
  [0x008B, {name: "SciLab Fish Stand Comp"}],
  [0x008C, {name: "Other Doghouse Comp"}],
  [0x018C, {name: "Other Servbot Comp"}],
  [0x028C, {name: "Other New Game Machine Comp"}],
  [0x038C, {name: "Other Telephone Comp"}],
  [0x048C, {name: "Other Car Comp"}],
  [0x058C, {name: "Other Waterworks Vending Machine"}],
  [0x068C, {name: "Other Lobby TV Comp"}],
  [0x078C, {name: "Other Large Monitor Comp"}],
  [0x088C, {name: "Other Control Equipment Comp"}],
  [0x098C, {name: "Other SciLab Vending Machine"}],
  [0x0A8C, {name: "Other Recycled PET Comp"}],
  [0x0B8C, {name: "Other Big Vase Comp"}],
  [0x0C8C, {name: "Other Blackboard Comp"}],
  [0x0090, {name: "Internet 1", img: "maps/Internet-01.png"}],
  [0x0190, {name: "Internet 2", img: "maps/Internet-02.png"}],
  [0x0290, {name: "Internet 3", img: "maps/Internet-03.png"}],
  [0x0390, {name: "Internet 4", img: "maps/Internet-04.png"}],
  [0x0490, {name: "Undernet 1", img: "maps/Internet-05.png"}],
  [0x0590, {name: "Undernet 2", img: "maps/Internet-06.png"}],
  [0x0690, {name: "Undernet 3", img: "maps/Internet-07.png"}],
  [0x0790, {name: "Undernet 4", img: "maps/Internet-08.png"}],
  [0x0890, {name: "Undernet 5", img: "maps/Internet-09.png"}],
  [0x0990, {name: "Undernet 6", img: "maps/Internet-10.png"}],
  [0x0A90, {name: "Undernet 7", img: "maps/Internet-11.png"}],
  [0x0B90, {name: "Undernet 8", img: "maps/Internet-12.png"}],
  [0x0C90, {name: "Undernet 9", img: "maps/Internet-13.png"}],
  [0x0D90, {name: "Undernet 10", img: "maps/Internet-14.png"}],
  [0x0E90, {name: "Undernet 11", img: "maps/Internet-15.png"}],
  [0x0F90, {name: "Undernet 12", img: "maps/Internet-16.png"}],
  [0xFFFF, {name: "None"}],
]);

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
  map: {},
});

window.state = state;

function buildMapWithImg() {
  return html`
  <img src="${() => state.map.img}" alt="">
  `
}

function buildMapWithoutImg() {
  return html`
  <h1>${() => state.map.name}</h1>
  `
}

html`
  ${() => state.map.img ? buildMapWithImg() : buildMapWithoutImg()}
`(document.querySelector(".container"))

async function update() {
  let {
    u8: {scene},
    u16le: {mapId}
  } = await client.request("read_memory", {
    u8: {
      scene: 0xf590 - 0x008000
    },
    u16le: {
      mapId: 0x008214 - 0x008000
    }
  });

  if (scene != 0x02) {
    return;
  }

  const map = mapNames.get(mapId);

  state.map.id = mapId;
  state.map.name = map.name;
  state.map.img = map.img;
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

    await sleep(1000);
  }
}

loop()
