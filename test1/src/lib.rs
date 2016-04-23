
use std::fmt::Debug;

/**
 * A generic message representation with headers and body.
 *
 */
trait Message<T> {
	/**
	 * Return the message payload.
	 */
	fn get_payload(&self) -> &T;

	/**
	 * Return message headers for the message (never {@code null} but may be empty).
	 */
	fn get_headers(&self) -> &MessageHeaders;
}

#[derive(Debug)]
struct MessageHeaders {
}

/**
 * Constant for sending a message without a prescribed timeout.
 */
const INDEFINITE_TIMEOUT: i32 = -1;

/**
 * Defines methods for sending messages.
 *
 */
trait MessageChannel<T> {
	/**
	 * Send a {@link Message} to this channel. If the message is sent successfully,
	 * the method returns {@code true}. If the message cannot be sent due to a
	 * non-fatal reason, the method returns {@code false}. The method may also
	 * throw a RuntimeException in case of non-recoverable errors.
	 * <p>This method may block indefinitely, depending on the implementation.
	 * To provide a maximum wait time, use {@link #send(Message, long)}.
	 */
	fn send(&mut self, message: Box<Message<T>>) -> Result<(), ()> {
		self.send_timeout(message, INDEFINITE_TIMEOUT)
	}

	/**
	 * Send a message, blocking until either the message is accepted or the
	 * specified timeout period elapses.
	 * @param message the message to send
	 * @param timeout the timeout in milliseconds or {@link #INDEFINITE_TIMEOUT}
	 * @return {@code true} if the message is sent, {@code false} if not
	 * including a timeout of an interrupt of the send
	 */
	fn send_timeout(&mut self, message: Box<Message<T>>, timeout: i32) -> Result<(), ()>;
}

/**
 * Contract for handling a Message.
 *
 */
trait MessageHandler<T> : Debug {
	/**
	 * Handle the given message.
	 */
	fn handle_message(&self, message: Box<Message<T>>) -> Result<(), ()>;
}

/**
 * A {@link MessageChannel} that maintains a registry of subscribers and invokes
 * them to handle messages sent through this channel.
 */
trait SubscribableChannel<T> : MessageChannel<T> {
	/**
	 * Register a message handler.
	 * @return {@code true} if the handler was subscribed or {@code false} if it
	 * was already subscribed.
	 */
	fn subscribe(&mut self, handler:Box<MessageHandler<T>>) -> bool;

	/**
	 * Un-register a message handler.
	 * @return {@code true} if the handler was un-registered, or {@code false}
	 * if was not registered.
	 */
	fn unsubscribe(&mut self, handler: Box<MessageHandler<T>>) -> bool { false }
}

#[cfg(test)]
mod tests {
	use super::{ Message, MessageHeaders, MessageChannel, 
		SubscribableChannel, MessageHandler };
	use std::sync::{ Arc, RwLock };

	struct TestMessage {
		payload: String,
		headers: MessageHeaders
	}

	impl TestMessage {
		fn new() -> TestMessage {
			TestMessage {
				payload: "payload".to_string(),
				headers: MessageHeaders{}
			}
		}
	}

	impl Message<String> for TestMessage {
		fn get_payload(&self) -> &String {
			&self.payload
		}

		fn get_headers(&self) -> &MessageHeaders {
			&self.headers
		}
	}

	#[derive(Debug)]
	struct TestChannel {
		handler: Option<Box<MessageHandler<String>>>,
		payload: String
	}

	impl TestChannel {
		fn new() -> TestChannel {
			TestChannel {
				handler: None,
				payload: "".to_string()
			}
		}
	}

	impl MessageChannel<String> for TestChannel {
	    fn send_timeout(&mut self, message: Box<Message<String>>, timeout: i32) -> Result<(), ()> {
	    	self.payload = message.get_payload().clone();
	    	let mut handler = self.handler.take().unwrap();
	    	let r = handler.handle_message(message);
	    	self.handler = Some(handler);
	    	r
	    }
	}

	impl SubscribableChannel<String> for TestChannel {
	    fn subscribe(&mut self, handler: Box<MessageHandler<String>>) -> bool {
	    	self.handler = Some(handler);
	    	true
	    }
	}

	#[derive(Debug)]
	struct TestMessageHandler {
	}

	impl TestMessageHandler {
		fn new() -> TestMessageHandler {
			TestMessageHandler {}
		}
	}

	impl MessageHandler<String> for TestMessageHandler {
		fn handle_message(&self, message: Box<Message<String>>) -> Result<(), ()> {
			Ok(())
		}
	}

    #[test]
    fn it_works() {
    	let handler = Box::new(TestMessageHandler::new());
    	let mut channel = Box::new(TestChannel::new());
    	let message = Box::new(TestMessage::new());

    	assert_eq!(true, channel.subscribe(handler));
    	assert_eq!(Ok(()), channel.send(message) );
    	assert_eq!("payload", channel.payload );

    }
}
