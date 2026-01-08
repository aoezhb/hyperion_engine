use tokio::time::{self, Duration};
use tracing::info;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum NodeState {
    #[allow(dead_code)]
    Init,
    Syncing,
    Idle,
    #[allow(dead_code)]
    Computing(String), // Task ID
}

pub struct Orchestrator {
    state: NodeState,
    is_demo_mode: bool,
}

impl Orchestrator {
    pub fn new(is_demo_mode: bool) -> Self {
        Self {
            state: NodeState::Init,
            is_demo_mode,
        }
    }

    /// Start the Orchestrator's main event loop
    pub async fn run(&mut self) {
        info!("Orchestrator: Core loop started");
        
        if self.is_demo_mode {
            self.run_demo_loop().await;
        } else {
            self.run_real_loop().await;
        }
    }

    /// Real Mode Loop (Placeholder for production logic)
    async fn run_real_loop(&mut self) {
        self.state = NodeState::Syncing;
        info!("Orchestrator: Syncing State Trie with main DePIN network...");
        time::sleep(Duration::from_secs(2)).await;
        
        self.state = NodeState::Idle;
        info!("Orchestrator: ✅ Sync complete. Waiting for real P2P tasks...");

        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            self.heartbeat();
        }
    }

    /// Demo Mode Loop (Simulated High-Frequency Task Flow)
    async fn run_demo_loop(&mut self) {
        // 1. Fast Sync Simulation
        self.state = NodeState::Syncing;
        info!("Orchestrator [Demo]: Syncing state from Testnet...");
        time::sleep(Duration::from_secs(1)).await;
        self.state = NodeState::Idle;
        info!("Orchestrator [Demo]: ✅ State sync complete. Simulated task flow activated.");

        let mut rng = rand::rng();
        let mut interval = time::interval(Duration::from_secs(1));
        let mut heartbeat_timer = 0;

        loop {
            interval.tick().await;
            heartbeat_timer += 1;

            // Log heartbeat every 5 seconds
            if heartbeat_timer % 5 == 0 {
                self.heartbeat();
            }

            match self.state {
                NodeState::Idle => {
                    // 20% chance to receive a new task
                    if rng.random_bool(0.2) {
                        let task_id = format!("task_{:06}", rng.random_range(1000..999999));
                        let task_type = if rng.random_bool(0.5) { "ZK-Rollup Proof" } else { "LLM Inference" };
                        
                        info!("✨ New Task Broadcast Received: [ID: {}] Type: {}", task_id, task_type);
                        info!("   -> Verifying task signature... Valid");
                        info!("   -> Checking resource requirements... VRAM: 12GB (Available)");
                        info!("   -> Locking GPU resources... Done");
                        
                        self.state = NodeState::Computing(task_id);
                    }
                }
                NodeState::Computing(ref task_id) => {
                    // Simulate computation progress
                    if rng.random_bool(0.3) {
                        // 30% chance task completes
                        info!("✅ Task [{}] Computation Complete!", task_id);
                        
                        // Simulate Proof Generation
                        info!("   -> Generating PoPW (Proof of Physical Work)...");
                        info!("   -> Invoking TEE Secure Module... Signature: 0x7f3a...b91c");
                        
                        // Simulate Reward
                        let reward = rng.random_range(5.5..15.0);
                        info!("💰 Submitting result to on-chain contract... Success! Reward: {:.2} HYP", reward);
                        
                        self.state = NodeState::Idle;
                    } else {
                        // Still computing, log hardware telemetry
                        let gpu_temp = rng.random_range(65..82);
                        let power = rng.random_range(250..400);
                        info!("⚙️  Computing Task [{}]... GPU Temp: {}°C | Power: {}W | Tensor Cores: Active", task_id, gpu_temp, power);
                    }
                }
                _ => {}
            }
        }
    }

    fn heartbeat(&self) {
        match &self.state {
            NodeState::Idle => {
                info!("Orchestrator [Heartbeat]: 🟢 Online | Latency: 12ms | VRAM Free: 24GB | Pending Queue: 0");
            }
            NodeState::Computing(task_id) => {
                info!("Orchestrator [Heartbeat]: 🟡 Computing (Task: {}) | GPU Util: 98% | VRAM Used: 18GB", task_id);
            }
            NodeState::Syncing => {
                info!("Orchestrator [Heartbeat]: 🔵 Syncing blocks...");
            }
            _ => {}
        }
    }
}
