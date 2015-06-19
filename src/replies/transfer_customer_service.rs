use time;
use replies::ReplyRenderer;

pub struct TransferCustomerServiceReply {
    pub source: String,
    pub target: String,
    pub time: i64,
}

impl TransferCustomerServiceReply {
	pub fn new(source: &str, target: &str) -> TransferCustomerServiceReply {
		TransferCustomerServiceReply {
			source: source.to_string(),
			target: target.to_string(),
			time: time::get_time().sec,
		}
	}
}

impl ReplyRenderer for TransferCustomerServiceReply {
	fn render(&self) -> String {
		format!("<xml>\n\
		    <ToUserName><![CDATA[{target}]]></ToUserName>\n\
		    <FromUserName><![CDATA[{source}]]></FromUserName>\n\
		    <CreateTime>{time}</CreateTime>\n\
		    <MsgType><![CDATA[transfer_customer_service]]></MsgType>\n\
		    </xml>",
		    target=self.target,
		    source=self.source,
		    time=self.time,
	    )
	}
}

#[cfg(test)]
mod tests {
	use replies::ReplyRenderer;
	use super::TransferCustomerServiceReply;

	#[test]
	fn test_render_text_reply() {
		let reply = TransferCustomerServiceReply::new("test1", "test2");
		let rendered = reply.render();
		assert!(rendered.contains("test1"));
		assert!(rendered.contains("test2"));
		assert!(rendered.contains("transfer_customer_service"));
	}
}