use time;
use replies::ReplyRenderer;

pub struct TextReply {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub content: String,
}

impl TextReply {
	pub fn new(source: &str, target: &str, content: &str) -> TextReply {
		TextReply {
			source: source.to_string(),
			target: target.to_string(),
			time: time::get_time().sec,
			content: content.to_string(),
		}
	}
}

impl ReplyRenderer for TextReply {
	fn render(&self) -> String {
		format!("<xml>\n\
		    <ToUserName><![CDATA[{target}]]></ToUserName>\n\
		    <FromUserName><![CDATA[{source}]]></FromUserName>\n\
		    <CreateTime>{time}</CreateTime>\n\
		    <MsgType><![CDATA[text]]></MsgType>\n\
		    <Content><![CDATA[{content}]]></Content>\n\
		    </xml>",
		    target=self.target,
		    source=self.source,
		    time=self.time,
		    content=self.content
	    )
	}
}

#[cfg(test)]
mod tests {
	use replies::ReplyRenderer;
	use super::TextReply;

	#[test]
	fn test_render_text_reply() {
		let reply = TextReply::new("test1", "test2", "test");
		let rendered = reply.render();
		assert!(rendered.contains("test1"));
		assert!(rendered.contains("test2"));
		assert!(rendered.contains("test"));
	}
}