use serde::{Deserialize, Serialize};
use serde_json::Result;

pub struct Chat {}

impl Chat {
    fn new() {
        //serde_json::from_slice(v)
    }

    fn from(data: &[u8]) {
        //let json = serde_json::from_slice(data);
    }
}
#[derive(Serialize, Deserialize)]
pub struct ChatObject {
    #[serde(rename(serialize = "text"))]
    text: String,

    #[serde(rename(serialize = "bold"))]
    bold: bool, //字体加粗

    #[serde(rename(serialize = "italic"))]
    italic: bool, //斜体

    #[serde(rename(serialize = "underlined"))]
    underlined: bool, //下划线

    #[serde(rename(serialize = "strikethrough"))]
    strikethrough: bool, //删除线

    #[serde(rename(serialize = "obfuscated"))]
    obfuscated: bool, //模糊样式

    #[serde(rename(serialize = "font"))]
    font: String, //字体样式

    // #[serde(rename(serialize = "color"))]
    // color: Color, //字体颜色

    // #[serde(rename(serialize = "insertion"))]
    // insertion: String, //插入

    // #[serde(rename(serialize = "clickEvent"))]
    // click_event: ClickEvent,

    // #[serde(rename(serialize = "hoverEvent"))]
    // hover_event: HoverEvent,
    #[serde(rename(serialize = "extra"))]
    extra: Vec<ChatObject>,
}

//"clickEvent":{"action":"suggest_command","value":"/msg Herobrine "}
pub enum ClickEvent {
    OpenURL(String),        //打开链接
    RunCommand(String),     //运行命令
    SuggestCommand(String), //提示命令
    ChangePage,             //改变页面
    CopyToClipboard,        //复制到粘贴板
}

pub enum HoverEvent {
    ShowText,
    ShowItem,
    ShowEntity,
}

pub enum Color {
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    Purple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    BrightGreen,
    Cyan,
    Red,
    Pink,
    Yellow,
    White,
    FormatCode(i32, i32, i32), //R G B
}
