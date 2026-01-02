use std::env;
use std::sync::{Arc, LazyLock, RwLock};
use std::thread;
use libretro_proxy::Core as LibretroCore ;
use libretro_proxy::Proxy as LibretroProxy;

mod rpc_server;

type SharedRomData = Arc<RwLock<Option<&'static [u8]>>>;

static STATE: LazyLock<ProxyState> = LazyLock::new(|| {
    let core_path = env::var("RETRO_SCRIPTING_CORE_PATH")
        .expect("[RetroScripting] Core path not specified. Please set the RETRO_SCRIPTING_CORE_PATH env var before running.");

    let core = LibretroCore::load(&core_path).unwrap_or_else(|| {panic!("Failed to load core from path: {}", &core_path) });
    let rom = Arc::new(RwLock::new(None));

    ProxyState {core, rom}
});

struct ProxyState {
    core: LibretroCore,
    rom: SharedRomData,
}

impl ProxyState {
    pub fn core(&self) -> &LibretroCore {
        &self.core
    }

    fn memory(&self) -> &[u8] {
        let memory_ptr = self.core.retro_get_memory_data(2);
        let memory_size = self.core.retro_get_memory_size(2);
        unsafe {
            std::slice::from_raw_parts(memory_ptr as *const u8, memory_size)
        }
    }

    pub fn rom(&self) -> &[u8] {
        self.rom.read().unwrap().unwrap()
    }
}

impl LibretroProxy for ProxyState {
    fn core(&self) -> &LibretroCore {
        &self.core
    }

    fn retro_load_game(&self, game: *const libretro_proxy::sys::retro_game_info) -> bool {
        let loaded = self.core().retro_load_game(game);

        if loaded {
            let game_info = unsafe { &*game };
            if !game_info.data.is_null() {
                let data = unsafe { std::slice::from_raw_parts(game_info.data as *const u8, game_info.size) };
                let mut rom = self.rom.write().unwrap();
                *rom = Some(data);
            }

            let _ = &*SERVER;
        }

        loaded
    }
}

static SERVER: LazyLock<()> = LazyLock::new(|| {
    println!("spawning!");

    thread::spawn(|| {
        rpc_server::handler(core_get_memory_data, core_get_rom_data);
    });
});

fn core_get_memory_data(address: usize) -> u8 {
    let state = &*STATE;
    state.memory()[address]
}

fn core_get_rom_data(address: usize) -> u8 {
    let state = &*STATE;
    state.rom()[address]
}

libretro_proxy::proxy_to!(&*STATE);
