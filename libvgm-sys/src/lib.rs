#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::default;
    #[test]
    fn test_it_works() {
        let dev_cfg = DEV_GEN_CFG {
            emuCore: 1,
            srMode: DEVRI_SRMODE_NATIVE as u8,
            clock: 357945,
            smplRate: 44100,
            flags: 0
        };
        assert_eq!(dev_cfg.emuCore, 1);
    }
    #[test]
    fn test_loader () {
        //PlayerBase* player;
        let loader: DATA_LOADER = Default::default();
    }
}
