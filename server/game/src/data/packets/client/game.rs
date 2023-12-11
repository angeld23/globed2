use crate::data::*;

#[derive(Packet, Decodable)]
#[packet(id = 12000, encrypted = false)]
pub struct RequestPlayerProfilesPacket {
    pub requested: i32, // 0 to get all ppl on the level
}

#[derive(Packet, Decodable)]
#[packet(id = 12001, encrypted = false)]
pub struct LevelJoinPacket {
    pub level_id: i32,
}

#[derive(Packet, Decodable)]
#[packet(id = 12002, encrypted = false)]
pub struct LevelLeavePacket;

#[derive(Packet, Decodable)]
#[packet(id = 12003, encrypted = false)]
pub struct PlayerDataPacket {
    pub data: PlayerData,
}

#[derive(Packet, Decodable)]
#[packet(id = 12010, encrypted = true)]
pub struct VoicePacket {
    pub data: FastEncodedAudioFrame,
}

#[derive(Packet, Decodable)]
#[packet(id = 12011, encrypted = true)]
pub struct ChatMessagePacket {
    pub message: FastString<MAX_MESSAGE_SIZE>,
}
