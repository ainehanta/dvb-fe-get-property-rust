#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use nix::{
    fcntl::{open, OFlag},
    ioctl_read,
};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// #define FE_GET_PROPERTY		   _IOR('o', 83, struct dtv_properties)
ioctl_read!(fe_get_property, b'o', 83, dtv_properties);

fn main() {
    let fd = open(
        "/dev/dvb/adapter2/frontend0",
        OFlag::O_RDWR,
        nix::sys::stat::Mode::empty(),
    )
    .unwrap();

    let mut props: [dtv_property; 4] = unsafe { std::mem::zeroed() };
    props[0].cmd = DTV_STAT_CNR;
    props[1].cmd = DTV_STAT_ERROR_BLOCK_COUNT;
    props[2].cmd = DTV_STAT_TOTAL_BLOCK_COUNT;
    props[3].cmd = DTV_STAT_SIGNAL_STRENGTH;

    let mut properties = dtv_properties {
        num: 4,
        props: props.as_mut_ptr(),
    };
    let properties_ptr = &mut properties as *mut dtv_properties;

    let result = unsafe { fe_get_property(fd, properties_ptr) };
    println!("result: {result:?}");
    unsafe {
        println!("DTV_STAT_CNR: {:?}", props[0].u.st.stat[0]);
        println!("DTV_STAT_ERROR_BLOCK_COUNT: {:?}", props[1].u.st.stat[0]);
        println!("DTV_STAT_TOTAL_BLOCK_COUNT: {:?}", props[2].u.st.stat[0]);
        println!("DTV_STAT_SIGNAL_STRENGTH: {:?}", props[3].u.st.stat[0]);
    }
}

impl std::fmt::Debug for dtv_fe_stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let f = f
            .debug_struct("dtv_fe_stats")
            .field("len", &self.len)
            .field("stat", &self.stat)
            .finish();
        return f;
    }
}

impl std::fmt::Debug for dtv_stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let mut f = f.debug_struct("dtv_stats");

            match self.scale.into() {
                fecap_scale_params_FE_SCALE_NOT_AVAILABLE => {
                    f.field("scale", &"FE_SCALE_NOT_AVAILABLE")
                }
                fecap_scale_params_FE_SCALE_DECIBEL => f.field("scale", &"FE_SCALE_DECIBEL").field(
                    "value",
                    &format!("{:?}dB", self.__bindgen_anon_1.svalue / 1000),
                ),
                fecap_scale_params_FE_SCALE_RELATIVE => {
                    f.field("scale", &"FE_SCALE_RELATIVE").field(
                        "value",
                        &format!("{:?}%", self.__bindgen_anon_1.uvalue as f64 / 655.35),
                    )
                }
                fecap_scale_params_FE_SCALE_COUNTER => f.field("scale", &"FE_SCALE_COUNTER").field(
                    "value",
                    &format!("{:?}", self.__bindgen_anon_1.uvalue as u64),
                ),
                _ => f.field("scale", &format!("{:?}", self.scale)),
            }
            .finish()
        }
    }
}
