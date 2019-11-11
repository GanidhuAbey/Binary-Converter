use std::io;

fn main() {
	//give user directions
	println!("this is the binary converter, please enter a number");

	//create variable to store user information
	let mut number = String::new();
	//create variable to determine whether the number is a binary or a integer
	let mut bn = 1;

	let mut num1 = 2;

	let mut timer = 4;

	//save user input into variable "number"
	io::stdin().read_line(&mut number).expect("fail");

	//create match statement to decipher whether the number is a binary of an integer
	//and store it into the variable "bn"
	match number.trim().parse::<u32>() {
		Ok(n) => num1 = n,
		Err(b) => bn = 2,
	};

	//differentiate what the computer does depending on if its a binary or a number
	if (bn == 1) {
		//create a loop so that the programs length changes based on the number input
		while (num1 != 0) {
			//the inputed number is converted into a binary
			print!("{}", num1 % 2);
			//tells the program when to stop
			num1 = num1 / 2;
		}
		// creates a new line for the binary
		println!("");
	}
	//if the user inputed a binary
	else if bn == 2 {
		let mut binary = &number[4..];

		let mut binary1 =  binary.to_string();

		let mut zero = "0000000000000000000000000000000".to_string();

		binary1.push_str(&zero);

		let mut p1 = &binary1[0..1];

		let mut p2 = &binary1[1..2];

		let mut p3 = &binary1[2..3];

		let mut p4 = &binary1[3..4];

		let mut p5 = &binary1[4..5];

		let mut p6 = &binary1[5..6];

		let mut p7 = &binary1[6..7];

		let mut p8 = &binary1[7..8];

		let mut p9 = &binary1[8..9];

		let mut p10 = &binary1[9..10];

		let mut p11 = &binary1[10..11];

		let mut p12 = &binary1[11..12];

		let mut p13 = &binary1[12..13];

		let mut p14 = &binary1[13..14];

		let mut p15 = &binary1[14..15];

		let mut p16 = &binary1[15..16];

		let mut p17 = &binary1[16..17];

		let mut p18 = &binary1[17..18];

		let mut p19 = &binary1[18..19];

		let mut p20 = &binary1[19..20];

		let mut p21 = &binary1[20..21];

		let mut p22 = &binary1[21..22];

		let mut p23 = &binary1[22..23];

		let mut p24 = &binary1[23..24];

		let mut p25 = &binary1[24..25];

		let mut p26 = &binary1[25..26];

		let mut p27 = &binary1[26..27];

		let mut p28 = &binary1[27..28];

		let mut p29 = &binary1[28..29];

		let mut p30 = &binary1[29..30];

		let mut p31 = &binary1[30..31];

		let mut p32 = &binary1[31..32];

		let mut np1 = 0;
		let mut np2 = 0;
		let mut np3 = 0;
		let mut np4 = 0;
		let mut np5 = 0;
		let mut np6 = 0;
		let mut np7 = 0;
		let mut np8 = 0;
		let mut np9 = 0;
		let mut np10 = 0;
		let mut np11 = 0;
		let mut np12 = 0;
		let mut np13 = 0;
		let mut np14 = 0;
		let mut np15 = 0;
		let mut np16 = 0;
		let mut np17 = 0;
		let mut np18 = 0;
		let mut np19 = 0;
		let mut np20 = 0;
		let mut np21 = 0;
		let mut np22 = 0;
		let mut np23 = 0;
		let mut np24 = 0;
		let mut np25 = 0;
		let mut np26 = 0;
		let mut np27 = 0;
		let mut np28 = 0;
		let mut np29 = 0;
		let mut np30 = 0;
		let mut np31 = 0;
		let mut np32 = 0;



		match p1.trim().parse::<u32>() {
		Ok(n) => np1 = n,
		Err(b) => bn = 2,
		};
		match p2.trim().parse::<u32>() {
		Ok(n) => np2 = n,
		Err(b) => bn = 2,
		};
		match p3.trim().parse::<u32>() {
		Ok(n) => np3 = n,
		Err(b) => bn = 2,
		};
		match p4.trim().parse::<u32>() {
		Ok(n) => np4 = n,
		Err(b) => bn = 2,
		};
		match p5.trim().parse::<u32>() {
		Ok(n) => np5 = n,
		Err(b) => bn = 2,
		};
		match p6.trim().parse::<u32>() {
		Ok(n) => np6 = n,
		Err(b) => bn = 2,
		};
		match p7.trim().parse::<u32>() {
		Ok(n) => np7 = n,
		Err(b) => bn = 2,
		};
		match p8.trim().parse::<u32>() {
		Ok(n) => np8 = n,
		Err(b) => bn = 2,
		};
		match p9.trim().parse::<u32>() {
		Ok(n) => np9 = n,
		Err(b) => bn = 2,
		};
		match p10.trim().parse::<u32>() {
		Ok(n) => np10 = n,
		Err(b) => bn = 2,
		};
		match p11.trim().parse::<u32>() {
		Ok(n) => np11 = n,
		Err(b) => bn = 2,
		};
		match p12.trim().parse::<u32>() {
		Ok(n) => np12 = n,
		Err(b) => bn = 2,
		};
		match p13.trim().parse::<u32>() {
		Ok(n) => np13 = n,
		Err(b) => bn = 2,
		};
		match p14.trim().parse::<u32>() {
		Ok(n) => np14 = n,
		Err(b) => bn = 2,
		};
		match p15.trim().parse::<u32>() {
		Ok(n) => np15 = n,
		Err(b) => bn = 2,
		};
		match p16.trim().parse::<u32>() {
		Ok(n) => np16 = n,
		Err(b) => bn = 2,
		};
		match p17.trim().parse::<u32>() {
		Ok(n) => np17 = n,
		Err(b) => bn = 2,
		};
		match p18.trim().parse::<u32>() {
		Ok(n) => np18 = n,
		Err(b) => bn = 2,
		};
		match p19.trim().parse::<u32>() {
		Ok(n) => np19 = n,
		Err(b) => bn = 2,
		};
		match p20.trim().parse::<u32>() {
		Ok(n) => np20 = n,
		Err(b) => bn = 2,
		};
		match p21.trim().parse::<u32>() {
		Ok(n) => np21 = n,
		Err(b) => bn = 2,
		};
		match p22.trim().parse::<u32>() {
		Ok(n) => np22 = n,
		Err(b) => bn = 2,
		};
		match p23.trim().parse::<u32>() {
		Ok(n) => np23 = n,
		Err(b) => bn = 2,
		};
		match p24.trim().parse::<u32>() {
		Ok(n) => np24 = n,
		Err(b) => bn = 2,
		};
		match p25.trim().parse::<u32>() {
		Ok(n) => np25 = n,
		Err(b) => bn = 2,
		};
		match p26.trim().parse::<u32>() {
		Ok(n) => np26 = n,
		Err(b) => bn = 2,
		};
		match p27.trim().parse::<u32>() {
		Ok(n) => np27 = n,
		Err(b) => bn = 2,
		};
		match p28.trim().parse::<u32>() {
		Ok(n) => np28 = n,
		Err(b) => bn = 2,
		};
		match p29.trim().parse::<u32>() {
		Ok(n) => np29 = n,
		Err(b) => bn = 2,
		};
		match p30.trim().parse::<u32>() {
		Ok(n) => np30 = n,
		Err(b) => bn = 2,
		};
		match p31.trim().parse::<u32>() {
		Ok(n) => np31 = n,
		Err(b) => bn = 2,
		};
		match p32.trim().parse::<u32>() {
		Ok(n) => np32 = n,
		Err(b) => bn = 2,
		};

		np1 = np1 * 1;
		np2 = np2 * 2;
		np3 = np3 * 4;
		np4 = np4 * 8;
		np5 = np5 * 16;
		np6 = np6 * 32;
		np7 = np7 * 64;
		np8 = np8 * 128;
		np9 = np9 * 256;
		np10 = np10 * 512;
		np11 = np11 * 1024;
		np12 = np12 * 2048;
		np13 = np13 * 4096;
		np14 = np14 * 8192;
		np15 = np15 * 16384;
		np16 = np16 * 32768;
		np17 = np17 * 65536;
		np18 = np18 * 131072;
		np19 = np19 * 262144;
		np20 = np20 * 524288;
		np21 = np21 * 1048576;
		np22 = np22 * 2097152;
		np23 = np23 * 4194304;
		np24 = np24 * 8388608;
		np25 = np25 * 16777216;
		np26 = np26 * 33554432;
		np27 = np27 * 67108864;
		np28 = np28 * 134217728;
		np29 = np29 * 268435456;
		np30 = np30 * 536870912;
		np31 = np31 * 1073741824;
		np32 = np32 * 2147483648;

		let binarypack1 = np1 + np2 + np3 + np4;
		let binarypack2 = np5 + np6 + np7 + np8;
		let binarypack3 = np9 + np10 + np11 + np12;
		let binarypack4 = np13 + np14 + np15 + np16;
		let binarypack5 = np17 + np18 + np19 + np20;
		let binarypack6 = np21 + np22 + np23 + np24;
		let binarypack7 = np25 + np26 + np27 + np28;
		let binarypack8 = np29 + np30 + np31 + np32;

		let fin_binarypack1 = binarypack1 + binarypack2 + binarypack3 + binarypack4;
		let fin_binarypack2 = binarypack5 + binarypack6 + binarypack7 + binarypack8;

		println!("{}", fin_binarypack1 + fin_binarypack2);



	}
}