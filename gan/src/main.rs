use anyhow::Context;
use futures_util::StreamExt;
use gan::{
    AppArgs, AppCfg, AppResult, AppState, AutoCfg, Comfy, Generator, StatusMsg, Workflow, WsMsg,
};

use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, info, trace, warn};
use uuid::Uuid;

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt::init();
    let args = AppArgs::default();
    trace!("{args:?}");
    let cfg = args.cfg.as_str();
    let app_cfgs = AppCfg::from_file(cfg)?;
    let db = cfg.replace(".toml", ".db");
    let mut db = AppState::from_file(db.as_str())?;
    if args.clean {
        warn!("clean db");
        db.clean();
    }

    let client_id = Uuid::new_v4().to_string();
    let ws_url = format!("ws://{}/ws?clientId={client_id}", app_cfgs.comfy_api);
    info!("ws_url: {ws_url}");
    let gen: Generator = Generator::new();

    let (ws_stream, _) = connect_async(ws_url).await?;
    let (_, mut read) = ws_stream.split();

    let api = Comfy::new(app_cfgs.comfy_api.as_str(), client_id.as_str());
    let mut cfg_idx = db.cfg_idx;
    let mut total_idx = db.total_idx;
    let auto_cfg = app_cfgs
        .auto_cfgs
        .get(cfg_idx)
        .context(format!("get auto_cfg {cfg_idx}"))?;
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
                        do_gen(&ac, &api, &gen, total_idx).await?;
                        total_idx += 1;
                        if total_idx >= ac.total {
                            total_idx = 0;
                            cfg_idx += 1;
                            if let Some(nxt_ac) = app_cfgs.auto_cfgs.get(cfg_idx) {
                                ac = AutoCfg::from_file(nxt_ac)?;
                            } else {
                                warn!("no more auto_cfg, exit");
                                let _ = db.update(cfg_idx, total_idx);
                                break;
                            }
                        }
                        db.update(cfg_idx, total_idx)?;
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

async fn do_gen(ac: &AutoCfg, api: &Comfy, gen: &Generator, idx: usize) -> AppResult<()> {
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
