use libvgm_sys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let dev_cfg = libvgm_sys::DEV_GEN_CFG {
            emuCore: 1,
            srMode: libvgm_sys::DEVRI_SRMODE_NATIVE as u8,
            clock: 357945,
            smplRate: 44100,
            flags: 0
        };
        assert_eq!(dev_cfg.emuCore, 1);
    }
}
