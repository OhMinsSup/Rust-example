use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        //  확장 가능한 UTF-8로 인코딩된 텍스트 비트인 표준 라이브러리에서
        //  제공하는 문자열 유형입니다.String::newString
        //  줄 의 ::구문 은 해당 유형 의 관련 함수::new 임을 나타냅니다.
        let mut guess = String::new(); // 비어있는 새 문자열을 만듬
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 입력을 읽을 수 없는 경우 예외를 던집니다.
        // 섀도잉
        // trim()은 문자열의 양 끝에 있는 공백을 제거합니다.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
