

pub fn gethtml<'a>(name: &'a str)-> String {
	let mut buffer = String::new();
	html!(buffer, {
			html {
				head {
					title {
						"Hello" $name
					}
					meta charset="utf-8" /
						
				}
				body {
	    		p { "Hi, " $name "!" }
			    h1 {"test of " $name "!"}
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

