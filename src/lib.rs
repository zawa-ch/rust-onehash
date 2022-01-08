pub mod md5;

/// 任意長のビット列から固定長ビットのダイジェストを生成する関数を表します。
trait HashFunction {
    /// ダイジェストに使用される型。
    type DigestType;
    /// 空の`HashFunction`を作成し、ハッシュ値の計算を行えるようにします。
    fn new() -> Self;
    ///
    fn put_value(&mut self, value: &u8);
    ///
    fn put_message<'a, I: IntoIterator<Item = &'a u8>>(&mut self, message: I) {
        for item in message { self.put_value(item); }
    }
    /// 現在のメッセージダイジェストを取得します。
    fn digest(&self) -> Self::DigestType;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
