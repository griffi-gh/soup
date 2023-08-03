macro_rules! const_block {
  {$type: ident, $block: block} => {{
    const VALUE: $type = $block;
    VALUE
  }};
}
pub(crate) use const_block;

macro_rules! const_wrap {
  ($type: ident ($value: expr)) => {
    $crate::util::const_block!{$type, {$type($value)}}
  }
}
pub(crate) use const_wrap;

macro_rules! box_array {
  [$val:expr ; $len:expr] => {{
    if true {
      let boxed_slice = vec![$val; $len].into_boxed_slice();
      let ptr = ::std::boxed::Box::into_raw(boxed_slice) as *mut [_; $len];
      unsafe { Box::from_raw(ptr) }
    } else {
      Box::new([$val; $len])
    }
  }};
}
pub(crate) use box_array;
