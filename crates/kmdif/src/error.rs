use thiserror::Error;

#[derive(Error, Debug)]
pub enum CfgFailType {
    #[error("Nix error: {0}")]
    Nix(#[from] nix::Error),

    #[error("Size mismiatch: recieved {0} bytes")]
    SizeMismatch(usize)
}

#[derive(Error, Debug)]
pub enum PciOpenError {
    #[error("Failed to open device /dev/tenstorrent/{id}: {source}")]
    DeviceOpenFailed{id: usize, source: std::io::Error},

    #[error("ioctl {name} failed for device {id} with: {source}")]
    IoctlError{ name: String, id: usize, source: nix::Error},

    #[error("Failed to map {name} from device {id}")]
    BarMappingError {name: String, id: usize},

    #[error("When creating anon buffer {buffer} for device {device_id} hit error {source}")]
    FakeMmapFailed {
        buffer: String,
        device_id: usize,
        source: std::io::Error
    },

    #[error(transparent)]
    PciError(#[from] PciError)
}

#[derive(Error, Debug)]
pub enum PciError {
    #[error("DMA buffer mapping failed for device {id} with error {source}")]
    DmaBufferMappingFailed {
        id: usize,
        source: std::io::Error
    },

    #[error("DMA for device {id} is not configured")]
    DmaNotConfigured { id: usize },

    #[error("On device {id} tried to use 64-bit DMA, but ARC fw does not support it")]
    No64bitDma { id: usize },

    #[error("On device {id} tried to write {size} bytes, but DMA only allows a max of 28 bits")]
    DmaTooLarge {
        id: usize,
        size: usize,
    },

    #[error("Read 0xffffffff from ARC scratch[6]: you should reset the board.")]
    BrokenConnection,

    #[error("Failed to read from device {id} config space[offset: {offset}, size: {size}]; Failed with {source}")]
    CfgReadFailed {
        id: usize,
        offset: usize,
        size: usize,

        source: CfgFailType,
    },
    #[error("Failed to write into device {id} config space[offset: {offset}, size: {size}]; Failed with {source}")]
    CfgWriteFailed {
        id: usize,
        offset: usize,
        size: usize,

        source: CfgFailType,
    }
}
