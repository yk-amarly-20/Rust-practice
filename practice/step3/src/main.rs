fn main() {
    // データ型
    // 浮動少数点数
    let x = 2.0;    // 倍精度 f64
    let y: f32 = 3.0;   // 単精度 f32

    // bool
    let t = true;
    let f: bool = false;

    // 文字
    let c = 'c';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';  // これ地味にいいね, juliaのやつと一緒か

    // タプル
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // 配列
    // 固定長
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // 配列の長さ以上のindexを指定するとパニック
}
