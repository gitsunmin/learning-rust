# Data Types

- Reference:
  The Rust Programming Language (Book)  
   [Common Programming Concepts/Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

Rust는 정적으로 타입이 지정된 언어이므로 컴파일러가 컴파일 하는 시점에 모든 변수의 타입을 알고 있어야 합니다. 만약 타입을 지정하지 않는다면, 컴파일 에러를 뱉어냅니다.

## Scalar Types

스칼라 타입은 단일 값을 나타냅니다. Rust에는 정수, 부동 소수점 숫자, 부울, 문자의 네 가지 기본 스칼라 유형이 있습니다.

### Integer Types

Integer는 정수는 양의 정수, 음의 정수, 0을 포함하는데, 음의 정수처럼 부호를 갖는 경우에는 `i**`의 타입을 갖고, 부호가 없는 값은 갖는 경우에는 `u**`의 타입을 갖습니다.

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

위 테이블에 있는 타입들은 비트를 n이라고 했을 때, 마이너스 부호도 표현이 가능한 `i**`는 $-\frac{2^n}{2}$ 부터 $\frac{2^n}{2}-1$만큼의 범위를 표시할 수 있는 데이터이고, 0부터 플러스 부호만을 표현할 수 있는 `u**`타입은 0 부터 $2^n-1$ 만큼의 범위를 표시할 수 있습니다.

- 만약 정의하였던 숫자 타입을 초과하였을 때에는 에러가 발생하니, 주의하여야합니다.

그리고 Rust에서는 숫자를 정의할 때, 구분자(\_)를 두어서 사람이 숫자를 확인할 때, 더욱 쉽게 읽을 수 있습니다.

```rust
const NUM:u32 = 1_000_000;
```

10진법이 아닌 다른 진법에서도 사용가능하니 알아두면 좋습니다.

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

### Floating-Point Types

소수를 표현하는 타입인 Floating-point type입니다. Rust에서는 기본적으로 64 bit인 `f64` 타입을 사용하며, `f32`, `f63` 두 가지만 선언할 수 있습니다.

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

### Numeric Operations

Rust는 덧셈, 뺄셈, 곱셈, 나눗셈, 나머지와 같은 모든 숫자 유형에서 기대할 수 있는 기본적인 수학적 연산을 지원합니다.

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

- 만약, Rust에서 제공하는 모든 연산자가 궁금하시다면, [여기](https://doc.rust-lang.org/book/appendix-02-operators.html) 부록 페이지를 참고하세요.

### The Boolean Type

대부분의 다른 프로그래밍 언어와 마찬가지로 Rust의 Boolean 유형에는 참과 거짓의 두 가지 값이 있습니다. Boolean의 크기는 1바이트입니다. Rust의 Boolean 유형은 `bool`을 사용하여 지정됩니다. 예를 들면

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

### The Character Type

Rust의 Character Type은 언어에서 가장 원시적인 유형입니다. 다음은 문자(`char`) 값을 선언하는 몇 가지 예입니다:

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

큰따옴표를 사용하는 문자열 리터럴과 달리 작은따옴표로 문자 리터럴을 지정한다는 점에 유의하세요. Rust의 문자 유형은 4바이트 크기이며 유니코드 스칼라 값을 나타내므로 ASCII보다 훨씬 더 많은 것을 나타낼 수 있습니다.

- 참조고 문자가 아니라 문자열은 다음과 같이 처리 합니다. (큰따옴표 사용.)

```rust
let s: &str = "hello";  // 불변적인 문자열 슬라이스
let mut s: String = String::from("hello");  // 가변적인 String

s.push_str(", world!");  // String에 문자열 추가
println!("{}", s);  // 출력: "hello, world!"

```

## Compound Types

Compound Types은 여러 값을 하나의 타입으로 그룹화할 수 있습니다. Rust에는 튜플과 배열이라는 두 가지 기본 복합 유형이 있습니다.

### The Tuple Type

튜플은 다양한 유형을 가진 여러 값을 하나의 복합 유형으로 그룹화하는 일반적인 방법입니다. 튜플은 길이가 고정되어 있어 한 번 선언하면 크기가 커지거나 줄어들지 않습니다.

튜플의 예시:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

튜플은 이렇게 타입과 함께 선언할 수 있고, 요소들의 타입이 모두 같을 필요는 없습니다.

튜플은 복합 요소로 간주되므로 변수 tup은 전체 튜플에 바인딩됩니다. 튜플에서 개별 값을 가져오려면 다음과 같이 패턴 일치를 사용하여 튜플 값을 `destructuring` 할 수 있습니다:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    // The value of y is: 6.4
}
```

이렇게까지 해야하나? 싶으시다면, 아래처럼 직점 요소의 인덱스를 이용하여 접근할 수 있습니다.

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

### The Array Type

여러 값의 컬렉션을 만드는 또 다른 방법은 배열을 사용하는 것입니다. 튜플과 달리 배열의 모든 요소는 동일한 유형을 가져야 합니다. 다른 언어의 배열과 달리 Rust의 배열은 길이가 고정되어 있습니다.

Array Type의 예시:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

특정 타입은 다음과 같이 지정합니다:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

여기서 i32는 각 요소의 유형입니다. 세미콜론 뒤의 숫자 5는 배열에 5개의 요소가 포함되어 있음을 나타냅니다.

또, Array를 특정 값으로 초기화하는 것이 가능합니다.

```rust
    let a = [3; 5];
    // [3, 3, 3, 3, 3]
```

### Accessing Array Elements

위에서 설명되어진 Array의 요소는 `[index]`를 이용하여 접근하는 것이 가능합니다. 예시:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

---

추가 내용

# Isize

Rust에서 `isize`는 시스템 아키텍처에 의존하는 정수 타입입니다. `isize`의 크기는 시스템이 32비트인지 64비트인지에 따라 달라집니다.

`isize`는 부호가 있는 정수를 나타내며, 'i'는 'integer'를, 'size'는 시스템 아키텍처의 포인터 크기에 의존하는 크기를 나타냅니다. 이 타입은 주로 시스템 메모리에 대한 인덱싱이나 참조 시에 사용됩니다.

32비트 시스템에서 `isize`는 `-2^31`에서 `2^31 - 1`까지의 값을 가질 수 있으며, 64비트 시스템에서는 `-2^63`에서 `2^63 - 1`까지의 값을 가질 수 있습니다. 이는 각각 4GB와 8EB(엑사바이트)의 메모리 공간을 주소 지정할 수 있음을 의미합니다.

그러나 일반적인 상황에서는 크기가 시스템에 의존하는 `isize`나 `usize` 대신에 명시적인 크기를 가진 정수 타입 (예: `i32`, `i64`)을 사용하는 것이 좋습니다. 이는 코드의 이식성을 높이며, 예상치 못한 오버플로우나 언더플로우 문제를 방지할 수 있기 때문입니다.
