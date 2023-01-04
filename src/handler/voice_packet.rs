use crate::client::Client;
use crate::error::MumbleError;
use crate::handler::Handler;
use crate::sync::RwLock;
use crate::voice::{Clientbound, VoicePacket};
use crate::ServerState;
use async_trait::async_trait;
use futures_util::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
impl Handler for VoicePacket<Clientbound> {
    async fn handle(&self, state: Arc<RwLock<ServerState>>, client: Arc<RwLock<Client>>) -> Result<(), MumbleError> {
        let mute = { client.read_err().await?.mute };

        if mute {
            return Ok(());
        }

        if let VoicePacket::<Clientbound>::Audio { target, session_id, .. } = self {
            let mut listening_clients = HashMap::new();

            match *target {
                // Channel
                0 => {
                    let channel_id = { client.read_err().await?.channel_id };
                    let channel_result = { state.read_err().await?.channels.get(&channel_id).cloned() };

                    if let Some(channel) = channel_result {
                        {
                            listening_clients.extend(channel.read_err().await?.get_listeners(state.clone()).await);
                        }
                    }
                }
                // Voice target (whisper)
                1..=30 => {
                    let target = { client.read_err().await?.get_target((*target - 1) as usize) };

                    if let Some(target) = target {
                        let target = target.read_err().await?;

                        for client_id in &target.sessions {
                            let client_result = { state.read_err().await?.clients.get(client_id).cloned() };

                            if let Some(client) = client_result {
                                listening_clients.insert(*client_id, client);
                            }
                        }

                        for channel_id in &target.channels {
                            let channel_result = { state.read_err().await?.channels.get(channel_id).cloned() };

                            if let Some(channel) = channel_result {
                                {
                                    listening_clients.extend(channel.read_err().await?.get_listeners(state.clone()).await);
                                }
                            }
                        }
                    }
                }
                // Loopback
                31 => {
                    {
                        client.read_err().await?.send_voice_packet(self).await?;
                    }

                    return Ok(());
                }
                _ => {
                    tracing::error!("invalid voice target: {}", *target);
                }
            }

            // Concurrent voice send
            futures_util::stream::iter(listening_clients)
                .for_each_concurrent(None, |(id, listening_client)| async move {
                    {
                        if id == *session_id {
                            return;
                        }

                        match listening_client.read_err().await {
                            Ok(listening_client) => match listening_client.send_voice_packet(self).await {
                                Ok(_) => (),
                                Err(err) => {
                                    let username = listening_client.authenticate.get_username();

                                    tracing::error!("failed to send voice packet to client {} - {}: {}", id, username, err)
                                }
                            },
                            Err(err) => tracing::error!("failed to send voice packet to client, lock error for {}: {}", id, err),
                        }
                    }
                })
                .await;
        }

        Ok(())
    }
}
