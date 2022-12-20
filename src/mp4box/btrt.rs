use crate::base_box;

base_box! {
    box (b"btrt", Btrt, BtrtBox)
    data {
        buffer_size_db: u32,
        max_bitrate: u32,
        avg_bitrate: u32
    }
}
