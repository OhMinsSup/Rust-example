### 크레이트를 사용하여 더 많은 기능 얻기

크레이트는 Rust 소스 코드 파일의 모음이라는 것을 기억하십시오. 우리가 구축한 프로젝트 는 실행 파일인 바이너리 크레이트 입니다. rand 상자는이다 라이브러리 상자 다른 프로그램에서 사용하기위한 코드가 포함되어 있습니다.

Cargo의 외부 상자의 조정은 Cargo가 진정으로 빛나는 곳입니다. 를 사용하는 코드를 작성하기 전에 상자를 종속성으로 포함하도록 Cargo.toml 파일을 rand수정해야 합니다 . 지금 그 파일을 열고 Cargo가 생성 한 섹션 헤더 아래 맨 아래에 다음 줄을 추가하십시오 . 여기에 있는 대로 정확하게 지정 하지 않으면 이 자습서의 코드 예제가 작동하지 않을 수 있습니다.rand[dependencies]rand

파일 이름: Cargo.toml

```bash
rand = "0.8.3"
```

에서 Cargo.toml의 파일 헤더를 다음과 모든 다른 섹션이 시작될 때까지 계속 섹션의 일부입니다. 이 [dependencies]섹션은 프로젝트가 의존하는 외부 크레이트와 필요한 크레이트 버전을 Cargo에 알려주는 곳입니다. 이 경우 rand 의미론적 버전 지정자로 크레이트를 지정합니다 0.8.3. Cargo 는 버전 번호 작성을 위한 표준인 Semantic Versioning ( SemVer 라고도 함 )을 이해 합니다. 숫자 0.8.3는 실제로 의 줄임말입니다 ^0.8.3. 즉, 최소한 0.8.3이지만 그 이하인 모든 버전을 의미합니다 0.9.0. Cargo는 이러한 버전이 버전과 호환되는 공개 API를 갖는 것으로 간주합니다.0.8.3, 그리고 이 사양은 이 장의 코드로 계속 컴파일되는 최신 패치 릴리스를 얻을 수 있도록 합니다. 모든 버전 0.9.0이상은 다음 예제에서 사용하는 것과 동일한 API를 사용하지 않을 수 있습니다.

```bash
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

다른 버전 번호(그러나 SemVer! 덕분에 모두 코드와 호환 가능), 다른 행(운영 체제에 따라 다름) 및 행의 순서가 다를 수 있습니다.

이제 외부 종속성이 있으므로 Cargo 는 Crates.io 의 데이터 복사 본인 레지스트리 에서 모든 최신 버전을 가져옵니다 . Crates.io는 Rust 생태계의 사람들이 다른 사람들이 사용할 수 있도록 오픈 소스 Rust 프로젝트를 게시하는 곳입니다.

레지스트리를 업데이트한 후 Cargo는 [dependencies]섹션을 확인하고 아직 가지고 있지 않은 상자를 다운로드합니다. 이 경우 rand종속성으로 만 나열했지만 Cargo는 rand작업에 종속된 다른 크레이트도 가져 왔습니다. 크레이트를 다운로드한 후, Rust는 이를 컴파일하고 사용 가능한 종속성을 사용하여 프로젝트를 컴파일합니다.

cargo build변경하지 않고 즉시 다시 실행 하면 Finished행 외에는 출력이 표시되지 않습니다 . Cargo는 이미 종속성을 다운로드하고 컴파일했으며 Cargo.toml 파일 에서 이에 대해 아무 것도 변경하지 않았 음을 알고 있습니다. Cargo는 또한 코드에 대해 아무 것도 변경하지 않았음을 알고 있으므로 이를 다시 컴파일하지도 않습니다. 할 일 없이 그냥 종료됩니다.

src/main.rs 파일 을 열고 사소한 변경을 한 다음 저장하고 다시 빌드하면 두 줄의 출력만 표시됩니다.

$ cargo build
Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
이 라인은 Cargo가 src/main.rs 파일에 대한 약간의 변경으로만 빌드를 업데이트한다는 것을 보여줍니다 . 의존성은 변경되지 않았으므로 Cargo는 이미 다운로드하고 컴파일한 것을 재사용할 수 있음을 알고 있습니다. 코드의 일부만 다시 빌드합니다.

### Cargo.lock 파일로 재현 가능한 빌드 확인

Cargo에는 귀하 또는 다른 사람이 코드를 빌드할 때마다 동일한 아티팩트를 다시 빌드할 수 있는 메커니즘이 있습니다. Cargo는 귀하가 달리 표시할 때까지 귀하가 지정한 종속성 버전만 사용합니다. 예를 들어 다음 주에 rand크레이트 버전 0.8.4 가 나오고 해당 버전에 중요한 버그 수정이 포함되어 있지만 코드를 손상시키는 회귀도 포함되어 있다면 어떻게 될까요?

이 문제에 대한 답은 Cargo.lock 파일입니다. 이 파일은 처음 실행했을 때 생성되었으며 cargo build지금은 추측 \_게임 디렉토리에 있습니다. 처음으로 프로젝트를 빌드할 때 Cargo는 기준에 맞는 모든 버전의 종속성을 파악한 다음 Cargo.lock 파일에 기록합니다 . 나중에 프로젝트를 빌드할 때 Cargo는 Cargo.lock 파일이 존재함을 확인하고 버전을 다시 파악하는 모든 작업을 수행하는 대신 거기에 지정된 버전을 사용합니다. 이를 통해 자동으로 재현 가능한 빌드를 만들 수 있습니다. 즉, Cargo.lock 파일 0.8.3덕분에 명시적으로 업그레이드할 때까지 프로젝트가 에 유지 됩니다.

### 새 버전을 얻기 위해 크레이트 업데이트

당신이하면 않는 나무 상자를 업데이트 할,화물은 다른 명령, 제공 update무시합니다, Cargo.lock의 에서 귀하의 사양에 맞게 모든 최신 버전에서 파일 및 그림 Cargo.toml을 . 그것이 작동하면 Cargo는 해당 버전을 Cargo.lock 파일 에 기록 합니다.

그러나 기본적으로 Cargo는 보다 크 0.8.3거나 작은 버전만 찾습니다 0.9.0. 는 IF rand상자는 두 가지 새로운 버전을 발표했다 0.8.4그리고 0.9.0, 당신은 당신이 실행 한 경우 다음을 볼 것입니다 cargo update:

```bash
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

이 시점에서 Cargo.lock 파일 에서 rand현재 사용 중인 크레이트 버전 이 0.8.4.

당신이 사용하고자하는 경우 rand버전 0.9.0또는 모든 버전을 0.9.x 시리즈, 당신은 업데이트해야 할 것 Cargo.toml의 대신이 같은 모습에 파일을 :

[dependencies]
rand = "0.9.0"
다음에 를 실행할 cargo build때 Cargo는 사용 가능한 상자의 레지스트리를 업데이트하고 rand지정한 새 버전에 따라 요구 사항을 재평가 합니다.

14장에서 논의할 Cargo 와 그 생태계 에 대해 할 말이 더 많지만 지금은 그것이 당신이 알아야 할 전부입니다. Cargo를 사용하면 라이브러리를 매우 쉽게 재사용할 수 있으므로 Rustaceans는 여러 패키지에서 조합된 더 작은 프로젝트를 작성할 수 있습니다.
