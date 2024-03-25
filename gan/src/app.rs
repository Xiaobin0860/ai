use std::{ops::Deref, sync::Arc};

use anyhow::Context;
use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{AppCfg, AppResult, AppState, AutoCfg, Comfy, Generator, StatusMsg, Workflow, WsMsg};

pub struct State {
    pub db: AppState,
    pub cfg: AppCfg,
}

impl State {
    pub fn get_auto_cfg(&self, cfg_idx: usize) -> Option<&str> {
        self.cfg.auto_cfgs.get(cfg_idx).map(|s| s.as_str())
    }

    pub fn get_total_idx(&self) -> usize {
        self.db.get_total_idx()
    }

    pub fn get_cfg_idx(&self) -> usize {
        self.db.get_cfg_idx()
    }

    pub fn incr_total_idx(&self) -> usize {
        self.db.incr_total_idx()
    }

    pub fn incr_cfg_idx(&self) -> usize {
        self.db.incr_cfg_idx()
    }

    pub fn reset_total_idx(&self) {
        self.db.set_total_idx(0);
    }

    pub fn reset_cfg_idx(&self) {
        self.db.set_cfg_idx(0);
    }

    pub fn save_db(&self) {
        if let Err(e) = self.db.save() {
            warn!("save db error: {e:?}");
        }
    }
}

pub struct App(Arc<State>);

impl Deref for App {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl App {
    pub fn new(cfg_file: &str, clean: bool) -> AppResult<App> {
        let cfg = AppCfg::from_file(cfg_file)?;
        let db_file = cfg_file.replace(".toml", ".db");
        let db = AppState::from_file(&db_file)?;
        if clean {
            warn!("clean db");
            db.clean();
        }
        let app_state = State { db, cfg };
        Ok(App(Arc::new(app_state)))
    }

    pub async fn run(&self) -> AppResult<()> {
        let client_id = Uuid::new_v4().to_string();
        let api = self.cfg.comfy_api.as_str();
        let ws_url = format!("ws://{api}/ws?clientId={client_id}");
        info!("ws_url: {ws_url}");
        let gen = Generator::new();

        let (ws_stream, _) = connect_async(ws_url).await?;
        let (_, mut read) = ws_stream.split();

        let api = Comfy::new(api, client_id.as_str());
        let cfg_idx = self.get_cfg_idx();
        let auto_cfg = self
            .get_auto_cfg(cfg_idx)
            .context(format!("get auto_cfg {cfg_idx}"))?;
        let total_idx = self.get_total_idx();
        info!("auto_cfg{cfg_idx} {auto_cfg}, total_idx={total_idx}");
        let mut ac = AutoCfg::from_file(auto_cfg)?;
        while let Some(msg) = read.next().await {
            let msg = msg?;
            match msg {
                Message::Text(text) => {
                    debug!("text: {text}");
                    let msg: WsMsg = serde_json::from_str(text.as_str())?;
                    if msg.typ == "status" {
                        let status: StatusMsg = serde_json::from_value(msg.data)?;
                        let remaining = status.status.exec_info.queue_remaining;
                        info!("remaining: {remaining}");
                        if remaining == 0 {
                            self.do_gen(&ac, &api, &gen, total_idx).await?;
                            let total_idx = self.incr_total_idx();
                            if total_idx >= ac.total {
                                self.reset_total_idx();
                                let cfg_idx = self.incr_cfg_idx();
                                if let Some(nxt_ac) = self.get_auto_cfg(cfg_idx) {
                                    ac = AutoCfg::from_file(nxt_ac)?;
                                } else {
                                    warn!("no more auto_cfg, exit");
                                    self.save_db();
                                    break;
                                }
                            }
                            self.save_db();
                        }
                    }
                }
                Message::Close(_) => {
                    info!("comfy ws closed");
                    break;
                }
                _ => {
                    warn!("unhandled: {msg}");
                }
            }
        }

        Ok(())
    }

    async fn do_gen(
        &self,
        ac: &AutoCfg,
        api: &Comfy,
        gen: &Generator,
        idx: usize,
    ) -> AppResult<()> {
        // 对每个流程, 随机参数, 生`ct_per_params`次, 即每次空闲跑`流程数*ct_per_params`个图
        for wf in ac.workflows.iter() {
            let mut wf = Workflow::from_file(wf.as_str())?;
            gen.rand(&mut wf, ac, idx)?;
            for _ in 0..ac.ct_per_params {
                wf.set_seed(rand::random::<u32>() as i64)?;
                let prompt = wf.to_json()?;
                api.queue_prompt(&prompt).await;
            }
        }
        Ok(())
    }
}
