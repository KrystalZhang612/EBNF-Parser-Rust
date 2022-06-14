

struct SimpleParser{
	
	//there must not be 'cd'
	//there can be only 'c' or 'd' 
	//if there is neither c nor d, syntax error at char_pos = 0
	
	char_pos: i32, 
	
	input: String, }



impl SimpleParser{} 



fn initializer(input:&str)->(){ }


	
	fn fun_s(input:&str)->(){
		
		let CharC = 'c'; 
		
		let CharD = 'd'; 
		
		//if there is c and d
		
		//input is valid 
		
		if (fun_x(input)){
			
			println!("Input is valid"); 
			
		}
		
		
		//if there is consecutive cd
		
		else if (input.contains(CharC)) && (input.contains(CharD)){
			
			
			println!("Syntax error at character position {}", input.chars().position(|c| c == 'd').unwrap()); 
			
		}

		
		else if (!input.contains(CharC)) && (!input.contains(CharD)){
			
			println!("Syntax error at character position 0"); 	
		}
		
	}
		
		
		//a boolean function to check if there is c or d in the input string 
		//but cannot occur both c and d consecutive or adjacent 
		
		
		fn fun_x(input: &str) -> bool{
			
			let CharC = 'c';
			
			let CharD = 'd'; 
	
			
			if (input.contains(CharC)) && (!input.contains(CharD)){
				
				return true; 
			}
			
			
			else if (!input.contains(CharC)) && (input.contains(CharD)){
				
				return true; 
			}
			else {
				
				return false; 
			}
			
		}
		
		
	
		//a void function to get the next character 
		
		fn get_next_char(input:&str)-> (){
			
			
			for  char_pos in 0..input.len() {
				
				println!("{}", char_pos); 
			}
			
		}
		
		
//driver test 
	
	
	
fn main() {
	
	
	let input1 = "bc";
	
	fun_s(input1); 
	
	
	let input2 = "acd";
	
	fun_s(input2); 
	
	
	let input3 = "aaad";
	
	fun_s(input3); 
	
	
	let input4 = "c";
	
	fun_s(input4);
	
	
	let input5 = "2yz";
	
	fun_s(input5); 
	
	
	let input6 = "";
	
	fun_s(input6); 
	
	
		
	
//List of Extra Test Cases Used for Debugging:
	
	
	let input7 = "suwhgd";
	
	fun_s(input7); 
	
	
	let input8 = "..!!";
	
	fun_s(input8); 
	
	
	let input9 = "dgdgg";
	
	fun_s(input9); 
	
	
	//Outputs for the extra debugging cases: 

	//Input is valid
	//Syntax error at character position 0
	//Input is valid
	
	
	
	
}