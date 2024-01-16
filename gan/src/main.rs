use futures_util::StreamExt;
use gan::{
    AppArgs, AppResult, AutoCfg, Comfy, Generator, StatusMsg, Workflow, WsMsg, NODE_KSAMPLER,
};

use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, info, trace, warn};
use uuid::Uuid;

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt::init();
    let args = AppArgs::default();
    trace!("{args:?}");

    let client_id = Uuid::new_v4().to_string();
    let ws_url = format!("ws://{}/ws?clientId={client_id}", args.comfy_host);
    info!("ws_url: {ws_url}");
    let mut gen: Generator = Generator::new();

    let (ws_stream, _) = connect_async(ws_url).await?;
    let (_, mut read) = ws_stream.split();

    let api = Comfy::new(args.comfy_host.as_str(), client_id.as_str());
    let ac = AutoCfg::from_file(&args.auto_cfg)?;
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
                        let mut wf = Workflow::from_file(args.workflow.as_str())?;
                        let _ = gen.rand(&mut wf, &ac)?;
                        for _ in 0..ac.ct_per_params {
                            wf.get_node_mut(NODE_KSAMPLER)?.k_sampler_mut().seed =
                                rand::random::<u32>() as i64;
                            let prompt = wf.to_json()?;
                            api.queue_prompt(&prompt).await;
                        }
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
