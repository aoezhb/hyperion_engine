pub mod gpu;

#[allow(dead_code)]
pub struct HardwareStatus {
    pub gpu_model: String,
    pub vram_gb: u32,
}

pub async fn detect_hardware() -> anyhow::Result<HardwareStatus> {
    gpu::detect_gpu().await
}
