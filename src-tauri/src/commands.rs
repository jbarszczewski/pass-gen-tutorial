use rand::Rng;

static CHARS: [char; 70] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9','a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
't', 'u', 'v', 'w', 'x', 'y', 'z','A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '!','@','#','$','%','^','&','*'];

#[tauri::command]
fn generate_password(length: u32) -> String{
	let mut rng = rand::thread_rng();
	let mut result = String::new();
	for _x in 0..length {
		result.push(CHARS[rng.gen_range(0..70)]);
	}
	
	result
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn generates_string() {
		let result = generate_password(8);
        assert_eq!(result.len(), 8);
    }
}
