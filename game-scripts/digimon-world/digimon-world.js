import { html } from './arrow.js'

export const stats = [
  {
    hp: html`<img src="img/hp.png">&nbsp;HP`,
    mp: html`<img src="img/mp.png">&nbsp;MP`,
    off: html`<img src="img/offense.png">&nbsp;Off`,
    def: html`<img src="img/defense.png">&nbsp;Def`,
    speed: html`<img src="img/speed.png">&nbsp;<small>Speed</small>`,
    brains: html`<img src="img/brains.png">&nbsp;<small>Brains</small>`,
  },
  {
    mistakes: html`<img src="img/care.png">&nbsp;<small>Care</small>`,
  },
  {
    weight: html`<img src="img/weight.png">&nbsp;<small>Weight</small>`,
  },
  {
    happiness: html`<img src="img/happiness.png">&nbsp;<small>Happy</small>`,
    discipline: html`<img src="img/discipline.png">&nbsp;<small>Disc.</small>`,
    battles: html`<img src="img/battle.png">&nbsp;<small>Battle</small>`,
    techs: html`<img src="img/techniques.png">&nbsp;<small>Techs</small>`,
    partner: "",
  }
];

export const digimonNames = new Map([
  [0x93dc, "Poyomon"],
  [0x98b4, "Yuramon"],
  [0x978c, "Botamon"],
  [0x9a14, "Punimon"],
  [0x924c, "Koromon"],
  [0x8e44, "Tsunomon"],
  [0xa4f4, "Tokomon"],
  [0xa550, "Tanemon"],
  [0xbccc, "Patamon"],
  [0xc4ac, "Betamon"],
  [0xc300, "Palmon"],
  [0xc248, "Elecmon"],
  [0xbeb8, "Penguinmon"],
  [0xc8d0, "Agumon"],
  [0xc890, "Gabumon"],
  [0xc5a4, "Kunemon"],
  [0xc0c8, "Biyomon"],
  [0xc8c8, "Birdramon"],
  [0xbb50, "Frigimon"],
  [0xccb0, "Mojyamon"],
  [0xc2dc, "Nanimon"],
  [0xcbdc, "Centarumon"],
  [0xc654, "Greymon"],
  [0xc9cc, "Monochromon"],
  [0xc714, "Drimogemon"],
  [0xc744, "Tyrannomon"],
  [0xd080, "Leomon"],
  [0xcdc8, "Ogremon"],
  [0xca4c, "Angemon"],
  [0xb200, "Bakemon"],
  [0xbe68, "Airdramon"],
  [0xcc70, "Kokatorimon"],
  [0xc5a0, "Unimon"],
  [0xcce0, "Devimon"],
  [0xcbd8, "Garurumon"],
  [0xcf94, "Kuwagamon"],
  [0xbad8, "Vegiemon"],
  [0xce14, "Ninjamon"],
  [0xbfc0, "Seadramon"],
  [0xca04, "Shellmon"],
  [0xb0e8, "Whamon"],
  [0xd010, "Kabuterimon"],
  [0xc834, "Coelamon"],
  [0xc7e0, "Numemon"],
  [0xcbcc, "Sukamon"],
  [0xd434, "Meramon"],
  [0xca80, "MetalGreymon"],
  [0xcc64, "Andromon"],
  [0xcc24, "SkullGreymon"],
  [0xcd48, "Megadramon"],
  [0xcb50, "MetalMamemon"],
  [0xbe8c, "Mamemon"],
  [0xb764, "Giromon"],
  [0xc798, "Piximon"],
  [0xb388, "Monzaemon"],
  [0xcb24, "Etemon"],
  [0xc3f0, "Vademon"],
  [0xd160, "Digitamamon"],
  [0xd148, "Phoenixmon"],
  [0xcc30, "MegaSeadramon"],
  [0xd1ac, "H-Kabuterimon"]
]);

export const evolutionPaths = {
  "Botamon": ["Koromon", "Sukamon"],
  "Punimon": ["Tsunomon", "Sukamon"],
  "Poyomon": ["Tokomon", "Sukamon"],
  "Yuramon": ["Tanemon", "Sukamon"],
  "Koromon": ["Agumon", "Gabumon", "Kunemon", "Sukamon"],
  "Tokomon": ["Patamon", "Biyomon", "Kunemon", "Sukamon"],
  "Tsunomon": ["Elecmon", "Penguinmon", "Kunemon", "Sukamon"],
  "Tanemon": ["Palmon", "Betamon", "Kunemon", "Sukamon"],
  "Agumon": ["Greymon", "Meramon", "Birdramon", "Centarumon", "Monochromon","Tyrannomon", "Numemon", "Nanimon", "Sukamon"],
  "Gabumon": ["Centarumon", "Monochromon", "Drimogemon", "Tyrannomon", "Ogremon","Garurumon", "Numemon", "Nanimon", "Sukamon"],
  "Patamon": ["Drimogemon", "Tyrannomon", "Ogremon", "Leomon", "Angemon", "Unimon", "Numemon", "Nanimon", "Sukamon"],
  "Elecmon": ["Leomon", "Angemon", "Bakemon", "Kokatorimon", "Numemon", "Nanimon", "Sukamon"],
  "Biyomon": ["Birdramon", "Airdramon", "Kokatorimon", "Unimon", "Kabuterimon", "Numemon", "Nanimon", "Sukamon"],
  "Kunemon": ["Bakemon", "Kabuterimon", "Kuwagamon", "Vegiemon", "Numemon", "Nanimon", "Sukamon"],
  "Palmon": ["Kuwagamon", "Vegiemon", "Ninjamon", "Whamon", "Coelamon", "Numemon", "Nanimon", "Sukamon"],
  "Betamon": ["Seadramon", "Whamon", "Shellmon", "Coelamon", "Numemon", "Nanimon", "Sukamon"],
  "Penguinmon": ["Whamon", "Shellmon", "Garurumon", "Frigimon", "Mojyamon", "Numemon", "Nanimon", "Sukamon"],
  "Greymon": ["MetalGreymon", "SkullGreymon", "Vademon", "Sukamon"],
  "Meramon": ["MetalGreymon", "Andromon", "Vademon", "Sukamon"],
  "Birdramon": ["Phoenixmon", "Vademon", "Sukamon"],
  "Centarumon": ["Andromon", "Giromon", "Vademon", "Sukamon"],
  "Monochromon": ["MetalGreymon", "MetalMamemon", "Vademon", "Sukamon"],
  "Drimogemon": ["MetalGreymon", "Vademon", "Sukamon"],
  "Tyrannomon": ["MetalGreymon", "Megadramon", "Vademon", "Sukamon"],
  "Devimon": ["SkullGreymon", "Megadramon", "Vademon", "Sukamon"],
  "Ogremon": ["Andromon", "Giromon", "Vademon", "Sukamon"],
  "Leomon": ["Andromon", "Mamemon", "Vademon", "Sukamon"],
  "Angemon": ["Andromon", "Phoenixmon", "Devimon", "Vademon", "Sukamon"],
  "Bakemon": ["SkullGreymon", "Giromon", "Vademon", "Sukamon"],
  "Airdramon": ["Megadramon", "Phoenixmon", "Vademon", "Sukamon"],
  "Kokatorimon": ["Phoenixmon", "Piximon", "Vademon", "Sukamon"],
  "Unimon": ["Giromon", "Phoenixmon", "Vademon", "Sukamon"],
  "Kabuterimon": ["H-Kabuterimon", "MetalMamemon", "Vademon", "Sukamon"],
  "Kuwagamon": ["H-Kabuterimon", "Piximon", "Vademon", "Sukamon"],
  "Vegiemon": ["Piximon", "Vademon", "Sukamon"],
  "Ninjamon": ["Piximon", "MetalMamemon", "Mamemon", "Vademon", "Sukamon"],
  "Seadramon": ["Megadramon", "MegaSeadramon", "Vademon", "Sukamon"],
  "Whamon": ["MegaSeadramon", "Mamemon", "Vademon", "Sukamon"],
  "Shellmon": ["H-Kabuterimon", "MegaSeadramon", "Vademon", "Sukamon"],
  "Coelamon": ["MegaSeadramon", "Vademon", "Sukamon"],
  "Garurumon": ["SkullGreymon", "MegaSeadramon", "Vademon", "Sukamon"],
  "Frigimon": ["MetalMamemon", "Mamemon", "Vademon", "Sukamon"],
  "Mojyamon": ["SkullGreymon", "Mamemon", "Vademon", "Sukamon"],
  "Numemon": ["Monzaemon", "Vademon", "Sukamon"],
  "Sukamon": ["Etemon", "Vademon"],
  "Nanimon": ["Digitamamon", "Vademon", "Sukamon"],
  "MetalGreymon": [],
  "Andromon": [],
  "SkullGreymon": [],
  "Megadramon": [],
  "Giromon": [],
  "Phoenixmon": [],
  "H-Kabuterimon": [],
  "Piximon": [],
  "MetalMamemon": [],
  "Mamemon": [],
  "MegaSeadramon": [],
  "Monzaemon": [],
  "Vademon": [],
  "Digitamamon": [],
  "Etemon": []
};

export const evolutionRequirements = {
  "Koromon": {},
  "Tsunomon": {},
  "Tokomon": {},
  "Tanemon": {},
  "Agumon": { hp: 10, mp: 10, off: 1, mistakes: [0, Infinity], weight: [10, 20], partner: 'Koromon' },
  "Betamon": { hp: 10, mp: 10, def: 1, mistakes: [0, Infinity], weight: [10, 20], partner: 'Tanemon' },
  "Biyomon": { mp: 10, def: 1, speed: 1, mistakes: [0, Infinity], weight: [10, 20], partner: 'Tokomon' },
  "Elecmon": { hp: 10, off: 1, speed: 1, mistakes: [0, Infinity], weight: [10, 20], partner: 'Tsunomon' },
  "Gabumon": { def: 1, speed: 1, brains: 1, mistakes: [0, Infinity], weight: [10, 20], partner: 'Koromon' },
  "Kunemon": { special: "50% chance after sleeping in Kunemon's Bed" },
  "Palmon": { mp: 10, speed: 1, brains: 1, mistakes: [0, Infinity], weight: [10, 20], partner: 'Tanemon' },
  "Patamon": { hp: 10, off: 1, brains: 1, mistakes: [0, Infinity], weight: [10, 20], partner: 'Tokomon' },
  "Penguinmon": { mp: 10, def: 1, brains: 1, mistakes: [0, Infinity], weight: [10, 20], partner: 'Tsunomon' },
  "Airdramon": { mp: 1000, speed: 100, brains: 100, mistakes: [0, 1], weight: [25, 35], discipline: 90, techs: 35 },
  "Angemon": { mp: 1000, brains: 100, mistakes: [0, 0], weight: [15, 25], techs: 35, partner: 'Patamon' },
  "Bakemon": { mp: 1000, mistakes: [3, Infinity], weight: [15, 25], happiness: 50, techs: 28 },
  "Birdramon": { speed: 100, mistakes: [3, Infinity], weight: [15, 25], techs: 35, partner: 'Biyomon' },
  "Centarumon": { brains: 100, mistakes: [0, 0], weight: [25, 35], discipline: 60, techs: 28 },
  "Coelamon": { def: 100, mistakes: [3, Infinity], weight: [25, 35], battles: [5, Infinity], techs: 35 },
  "Devimon": { special: "50% chance after losing a battle with Angemon with discipline < 50%" },
  "Drimogemon": { off: 100, mistakes: [3, Infinity], weight: [35, 45], happiness: 50, techs: 28 },
  "Frigimon": { mp: 1000, brains: 100, mistakes: [0, 5], weight: [25, 35], happiness: 50, techs: 28 },
  "Garurumon": { mp: 1000, speed: 100, mistakes: [0, 1], weight: [25, 35], discipline: 90, techs: 28 },
  "Greymon": { off: 100, def: 100, speed: 100, brains: 100, mistakes: [0, 1], weight: [25, 35], discipline: 90, techs: 35 },
  "Kabuterimon": { hp: 1000, mp: 1000, off: 100, speed: 100, mistakes: [0, 1], weight: [25, 35], techs: 35, partner: 'Kunemon' },
  "Kokatorimon": { hp: 1000, mistakes: 3, weight: [25, 35], techs: 28, partner: 'Biyomon' },
  "Kuwagamon": { hp: 1000, mp: 1000, off: 100, speed: 100, mistakes: 5, weight: [25, 35], techs: 28, partner: 'Kunemon' },
  "Leomon": { off: 100, speed: 100, brains: 100, mistakes: [0, 1], weight: [15, 25], battles: [10, Infinity], techs: 35 },
  "Meramon": { off: 100, mistakes: [5, Infinity], weight: [15, 25], battles: [10, Infinity], techs: 28 },
  "Mojyamon": { hp: 1000, mistakes: [5, Infinity], weight: [15, 25], battles: [0, 5], techs: 28 },
  "Monochromon": { hp: 1000, def: 100, brains: 100, mistakes: [0, 3], weight: [35, 45], battles: [0, 5], techs: 35 },
  "Nanimon": { special: "Scold when happiness and discipline are both 0" },
  "Ninjamon": { mp: 1000, off: 100, speed: 100, mistakes: [0, 1], weight: [5, 15], battles: [15, Infinity], techs: 35 },
  "Numemon": { special: "After 96h when requirements for other evolutions are not met" },
  "Ogremon": { hp: 1000, off: 100, mistakes: [5, Infinity], weight: [25, 35], battles: [15, Infinity], techs: 35 },
  "Seadramon": { hp: 1000, mp: 1000, mistakes: [3, Infinity], weight: [25, 35], battles: [0, 5], techs: 28 },
  "Shellmon": { hp: 1000, def: 100, mistakes: 5, weight: [35, 45], techs: 35, partner: 'Betamon' },
  "Sukamon": { special: "Fill virus bar" },
  "Tyrannomon": { hp: 1000, def: 100, mistakes: [0, 5], weight: [25, 35], battles: [0, 5], techs: 28 },
  "Unimon": { hp: 1000, speed: 100, mistakes: [0, 3], weight: [25, 35], battles: [10, Infinity], techs: 35 },
  "Vegiemon": { mp: 1000, mistakes: [5, Infinity], weight: [5, 15], happiness: 50, techs: 21 },
  "Whamon": { hp: 1000, brains: 100, mistakes: [0, 5], weight: [35, 45], discipline: 60, techs: 28 },
  "Andromon": { hp: 2000, mp: 4000, off: 200, def: 400, speed: 200, brains: 400, mistakes: [0, 5], weight: [35, 45], discipline: 95, battles: [30, Infinity], techs: 30 },
  "Digitamamon": { hp: 3000, mp: 3000, off: 400, def: 400, speed: 400, brains: 300, mistakes: [0, 0], weight: [5, 15], battles: [100, Infinity], techs: 49 },
  "Etemon": { hp: 2000, mp: 3000, off: 400, def: 200, speed: 400, brains: 300, mistakes: [0, 0], weight: [10, 20], battles: [50, Infinity], techs: 49 },
  "Giromon": { off: 400, speed: 300, brains: 400, mistakes: [15, Infinity], weight: [0, 10], happiness: 95, battles: [100, Infinity], techs: 35 },
  "H-Kabuterimon": { hp: 7000, def: 400, speed: 600, brains: 400, mistakes: [0, 55], weight: [0, 10], battles: [0, 0], techs: 40 },
  "Mamemon": { off: 400, def: 300, speed: 300, brains: 400, mistakes: [15, Infinity], weight: [0, 10], happiness: 90, techs: 25 },
  "Megadramon": { hp: 3000, mp: 5000, off: 500, def: 300, speed: 400, brains: 400, mistakes: [50, 60], weight: [0, 10], battles: [30, Infinity], techs: 30 },
  "MegaSeadramon": { mp: 4000, off: 500, def: 400, brains: 400, mistakes: [0, 5], weight: [25, 35], battles: [0, 0], techs: 40 },
  "MetalGreymon": { hp: 4000, mp: 3000, off: 500, def: 500, speed: 300, brains: 300, mistakes: [0, 10], weight: [60, 70], discipline: 95, battles: [30, Infinity], techs: 30 },
  "MetalMamemon": { off: 500, def: 400, speed: 400, brains: 400, mistakes: [0, 15], weight: [5, 15], happiness: 95, techs: 30 },
  "Monzaemon": { hp: 3000, mp: 3000, off: 300, def: 300, speed: 300, brains: 300, mistakes: [0, Infinity], weight: [35, 45], battles: [50, Infinity], techs: 49 },
  "Phoenixmon": { hp: 4000, mp: 4000, speed: 400, brains: 600, mistakes: [0, 3], weight: [25, 35], discipline: 100, battles: [0, 0], techs: 40 },
  "Piximon": { off: 300, def: 300, speed: 400, brains: 400, mistakes: [15, Infinity], weight: [0, 10], happiness: 95, techs: 25 },
  "SkullGreymon": { hp: 4000, mp: 6000, off: 400, def: 400, speed: 200, brains: 500, mistakes: [10, Infinity], weight: [25, 35], battles: [40, Infinity], techs: 45 },
  "Vademon": { special: "After 240h when praising or scolding" },
}

export function countTechs(techs) {
  return countSetBits(techs.fire_techs) +
  countSetBits(techs.air_techs) +
  countSetBits(techs.ice_techs) +
  countSetBits(techs.mech_techs) +
  countSetBits(techs.earth_techs) +
  countSetBits(techs.battle_techs) +
  countSetBits(techs.filth_techs) +
  countSetBits(techs.filth_techs_ext & 0xF);
}

export function timeUntilEvolution(evoTimer, digimonLevel) {
  if (digimonLevel == 1) return 6 - evoTimer;
  if (digimonLevel == 2) return 24 - evoTimer;
  if (digimonLevel == 3) return 72 - evoTimer;
  if (digimonLevel == 4) return 144 - evoTimer;

  return Infinity;
}

function countSetBits(num) {
  let count = 0;
  while (num) {
    count += num & 1;
    num >>= 1;
  }
  return count;
}
