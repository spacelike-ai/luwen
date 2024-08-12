use luwen_if::{chip::ArcMsgOptions, ArcMsg, ArcMsgOk, ChipImpl, TypedArcMsg};
use ttkmd_if::PciDevice;

fn main() {
    for id in PciDevice::scan() {
        let mut raw_device = PciDevice::open(id).unwrap();
        let device = luwen_ref::open(id).unwrap();

        if let Some(wh) = device.as_wh() {
            let dump_addr = if let Ok(result) = wh.arc_msg(ArcMsgOptions {
                msg: ArcMsg::Typed(TypedArcMsg::GetSpiDumpAddr),
                ..Default::default()
            }) {
                match result {
                    ArcMsgOk::Ok { rc: _, arg } => Some(arg),
                    ArcMsgOk::OkNoWait => None,
                }
            } else {
                None
            }
            .unwrap();

            let csm_offset =
                wh.arc_if.axi_translate("ARC_CSM.DATA[0]").unwrap().addr - 0x10000000_u64;

            let addr = csm_offset + (dump_addr as u64);

            let aligned_addr = (addr + 3) & !3;

            raw_device.write32(aligned_addr as u32, 0xfaca).unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0xfaca, "{:x} != faca", readback);

            raw_device
                .write32(aligned_addr as u32, 0xcdcd_cdcd)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

            raw_device
                .write32(aligned_addr as u32 + 4, 0xcdcd_cdcd)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32 + 4).unwrap();
            assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

            raw_device.write32(aligned_addr as u32 + 1, 0xdead).unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0xdeadcd, "{:x} != deadcd", readback);
            let readback = raw_device.read32(aligned_addr as u32 + 4).unwrap();
            assert_eq!(readback, 0xcdcdcd00, "{:x} != 00cdcdcd", readback);

            raw_device
                .write32(aligned_addr as u32, 0xcdcd_cdcd)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

            raw_device
                .write32(aligned_addr as u32 + 4, 0xcdcd_cdcd)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32 + 4).unwrap();
            assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

            raw_device
                .write32(aligned_addr as u32 + 3, 0xc0ffe)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0xfecdcdcd, "{:x} != fecdcdcd", readback);
            let readback = raw_device.read32(aligned_addr as u32 + 4).unwrap();
            assert_eq!(readback, 0xcd000c0f, "{:x} != c0f", readback);

            raw_device.write32(aligned_addr as u32, 0x01234567).unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0x01234567, "{:x} != 01234567", readback);

            raw_device
                .write32(aligned_addr as u32 + 4, 0xabcdef)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32 + 4).unwrap();
            assert_eq!(readback, 0xabcdef, "{:x} != abcdef", readback);

            let readback = raw_device.read32(aligned_addr as u32 + 1).unwrap();
            assert_eq!(readback, 0xef012345, "{:x} != ef012345", readback);

            let readback = raw_device.read32(aligned_addr as u32 + 3).unwrap();
            assert_eq!(readback, 0xabcdef01, "{:x} != abcdef01", readback);
        }
    }
}
