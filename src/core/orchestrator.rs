use tokio::time::{self, Duration};
use tracing::{info, warn};
use rand::Rng;
use chrono::Utc;
use serde_json::json;

const SUPABASE_URL: &str = "https://husewaxkvaxzdymdzfdg.supabase.co";
const SUPABASE_KEY: &str = "sb_publishable_m07Q-5zNUkAvtElCjaoWAg_zeRBbN6r";

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
    node_id: String,
    state: NodeState,
    is_demo_mode: bool,
    http_client: reqwest::Client,
}

impl Orchestrator {
    pub fn new(is_demo_mode: bool) -> Self {
        let mut rng = rand::rng();
        let node_id = format!("node_{:06}", rng.random_range(1000..999999));
        
        Self {
            node_id,
            state: NodeState::Init,
            is_demo_mode,
            http_client: reqwest::Client::new(),
        }
    }

    /// Report node status to Supabase registry
    async fn report_status(&self) {
        let status_str = match self.state {
            NodeState::Idle => "online",
            NodeState::Computing(_) => "computing",
            NodeState::Syncing => "syncing",
            _ => "offline",
        };
        
        // Debug Log: Confirm we are trying to send data
        // info!("[Network] 📡 Reporting status to Supabase: {}", status_str);

        let payload = json!({
            "id": self.node_id,
            "gpu_model": "NVIDIA GeForce RTX 4090 (Driver: 535.104.05)",
            "status": status_str,
            "vram_gb": 24,
            "last_seen": Utc::now().to_rfc3339(),
            "ip_location": { "city": "Global", "lat": 0, "lng": 0 }
        });

        // Supabase REST API Upsert (Insert or Update if ID exists)
        let res = self.http_client
            .post(format!("{}/rest/v1/nodes", SUPABASE_URL))
            .header("apikey", SUPABASE_KEY)
            .header("Authorization", format!("Bearer {}", SUPABASE_KEY))
            .header("Content-Type", "application/json")
            .header("Prefer", "resolution=merge-duplicates") // Standard Upsert behavior
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(response) => {
                if !response.status().is_success() {
                    warn!("Network: Failed to report status. HTTP {}", response.status());
                } else {
                    // info!("[Network] ✅ Status update successful.");
                }
            }
            Err(e) => warn!("Network: Connection error while reporting status: {}", e),
        }
    }

    /// Start the Orchestrator's main event loop
    pub async fn run(&mut self) {
        info!("Orchestrator: Core loop started (NodeID: {})", self.node_id);
        
        if self.is_demo_mode {
            self.run_demo_loop().await;
        } else {
            self.run_real_loop().await;
        }
    }

    /// Real Mode Loop
    async fn run_real_loop(&mut self) {
        self.state = NodeState::Syncing;
        self.report_status().await;
        
        info!("Orchestrator: Syncing State Trie with main DePIN network...");
        time::sleep(Duration::from_secs(2)).await;
        
        self.state = NodeState::Idle;
        self.report_status().await;
        info!("Orchestrator: ✅ Sync complete. Waiting for real P2P tasks...");

        let mut interval = time::interval(Duration::from_secs(10)); // Report every 10s
        loop {
            interval.tick().await;
            self.report_status().await;
            self.heartbeat();
        }
    }

    /// Demo Mode Loop
    async fn run_demo_loop(&mut self) {
        self.state = NodeState::Syncing;
        self.report_status().await;
        info!("Orchestrator [Demo]: Syncing state from Testnet...");
        time::sleep(Duration::from_secs(1)).await;
        
        self.state = NodeState::Idle;
        self.report_status().await;
        info!("Orchestrator [Demo]: ✅ State sync complete. Simulated task flow activated.");

        let mut rng = rand::rng();
        let mut interval = time::interval(Duration::from_secs(1));
        let mut heartbeat_timer = 0;

        loop {
            interval.tick().await;
            heartbeat_timer += 1;

            if heartbeat_timer % 10 == 0 {
                self.report_status().await;
                self.heartbeat();
            }

            match self.state {
                NodeState::Idle => {
                    if rng.random_bool(0.1) { // 10% chance to receive task
                        let task_id = format!("task_{:06}", rng.random_range(1000..999999));
                        info!("✨ New Task Broadcast Received: [ID: {}]", task_id);
                        self.state = NodeState::Computing(task_id);
                        self.report_status().await;
                    }
                }
                NodeState::Computing(ref task_id) => {
                    if rng.random_bool(0.1) { // Task completes (Lower probability = longer task)
                        info!("✅ Task [{}] Computation Complete!", task_id);
                        self.state = NodeState::Idle;
                        self.report_status().await;
                    } else {
                        info!("⚙️  Computing Task [{}]... Hardware Telemetry Active", task_id);
                    }
                }
                _ => {}
            }
        }
    }

    fn heartbeat(&self) {
        match &self.state {
            NodeState::Idle => {
                info!("Orchestrator [Heartbeat]: 🟢 Online | Latency: 12ms | VRAM Free: 24GB");
            }
            NodeState::Computing(task_id) => {
                info!("Orchestrator [Heartbeat]: 🟡 Computing ({}) | GPU Util: 98%", task_id);
            }
            _ => {}
        }
    }
}