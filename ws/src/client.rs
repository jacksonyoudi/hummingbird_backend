use std::sync::{Arc};
use axum::extract::ws::Message;
use futures::SinkExt;
use futures::stream::SplitSink;
use tokio::sync::RwLock;


// 定义一个类型别名，用于表示客户端发送消息的通道
// ClientSender是一个类型别名，它是Arc和RwLock的嵌套用法，Arc保证跨多个异步任务共享，
// 并且RwLock使得同时只有一个任务能够写入数据到WebSocket， 注意RwLock是tokio::sync::RwLock。
// SplitSink来自futures库，它是一个WebSocket的一个分离的发送半部分，用于处理发送消息的能力。
// 这允许WebSocket流同时进行读和写操作，而不会相互阻塞。
type ClientSender = Arc<RwLock<SplitSink<axum::extract::ws::WebSocket, Message>>>;


pub struct Client {
    // 客户端发送消息的通道
    pub sender: ClientSender,
    // 用户的唯一标识符。
    pub user_id: String,
    // 用于识别用户的平台（设备）ID。
    pub platform_id: String,
}

impl Client {
    pub async fn send_text(&self, msg: String) -> Result<(), axum::Error> {
        self.sender.write().await.send(Message::Text(msg)).await
    }

    pub async fn send_binary(&self, msg: Vec<u8>) -> Result<(), axum::Error> {
        self.sender.write().await.send(Message::Binary(msg)).await
    }
}