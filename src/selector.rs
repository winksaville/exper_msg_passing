	    
fn selector() {
	println!("Top select! Loop");

	select! {
		recv(int_receiver) -> msg => match msg {
			Ok(message) => println!("Received an integer: {}", message),
			Err(why) => println!("Error int_receiver: {why:?}"),
		},
		recv(string_receiver) -> msg => match msg {
			Ok(message) => println!("Received a string: {}", message),
			Err(why) => println!("Error string_receiver: {why:?}"),
		},
		recv(done_receiver) -> msg => {
			match msg {
			Ok(()) => println!("Done received"),
			Err(why) => println!("Error done_receiver: {why:?}"),
			}

			break;
		},
	}
}