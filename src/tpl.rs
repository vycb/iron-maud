use std::collections::HashMap;

pub fn gethtml<'a>(map: &'a HashMap<&str,&str>)-> String {
	let mut buffer = String::new();
	html!(buffer, {
			html {
				head {
					title {
						"Hello " $map["name"] "!"
					}
					meta charset="utf-8" /
						
				}
				body {
	    		p { "Hi, " $map["name"] "!" }
			    h1 {"test of " $map["greating"] "!"}
			    p {
			      "Watch as I work my gypsy magic"
						br /
						"Eye of a newt and cinnamon"
					}
				}
			}
		}).unwrap();
	
	buffer
}

