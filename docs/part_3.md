## 추측 게임 프로그래밍

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

사용자 입력을 얻은 다음 결과를 출력으로 인쇄하려면 io(입력/출력) 라이브러리를 범위 로 가져와야 합니다 . io라이브러리 (이 알려져되는 표준 라이브러리에서 제공 std)

```rust
use std::io;
```

fn구문은 새로운 기능, 괄호는, 선언 (), 매개 변수가없는 표시하고 중괄호는 {, 함수의 본문을 시작합니다.

```rust
fn main() { }
```

1장에서 배웠듯이 println!은 문자열을 화면에 출력하는 매크로입니다.

```rust
println!("Hello, world!");
```

### 변수로 값 저장하기

```rust
let mut guess = String::new();
```

이제 프로그램이 흥미로워지고 있습니다! 이 작은 줄에 많은 일이 있습니다. 이것은 변수let 를 생성하는 데 사용되는 문 입니다. 다음은 또 다른 예입니다.

```rust
let apples = 5;
```

이 줄은 이름이 지정된 새 변수를 만들고 apples값 5에 바인딩합니다. Rust에서 변수는 기본적으로 변경할 수 없습니다. 이 개념에 대해서는 3장의 "변수와 변경 가능성" 섹션 에서 자세히 논의할 것입니다 . 다음 예는 mut변수 이름 앞에 변수를 사용하여 변수를 변경 가능하게 만드는 방법을 보여줍니다 .

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

추리 게임 프로그램으로 돌아가자. 이제 let mut guess 라는 이름의 변경 가능한 변수가 도입 된다는 것을 알았습니다 guess. 등호( =) 의 반대쪽에는 의 새 인스턴스를 반환하는 함수를 guess호출한 결과인 바인딩된 값이 있습니다 . 확장 가능한 UTF-8로 인코딩된 텍스트 비트인 표준 라이브러리에서 제공하는 문자열 유형입니다.String::newStringString

줄 의 ::구문 은 해당 유형 의 관련 함수::new 임을 나타냅니다 . 관련 함수는 유형(이 경우 .newStringString

이 new함수는 비어 있는 새 문자열을 만듭니다. new어떤 종류의 새로운 값을 만드는 함수의 일반적인 이름이기 때문에 많은 유형에서 함수를 찾을 수 있습니다.

요약하자면, let mut guess = String::new();라인은 현재 비어 있는 새 인스턴스에 바인딩된 변경 가능한 변수를 생성했습니다 String. 아휴!

use std::io;프로그램의 첫 번째 줄에 표준 라이브러리의 입력/출력 기능을 포함했음을 상기하십시오. 이제 모듈 에서 stdin함수를 호출합니다 io.

```rust
    io::stdin()
        .read_line(&mut guess)
```

use std::io프로그램의 시작 부분에 라인을 넣지 않았다면 이 함수 호출을 std::io::stdin. 이 stdin함수는 std::io::Stdin터미널의 표준 입력에 대한 핸들을 나타내는 유형인 의 인스턴스를 반환합니다 .

코드의 다음 부분인 은 표준 입력 핸들 .read_line(&mut guess)의 read_line메서드를 호출 하여 사용자로부터 입력을 받습니다. 우리는 또한 하나의 인수를 전달합니다 read_line: &mut guess.

의 작업은 read_line사용자가 입력한 내용을 표준 입력으로 가져와서 내용을 덮어쓰지 않고 문자열에 추가하여 해당 문자열을 인수로 취하는 것입니다. 문자열 인수는 변경 가능해야 메서드가 사용자 입력을 추가하여 문자열의 내용을 변경할 수 있습니다.

는 &이 인수가 있음을 나타냅니다 참조 하면 메모리를 여러 번에 데이터를 복사 할 필요없이 데이터의 코드 액세스 한 조각의 여러 부분을 할 수있는 방법을 제공한다. 참조는 복잡한 기능이며, Rust의 주요 장점 중 하나는 참조를 사용하는 것이 얼마나 안전하고 쉽다는 것입니다. 이 프로그램을 마치기 위해 많은 세부 사항을 알 필요는 없습니다. 지금은 변수와 마찬가지로 참조도 기본적으로 변경 불가능하다는 사실만 알면 됩니다. 따라서 변경 가능하게 만드는 &mut guess것보다 작성해야 합니다 &guess. (제4장에서 참조에 대해 더 자세히 설명합니다.

### Result유형으로 잠재적 실패 처리

우리는 여전히 이 코드 줄을 작업 중입니다. 우리는 지금 세 번째 텍스트 줄에 대해 논의하고 있지만 여전히 단일 논리적 코드 줄의 일부입니다. 다음 부분은 이 방법입니다.

```rust
.expect("Failed to read line");
```

.method_name()구문을 사용 하여 메서드를 호출할 때 긴 줄을 구분하는 데 도움이 되도록 개행 및 기타 공백을 도입하는 것이 종종 현명합니다. 이 코드를 다음과 같이 작성할 수 있습니다.

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

단, 긴 줄 하나는 읽기 어려우므로 나누어서 하는 것이 좋다. 이제 이 줄이 무엇을 하는지 논의해 보겠습니다.

앞서 언급했듯이 read_line사용자가 입력한 내용을 전달하는 문자열에 넣지만 값도 반환합니다(이 경우 io::Result. 녹라는 유형의 숫자가 Result일반 : 표준 라이브러리에 Result 같은 서브 모듈뿐만 아니라 특정 버전을 io::Result.

Result유형은 열거 종종 언급 열거 . 열거형은 고정된 값 집합을 가질 수 있는 유형이며 이러한 값을 열거형의 변형 이라고 합니다 . 6장에서는 열거형을 더 자세히 다룰 것입니다.

의 경우 Result변형은 Ok또는 Err입니다. Ok변형 작업이 성공적으로 표시하고, 내부는 Ok성공적으로 생성 된 값입니다. Err변형 작업이 실패 의미하며, Err작업이 실패하는 방법이나 이유에 대한 정보가 포함되어 있습니다.

이러한 Result유형 의 목적은 오류 처리 정보를 인코딩하는 것입니다. 의 값 Result타입은 모든 유형의 값처럼, 방법은 그들에 정의했습니다. 의 인스턴스 io::Result에는 호출할 수 있는 expect메서드 가 있습니다. 이 인스턴스 io::Result가 Err값이면 expect프로그램이 충돌하고 에 인수로 전달한 메시지가 표시됩니다 expect. read_line메서드가 반환하는 경우 Err기본 운영 체제에서 발생한 오류의 결과일 수 있습니다. 이 인스턴스 io::Result가 Ok값이면 expect반환 값을 취합니다.Ok당신이 그것을 사용할 수 있도록 해당 값을 보유하고 반환합니다. 이 경우 해당 값은 사용자가 표준 입력에 입력한 바이트 수입니다.

를 호출하지 않으면 expect프로그램이 컴파일되지만 경고가 표시됩니다.

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Rust는 프로그램이 가능한 오류를 처리하지 않았음을 나타내는 Result에서 반환된 값을 사용하지 않았다고 경고합니다 read_line.

경고를 억제하는 올바른 방법은 실제로 오류 처리를 작성하는 것이지만 문제가 발생했을 때 이 프로그램을 중단시키려면 expect. 9장에서 오류 복구에 대해 배웁니다.
