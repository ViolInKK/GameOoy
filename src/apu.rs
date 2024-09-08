struct Channel;

pub struct Apu {
    CH1: Channel,
    CH2: Channel,
    CH3: Channel,
    CH4: Channel,
}

impl Apu {
    pub fn new() -> Apu {
        Apu {
            CH1: Channel,
            CH2: Channel,
            CH3: Channel,
            CH4: Channel,
        }

    }
}
