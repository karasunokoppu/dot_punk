use bevy::prelude::*;

// NPCタイプがNPCType::GenericのNPCに話しかけた際の会話
#[derive(Component, PartialEq, Eq)]
pub struct TalkDialog {
    pub id: u32, //会話のユニークID
    pub dialog: Vec<TalkElement>,
}
impl Default for TalkDialog {
    fn default() -> Self {
        TalkDialog {
            id: 0,
            dialog: vec![
                TalkElement {
                    local_talk_id: 0,
                    element_type: TalkElementType::Text(TalkTextElement {
                        talker: Talkers::Player,
                        text: "Hello, This is talk test 0-0".to_string(),
                        next_talk_element_id: 1,
                    }),
                },
                TalkElement {
                    local_talk_id: 1,
                    element_type: TalkElementType::Text(TalkTextElement {
                        talker: Talkers::NPC(1),
                        text: "Hello, Player! This is talk test 0-1".to_string(),
                        next_talk_element_id: 2,
                    }),
                },
                TalkElement {
                    local_talk_id: 2,
                    element_type: TalkElementType::Choice(vec![
                        TalkChoiceElement {
                            text: "Choice 1".to_string(),
                            next_talk_element_id: 3,
                        },
                        TalkChoiceElement {
                            text: "Choice 2".to_string(),
                            next_talk_element_id: 4,
                        },
                    ]),
                },
                TalkElement {
                    local_talk_id: 3,
                    element_type: TalkElementType::End,
                },
                TalkElement {
                    local_talk_id: 4,
                    element_type: TalkElementType::End,
                },
            ],
        }
    }
}

#[derive(Component, PartialEq, Eq)]
pub struct TalkElement {
    pub local_talk_id: u32, //各会話ごとにローカルなユニークID
    pub element_type: TalkElementType,
}

#[derive(Component, PartialEq, Eq)]
pub enum TalkElementType {
    Text(TalkTextElement),
    Choice(Vec<TalkChoiceElement>),
    End,
}

// 会話のテキスト要素
#[derive(Component, PartialEq, Eq)]
pub struct TalkTextElement {
    pub talker: Talkers,
    pub text: String,
    pub next_talk_element_id: u32,
}

#[derive(Component, PartialEq, Eq)]
pub enum Talkers {
    Player,
    NPC(u32), //NPCのユニークID
}

// 会話の選択肢要素
#[derive(Component, PartialEq, Eq)]
pub struct TalkChoiceElement {
    pub text: String,
    pub next_talk_element_id: u32,
}
