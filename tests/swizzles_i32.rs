// Generated by swizzlegen. Do not edit.
#[macro_use]
mod support;
use glam::*;

glam_test!(test_ivec4_swizzles, {
    let v = ivec4(1_i32, 2_i32, 3_i32, 4_i32);
    let rhs3 = ivec3(11_i32, 12_i32, 13_i32);
    let rhs2 = ivec2(11_i32, 12_i32);
    assert_eq!(v, v.xyzw());
    assert_eq!(v.xxxx(), ivec4(1_i32, 1_i32, 1_i32, 1_i32));
    assert_eq!(v.xxxy(), ivec4(1_i32, 1_i32, 1_i32, 2_i32));
    assert_eq!(v.xxxz(), ivec4(1_i32, 1_i32, 1_i32, 3_i32));
    assert_eq!(v.xxxw(), ivec4(1_i32, 1_i32, 1_i32, 4_i32));
    assert_eq!(v.xxyx(), ivec4(1_i32, 1_i32, 2_i32, 1_i32));
    assert_eq!(v.xxyy(), ivec4(1_i32, 1_i32, 2_i32, 2_i32));
    assert_eq!(v.xxyz(), ivec4(1_i32, 1_i32, 2_i32, 3_i32));
    assert_eq!(v.xxyw(), ivec4(1_i32, 1_i32, 2_i32, 4_i32));
    assert_eq!(v.xxzx(), ivec4(1_i32, 1_i32, 3_i32, 1_i32));
    assert_eq!(v.xxzy(), ivec4(1_i32, 1_i32, 3_i32, 2_i32));
    assert_eq!(v.xxzz(), ivec4(1_i32, 1_i32, 3_i32, 3_i32));
    assert_eq!(v.xxzw(), ivec4(1_i32, 1_i32, 3_i32, 4_i32));
    assert_eq!(v.xxwx(), ivec4(1_i32, 1_i32, 4_i32, 1_i32));
    assert_eq!(v.xxwy(), ivec4(1_i32, 1_i32, 4_i32, 2_i32));
    assert_eq!(v.xxwz(), ivec4(1_i32, 1_i32, 4_i32, 3_i32));
    assert_eq!(v.xxww(), ivec4(1_i32, 1_i32, 4_i32, 4_i32));
    assert_eq!(v.xyxx(), ivec4(1_i32, 2_i32, 1_i32, 1_i32));
    assert_eq!(v.xyxy(), ivec4(1_i32, 2_i32, 1_i32, 2_i32));
    assert_eq!(v.xyxz(), ivec4(1_i32, 2_i32, 1_i32, 3_i32));
    assert_eq!(v.xyxw(), ivec4(1_i32, 2_i32, 1_i32, 4_i32));
    assert_eq!(v.xyyx(), ivec4(1_i32, 2_i32, 2_i32, 1_i32));
    assert_eq!(v.xyyy(), ivec4(1_i32, 2_i32, 2_i32, 2_i32));
    assert_eq!(v.xyyz(), ivec4(1_i32, 2_i32, 2_i32, 3_i32));
    assert_eq!(v.xyyw(), ivec4(1_i32, 2_i32, 2_i32, 4_i32));
    assert_eq!(v.xyzx(), ivec4(1_i32, 2_i32, 3_i32, 1_i32));
    assert_eq!(v.xyzy(), ivec4(1_i32, 2_i32, 3_i32, 2_i32));
    assert_eq!(v.xyzz(), ivec4(1_i32, 2_i32, 3_i32, 3_i32));
    assert_eq!(v.xywx(), ivec4(1_i32, 2_i32, 4_i32, 1_i32));
    assert_eq!(v.xywy(), ivec4(1_i32, 2_i32, 4_i32, 2_i32));
    assert_eq!(v.xywz(), ivec4(1_i32, 2_i32, 4_i32, 3_i32));
    assert_eq!(v.xyww(), ivec4(1_i32, 2_i32, 4_i32, 4_i32));
    assert_eq!(v.xzxx(), ivec4(1_i32, 3_i32, 1_i32, 1_i32));
    assert_eq!(v.xzxy(), ivec4(1_i32, 3_i32, 1_i32, 2_i32));
    assert_eq!(v.xzxz(), ivec4(1_i32, 3_i32, 1_i32, 3_i32));
    assert_eq!(v.xzxw(), ivec4(1_i32, 3_i32, 1_i32, 4_i32));
    assert_eq!(v.xzyx(), ivec4(1_i32, 3_i32, 2_i32, 1_i32));
    assert_eq!(v.xzyy(), ivec4(1_i32, 3_i32, 2_i32, 2_i32));
    assert_eq!(v.xzyz(), ivec4(1_i32, 3_i32, 2_i32, 3_i32));
    assert_eq!(v.xzyw(), ivec4(1_i32, 3_i32, 2_i32, 4_i32));
    assert_eq!(v.xzzx(), ivec4(1_i32, 3_i32, 3_i32, 1_i32));
    assert_eq!(v.xzzy(), ivec4(1_i32, 3_i32, 3_i32, 2_i32));
    assert_eq!(v.xzzz(), ivec4(1_i32, 3_i32, 3_i32, 3_i32));
    assert_eq!(v.xzzw(), ivec4(1_i32, 3_i32, 3_i32, 4_i32));
    assert_eq!(v.xzwx(), ivec4(1_i32, 3_i32, 4_i32, 1_i32));
    assert_eq!(v.xzwy(), ivec4(1_i32, 3_i32, 4_i32, 2_i32));
    assert_eq!(v.xzwz(), ivec4(1_i32, 3_i32, 4_i32, 3_i32));
    assert_eq!(v.xzww(), ivec4(1_i32, 3_i32, 4_i32, 4_i32));
    assert_eq!(v.xwxx(), ivec4(1_i32, 4_i32, 1_i32, 1_i32));
    assert_eq!(v.xwxy(), ivec4(1_i32, 4_i32, 1_i32, 2_i32));
    assert_eq!(v.xwxz(), ivec4(1_i32, 4_i32, 1_i32, 3_i32));
    assert_eq!(v.xwxw(), ivec4(1_i32, 4_i32, 1_i32, 4_i32));
    assert_eq!(v.xwyx(), ivec4(1_i32, 4_i32, 2_i32, 1_i32));
    assert_eq!(v.xwyy(), ivec4(1_i32, 4_i32, 2_i32, 2_i32));
    assert_eq!(v.xwyz(), ivec4(1_i32, 4_i32, 2_i32, 3_i32));
    assert_eq!(v.xwyw(), ivec4(1_i32, 4_i32, 2_i32, 4_i32));
    assert_eq!(v.xwzx(), ivec4(1_i32, 4_i32, 3_i32, 1_i32));
    assert_eq!(v.xwzy(), ivec4(1_i32, 4_i32, 3_i32, 2_i32));
    assert_eq!(v.xwzz(), ivec4(1_i32, 4_i32, 3_i32, 3_i32));
    assert_eq!(v.xwzw(), ivec4(1_i32, 4_i32, 3_i32, 4_i32));
    assert_eq!(v.xwwx(), ivec4(1_i32, 4_i32, 4_i32, 1_i32));
    assert_eq!(v.xwwy(), ivec4(1_i32, 4_i32, 4_i32, 2_i32));
    assert_eq!(v.xwwz(), ivec4(1_i32, 4_i32, 4_i32, 3_i32));
    assert_eq!(v.xwww(), ivec4(1_i32, 4_i32, 4_i32, 4_i32));
    assert_eq!(v.yxxx(), ivec4(2_i32, 1_i32, 1_i32, 1_i32));
    assert_eq!(v.yxxy(), ivec4(2_i32, 1_i32, 1_i32, 2_i32));
    assert_eq!(v.yxxz(), ivec4(2_i32, 1_i32, 1_i32, 3_i32));
    assert_eq!(v.yxxw(), ivec4(2_i32, 1_i32, 1_i32, 4_i32));
    assert_eq!(v.yxyx(), ivec4(2_i32, 1_i32, 2_i32, 1_i32));
    assert_eq!(v.yxyy(), ivec4(2_i32, 1_i32, 2_i32, 2_i32));
    assert_eq!(v.yxyz(), ivec4(2_i32, 1_i32, 2_i32, 3_i32));
    assert_eq!(v.yxyw(), ivec4(2_i32, 1_i32, 2_i32, 4_i32));
    assert_eq!(v.yxzx(), ivec4(2_i32, 1_i32, 3_i32, 1_i32));
    assert_eq!(v.yxzy(), ivec4(2_i32, 1_i32, 3_i32, 2_i32));
    assert_eq!(v.yxzz(), ivec4(2_i32, 1_i32, 3_i32, 3_i32));
    assert_eq!(v.yxzw(), ivec4(2_i32, 1_i32, 3_i32, 4_i32));
    assert_eq!(v.yxwx(), ivec4(2_i32, 1_i32, 4_i32, 1_i32));
    assert_eq!(v.yxwy(), ivec4(2_i32, 1_i32, 4_i32, 2_i32));
    assert_eq!(v.yxwz(), ivec4(2_i32, 1_i32, 4_i32, 3_i32));
    assert_eq!(v.yxww(), ivec4(2_i32, 1_i32, 4_i32, 4_i32));
    assert_eq!(v.yyxx(), ivec4(2_i32, 2_i32, 1_i32, 1_i32));
    assert_eq!(v.yyxy(), ivec4(2_i32, 2_i32, 1_i32, 2_i32));
    assert_eq!(v.yyxz(), ivec4(2_i32, 2_i32, 1_i32, 3_i32));
    assert_eq!(v.yyxw(), ivec4(2_i32, 2_i32, 1_i32, 4_i32));
    assert_eq!(v.yyyx(), ivec4(2_i32, 2_i32, 2_i32, 1_i32));
    assert_eq!(v.yyyy(), ivec4(2_i32, 2_i32, 2_i32, 2_i32));
    assert_eq!(v.yyyz(), ivec4(2_i32, 2_i32, 2_i32, 3_i32));
    assert_eq!(v.yyyw(), ivec4(2_i32, 2_i32, 2_i32, 4_i32));
    assert_eq!(v.yyzx(), ivec4(2_i32, 2_i32, 3_i32, 1_i32));
    assert_eq!(v.yyzy(), ivec4(2_i32, 2_i32, 3_i32, 2_i32));
    assert_eq!(v.yyzz(), ivec4(2_i32, 2_i32, 3_i32, 3_i32));
    assert_eq!(v.yyzw(), ivec4(2_i32, 2_i32, 3_i32, 4_i32));
    assert_eq!(v.yywx(), ivec4(2_i32, 2_i32, 4_i32, 1_i32));
    assert_eq!(v.yywy(), ivec4(2_i32, 2_i32, 4_i32, 2_i32));
    assert_eq!(v.yywz(), ivec4(2_i32, 2_i32, 4_i32, 3_i32));
    assert_eq!(v.yyww(), ivec4(2_i32, 2_i32, 4_i32, 4_i32));
    assert_eq!(v.yzxx(), ivec4(2_i32, 3_i32, 1_i32, 1_i32));
    assert_eq!(v.yzxy(), ivec4(2_i32, 3_i32, 1_i32, 2_i32));
    assert_eq!(v.yzxz(), ivec4(2_i32, 3_i32, 1_i32, 3_i32));
    assert_eq!(v.yzxw(), ivec4(2_i32, 3_i32, 1_i32, 4_i32));
    assert_eq!(v.yzyx(), ivec4(2_i32, 3_i32, 2_i32, 1_i32));
    assert_eq!(v.yzyy(), ivec4(2_i32, 3_i32, 2_i32, 2_i32));
    assert_eq!(v.yzyz(), ivec4(2_i32, 3_i32, 2_i32, 3_i32));
    assert_eq!(v.yzyw(), ivec4(2_i32, 3_i32, 2_i32, 4_i32));
    assert_eq!(v.yzzx(), ivec4(2_i32, 3_i32, 3_i32, 1_i32));
    assert_eq!(v.yzzy(), ivec4(2_i32, 3_i32, 3_i32, 2_i32));
    assert_eq!(v.yzzz(), ivec4(2_i32, 3_i32, 3_i32, 3_i32));
    assert_eq!(v.yzzw(), ivec4(2_i32, 3_i32, 3_i32, 4_i32));
    assert_eq!(v.yzwx(), ivec4(2_i32, 3_i32, 4_i32, 1_i32));
    assert_eq!(v.yzwy(), ivec4(2_i32, 3_i32, 4_i32, 2_i32));
    assert_eq!(v.yzwz(), ivec4(2_i32, 3_i32, 4_i32, 3_i32));
    assert_eq!(v.yzww(), ivec4(2_i32, 3_i32, 4_i32, 4_i32));
    assert_eq!(v.ywxx(), ivec4(2_i32, 4_i32, 1_i32, 1_i32));
    assert_eq!(v.ywxy(), ivec4(2_i32, 4_i32, 1_i32, 2_i32));
    assert_eq!(v.ywxz(), ivec4(2_i32, 4_i32, 1_i32, 3_i32));
    assert_eq!(v.ywxw(), ivec4(2_i32, 4_i32, 1_i32, 4_i32));
    assert_eq!(v.ywyx(), ivec4(2_i32, 4_i32, 2_i32, 1_i32));
    assert_eq!(v.ywyy(), ivec4(2_i32, 4_i32, 2_i32, 2_i32));
    assert_eq!(v.ywyz(), ivec4(2_i32, 4_i32, 2_i32, 3_i32));
    assert_eq!(v.ywyw(), ivec4(2_i32, 4_i32, 2_i32, 4_i32));
    assert_eq!(v.ywzx(), ivec4(2_i32, 4_i32, 3_i32, 1_i32));
    assert_eq!(v.ywzy(), ivec4(2_i32, 4_i32, 3_i32, 2_i32));
    assert_eq!(v.ywzz(), ivec4(2_i32, 4_i32, 3_i32, 3_i32));
    assert_eq!(v.ywzw(), ivec4(2_i32, 4_i32, 3_i32, 4_i32));
    assert_eq!(v.ywwx(), ivec4(2_i32, 4_i32, 4_i32, 1_i32));
    assert_eq!(v.ywwy(), ivec4(2_i32, 4_i32, 4_i32, 2_i32));
    assert_eq!(v.ywwz(), ivec4(2_i32, 4_i32, 4_i32, 3_i32));
    assert_eq!(v.ywww(), ivec4(2_i32, 4_i32, 4_i32, 4_i32));
    assert_eq!(v.zxxx(), ivec4(3_i32, 1_i32, 1_i32, 1_i32));
    assert_eq!(v.zxxy(), ivec4(3_i32, 1_i32, 1_i32, 2_i32));
    assert_eq!(v.zxxz(), ivec4(3_i32, 1_i32, 1_i32, 3_i32));
    assert_eq!(v.zxxw(), ivec4(3_i32, 1_i32, 1_i32, 4_i32));
    assert_eq!(v.zxyx(), ivec4(3_i32, 1_i32, 2_i32, 1_i32));
    assert_eq!(v.zxyy(), ivec4(3_i32, 1_i32, 2_i32, 2_i32));
    assert_eq!(v.zxyz(), ivec4(3_i32, 1_i32, 2_i32, 3_i32));
    assert_eq!(v.zxyw(), ivec4(3_i32, 1_i32, 2_i32, 4_i32));
    assert_eq!(v.zxzx(), ivec4(3_i32, 1_i32, 3_i32, 1_i32));
    assert_eq!(v.zxzy(), ivec4(3_i32, 1_i32, 3_i32, 2_i32));
    assert_eq!(v.zxzz(), ivec4(3_i32, 1_i32, 3_i32, 3_i32));
    assert_eq!(v.zxzw(), ivec4(3_i32, 1_i32, 3_i32, 4_i32));
    assert_eq!(v.zxwx(), ivec4(3_i32, 1_i32, 4_i32, 1_i32));
    assert_eq!(v.zxwy(), ivec4(3_i32, 1_i32, 4_i32, 2_i32));
    assert_eq!(v.zxwz(), ivec4(3_i32, 1_i32, 4_i32, 3_i32));
    assert_eq!(v.zxww(), ivec4(3_i32, 1_i32, 4_i32, 4_i32));
    assert_eq!(v.zyxx(), ivec4(3_i32, 2_i32, 1_i32, 1_i32));
    assert_eq!(v.zyxy(), ivec4(3_i32, 2_i32, 1_i32, 2_i32));
    assert_eq!(v.zyxz(), ivec4(3_i32, 2_i32, 1_i32, 3_i32));
    assert_eq!(v.zyxw(), ivec4(3_i32, 2_i32, 1_i32, 4_i32));
    assert_eq!(v.zyyx(), ivec4(3_i32, 2_i32, 2_i32, 1_i32));
    assert_eq!(v.zyyy(), ivec4(3_i32, 2_i32, 2_i32, 2_i32));
    assert_eq!(v.zyyz(), ivec4(3_i32, 2_i32, 2_i32, 3_i32));
    assert_eq!(v.zyyw(), ivec4(3_i32, 2_i32, 2_i32, 4_i32));
    assert_eq!(v.zyzx(), ivec4(3_i32, 2_i32, 3_i32, 1_i32));
    assert_eq!(v.zyzy(), ivec4(3_i32, 2_i32, 3_i32, 2_i32));
    assert_eq!(v.zyzz(), ivec4(3_i32, 2_i32, 3_i32, 3_i32));
    assert_eq!(v.zyzw(), ivec4(3_i32, 2_i32, 3_i32, 4_i32));
    assert_eq!(v.zywx(), ivec4(3_i32, 2_i32, 4_i32, 1_i32));
    assert_eq!(v.zywy(), ivec4(3_i32, 2_i32, 4_i32, 2_i32));
    assert_eq!(v.zywz(), ivec4(3_i32, 2_i32, 4_i32, 3_i32));
    assert_eq!(v.zyww(), ivec4(3_i32, 2_i32, 4_i32, 4_i32));
    assert_eq!(v.zzxx(), ivec4(3_i32, 3_i32, 1_i32, 1_i32));
    assert_eq!(v.zzxy(), ivec4(3_i32, 3_i32, 1_i32, 2_i32));
    assert_eq!(v.zzxz(), ivec4(3_i32, 3_i32, 1_i32, 3_i32));
    assert_eq!(v.zzxw(), ivec4(3_i32, 3_i32, 1_i32, 4_i32));
    assert_eq!(v.zzyx(), ivec4(3_i32, 3_i32, 2_i32, 1_i32));
    assert_eq!(v.zzyy(), ivec4(3_i32, 3_i32, 2_i32, 2_i32));
    assert_eq!(v.zzyz(), ivec4(3_i32, 3_i32, 2_i32, 3_i32));
    assert_eq!(v.zzyw(), ivec4(3_i32, 3_i32, 2_i32, 4_i32));
    assert_eq!(v.zzzx(), ivec4(3_i32, 3_i32, 3_i32, 1_i32));
    assert_eq!(v.zzzy(), ivec4(3_i32, 3_i32, 3_i32, 2_i32));
    assert_eq!(v.zzzz(), ivec4(3_i32, 3_i32, 3_i32, 3_i32));
    assert_eq!(v.zzzw(), ivec4(3_i32, 3_i32, 3_i32, 4_i32));
    assert_eq!(v.zzwx(), ivec4(3_i32, 3_i32, 4_i32, 1_i32));
    assert_eq!(v.zzwy(), ivec4(3_i32, 3_i32, 4_i32, 2_i32));
    assert_eq!(v.zzwz(), ivec4(3_i32, 3_i32, 4_i32, 3_i32));
    assert_eq!(v.zzww(), ivec4(3_i32, 3_i32, 4_i32, 4_i32));
    assert_eq!(v.zwxx(), ivec4(3_i32, 4_i32, 1_i32, 1_i32));
    assert_eq!(v.zwxy(), ivec4(3_i32, 4_i32, 1_i32, 2_i32));
    assert_eq!(v.zwxz(), ivec4(3_i32, 4_i32, 1_i32, 3_i32));
    assert_eq!(v.zwxw(), ivec4(3_i32, 4_i32, 1_i32, 4_i32));
    assert_eq!(v.zwyx(), ivec4(3_i32, 4_i32, 2_i32, 1_i32));
    assert_eq!(v.zwyy(), ivec4(3_i32, 4_i32, 2_i32, 2_i32));
    assert_eq!(v.zwyz(), ivec4(3_i32, 4_i32, 2_i32, 3_i32));
    assert_eq!(v.zwyw(), ivec4(3_i32, 4_i32, 2_i32, 4_i32));
    assert_eq!(v.zwzx(), ivec4(3_i32, 4_i32, 3_i32, 1_i32));
    assert_eq!(v.zwzy(), ivec4(3_i32, 4_i32, 3_i32, 2_i32));
    assert_eq!(v.zwzz(), ivec4(3_i32, 4_i32, 3_i32, 3_i32));
    assert_eq!(v.zwzw(), ivec4(3_i32, 4_i32, 3_i32, 4_i32));
    assert_eq!(v.zwwx(), ivec4(3_i32, 4_i32, 4_i32, 1_i32));
    assert_eq!(v.zwwy(), ivec4(3_i32, 4_i32, 4_i32, 2_i32));
    assert_eq!(v.zwwz(), ivec4(3_i32, 4_i32, 4_i32, 3_i32));
    assert_eq!(v.zwww(), ivec4(3_i32, 4_i32, 4_i32, 4_i32));
    assert_eq!(v.wxxx(), ivec4(4_i32, 1_i32, 1_i32, 1_i32));
    assert_eq!(v.wxxy(), ivec4(4_i32, 1_i32, 1_i32, 2_i32));
    assert_eq!(v.wxxz(), ivec4(4_i32, 1_i32, 1_i32, 3_i32));
    assert_eq!(v.wxxw(), ivec4(4_i32, 1_i32, 1_i32, 4_i32));
    assert_eq!(v.wxyx(), ivec4(4_i32, 1_i32, 2_i32, 1_i32));
    assert_eq!(v.wxyy(), ivec4(4_i32, 1_i32, 2_i32, 2_i32));
    assert_eq!(v.wxyz(), ivec4(4_i32, 1_i32, 2_i32, 3_i32));
    assert_eq!(v.wxyw(), ivec4(4_i32, 1_i32, 2_i32, 4_i32));
    assert_eq!(v.wxzx(), ivec4(4_i32, 1_i32, 3_i32, 1_i32));
    assert_eq!(v.wxzy(), ivec4(4_i32, 1_i32, 3_i32, 2_i32));
    assert_eq!(v.wxzz(), ivec4(4_i32, 1_i32, 3_i32, 3_i32));
    assert_eq!(v.wxzw(), ivec4(4_i32, 1_i32, 3_i32, 4_i32));
    assert_eq!(v.wxwx(), ivec4(4_i32, 1_i32, 4_i32, 1_i32));
    assert_eq!(v.wxwy(), ivec4(4_i32, 1_i32, 4_i32, 2_i32));
    assert_eq!(v.wxwz(), ivec4(4_i32, 1_i32, 4_i32, 3_i32));
    assert_eq!(v.wxww(), ivec4(4_i32, 1_i32, 4_i32, 4_i32));
    assert_eq!(v.wyxx(), ivec4(4_i32, 2_i32, 1_i32, 1_i32));
    assert_eq!(v.wyxy(), ivec4(4_i32, 2_i32, 1_i32, 2_i32));
    assert_eq!(v.wyxz(), ivec4(4_i32, 2_i32, 1_i32, 3_i32));
    assert_eq!(v.wyxw(), ivec4(4_i32, 2_i32, 1_i32, 4_i32));
    assert_eq!(v.wyyx(), ivec4(4_i32, 2_i32, 2_i32, 1_i32));
    assert_eq!(v.wyyy(), ivec4(4_i32, 2_i32, 2_i32, 2_i32));
    assert_eq!(v.wyyz(), ivec4(4_i32, 2_i32, 2_i32, 3_i32));
    assert_eq!(v.wyyw(), ivec4(4_i32, 2_i32, 2_i32, 4_i32));
    assert_eq!(v.wyzx(), ivec4(4_i32, 2_i32, 3_i32, 1_i32));
    assert_eq!(v.wyzy(), ivec4(4_i32, 2_i32, 3_i32, 2_i32));
    assert_eq!(v.wyzz(), ivec4(4_i32, 2_i32, 3_i32, 3_i32));
    assert_eq!(v.wyzw(), ivec4(4_i32, 2_i32, 3_i32, 4_i32));
    assert_eq!(v.wywx(), ivec4(4_i32, 2_i32, 4_i32, 1_i32));
    assert_eq!(v.wywy(), ivec4(4_i32, 2_i32, 4_i32, 2_i32));
    assert_eq!(v.wywz(), ivec4(4_i32, 2_i32, 4_i32, 3_i32));
    assert_eq!(v.wyww(), ivec4(4_i32, 2_i32, 4_i32, 4_i32));
    assert_eq!(v.wzxx(), ivec4(4_i32, 3_i32, 1_i32, 1_i32));
    assert_eq!(v.wzxy(), ivec4(4_i32, 3_i32, 1_i32, 2_i32));
    assert_eq!(v.wzxz(), ivec4(4_i32, 3_i32, 1_i32, 3_i32));
    assert_eq!(v.wzxw(), ivec4(4_i32, 3_i32, 1_i32, 4_i32));
    assert_eq!(v.wzyx(), ivec4(4_i32, 3_i32, 2_i32, 1_i32));
    assert_eq!(v.wzyy(), ivec4(4_i32, 3_i32, 2_i32, 2_i32));
    assert_eq!(v.wzyz(), ivec4(4_i32, 3_i32, 2_i32, 3_i32));
    assert_eq!(v.wzyw(), ivec4(4_i32, 3_i32, 2_i32, 4_i32));
    assert_eq!(v.wzzx(), ivec4(4_i32, 3_i32, 3_i32, 1_i32));
    assert_eq!(v.wzzy(), ivec4(4_i32, 3_i32, 3_i32, 2_i32));
    assert_eq!(v.wzzz(), ivec4(4_i32, 3_i32, 3_i32, 3_i32));
    assert_eq!(v.wzzw(), ivec4(4_i32, 3_i32, 3_i32, 4_i32));
    assert_eq!(v.wzwx(), ivec4(4_i32, 3_i32, 4_i32, 1_i32));
    assert_eq!(v.wzwy(), ivec4(4_i32, 3_i32, 4_i32, 2_i32));
    assert_eq!(v.wzwz(), ivec4(4_i32, 3_i32, 4_i32, 3_i32));
    assert_eq!(v.wzww(), ivec4(4_i32, 3_i32, 4_i32, 4_i32));
    assert_eq!(v.wwxx(), ivec4(4_i32, 4_i32, 1_i32, 1_i32));
    assert_eq!(v.wwxy(), ivec4(4_i32, 4_i32, 1_i32, 2_i32));
    assert_eq!(v.wwxz(), ivec4(4_i32, 4_i32, 1_i32, 3_i32));
    assert_eq!(v.wwxw(), ivec4(4_i32, 4_i32, 1_i32, 4_i32));
    assert_eq!(v.wwyx(), ivec4(4_i32, 4_i32, 2_i32, 1_i32));
    assert_eq!(v.wwyy(), ivec4(4_i32, 4_i32, 2_i32, 2_i32));
    assert_eq!(v.wwyz(), ivec4(4_i32, 4_i32, 2_i32, 3_i32));
    assert_eq!(v.wwyw(), ivec4(4_i32, 4_i32, 2_i32, 4_i32));
    assert_eq!(v.wwzx(), ivec4(4_i32, 4_i32, 3_i32, 1_i32));
    assert_eq!(v.wwzy(), ivec4(4_i32, 4_i32, 3_i32, 2_i32));
    assert_eq!(v.wwzz(), ivec4(4_i32, 4_i32, 3_i32, 3_i32));
    assert_eq!(v.wwzw(), ivec4(4_i32, 4_i32, 3_i32, 4_i32));
    assert_eq!(v.wwwx(), ivec4(4_i32, 4_i32, 4_i32, 1_i32));
    assert_eq!(v.wwwy(), ivec4(4_i32, 4_i32, 4_i32, 2_i32));
    assert_eq!(v.wwwz(), ivec4(4_i32, 4_i32, 4_i32, 3_i32));
    assert_eq!(v.wwww(), ivec4(4_i32, 4_i32, 4_i32, 4_i32));
    assert_eq!(v.xxx(), ivec3(1_i32, 1_i32, 1_i32));
    assert_eq!(v.xxy(), ivec3(1_i32, 1_i32, 2_i32));
    assert_eq!(v.xxz(), ivec3(1_i32, 1_i32, 3_i32));
    assert_eq!(v.xxw(), ivec3(1_i32, 1_i32, 4_i32));
    assert_eq!(v.xyx(), ivec3(1_i32, 2_i32, 1_i32));
    assert_eq!(v.xyy(), ivec3(1_i32, 2_i32, 2_i32));
    assert_eq!(v.xyz(), ivec3(1_i32, 2_i32, 3_i32));
    assert_eq!(v.xyw(), ivec3(1_i32, 2_i32, 4_i32));
    assert_eq!(v.xzx(), ivec3(1_i32, 3_i32, 1_i32));
    assert_eq!(v.xzy(), ivec3(1_i32, 3_i32, 2_i32));
    assert_eq!(v.xzz(), ivec3(1_i32, 3_i32, 3_i32));
    assert_eq!(v.xzw(), ivec3(1_i32, 3_i32, 4_i32));
    assert_eq!(v.xwx(), ivec3(1_i32, 4_i32, 1_i32));
    assert_eq!(v.xwy(), ivec3(1_i32, 4_i32, 2_i32));
    assert_eq!(v.xwz(), ivec3(1_i32, 4_i32, 3_i32));
    assert_eq!(v.xww(), ivec3(1_i32, 4_i32, 4_i32));
    assert_eq!(v.yxx(), ivec3(2_i32, 1_i32, 1_i32));
    assert_eq!(v.yxy(), ivec3(2_i32, 1_i32, 2_i32));
    assert_eq!(v.yxz(), ivec3(2_i32, 1_i32, 3_i32));
    assert_eq!(v.yxw(), ivec3(2_i32, 1_i32, 4_i32));
    assert_eq!(v.yyx(), ivec3(2_i32, 2_i32, 1_i32));
    assert_eq!(v.yyy(), ivec3(2_i32, 2_i32, 2_i32));
    assert_eq!(v.yyz(), ivec3(2_i32, 2_i32, 3_i32));
    assert_eq!(v.yyw(), ivec3(2_i32, 2_i32, 4_i32));
    assert_eq!(v.yzx(), ivec3(2_i32, 3_i32, 1_i32));
    assert_eq!(v.yzy(), ivec3(2_i32, 3_i32, 2_i32));
    assert_eq!(v.yzz(), ivec3(2_i32, 3_i32, 3_i32));
    assert_eq!(v.yzw(), ivec3(2_i32, 3_i32, 4_i32));
    assert_eq!(v.ywx(), ivec3(2_i32, 4_i32, 1_i32));
    assert_eq!(v.ywy(), ivec3(2_i32, 4_i32, 2_i32));
    assert_eq!(v.ywz(), ivec3(2_i32, 4_i32, 3_i32));
    assert_eq!(v.yww(), ivec3(2_i32, 4_i32, 4_i32));
    assert_eq!(v.zxx(), ivec3(3_i32, 1_i32, 1_i32));
    assert_eq!(v.zxy(), ivec3(3_i32, 1_i32, 2_i32));
    assert_eq!(v.zxz(), ivec3(3_i32, 1_i32, 3_i32));
    assert_eq!(v.zxw(), ivec3(3_i32, 1_i32, 4_i32));
    assert_eq!(v.zyx(), ivec3(3_i32, 2_i32, 1_i32));
    assert_eq!(v.zyy(), ivec3(3_i32, 2_i32, 2_i32));
    assert_eq!(v.zyz(), ivec3(3_i32, 2_i32, 3_i32));
    assert_eq!(v.zyw(), ivec3(3_i32, 2_i32, 4_i32));
    assert_eq!(v.zzx(), ivec3(3_i32, 3_i32, 1_i32));
    assert_eq!(v.zzy(), ivec3(3_i32, 3_i32, 2_i32));
    assert_eq!(v.zzz(), ivec3(3_i32, 3_i32, 3_i32));
    assert_eq!(v.zzw(), ivec3(3_i32, 3_i32, 4_i32));
    assert_eq!(v.zwx(), ivec3(3_i32, 4_i32, 1_i32));
    assert_eq!(v.zwy(), ivec3(3_i32, 4_i32, 2_i32));
    assert_eq!(v.zwz(), ivec3(3_i32, 4_i32, 3_i32));
    assert_eq!(v.zww(), ivec3(3_i32, 4_i32, 4_i32));
    assert_eq!(v.wxx(), ivec3(4_i32, 1_i32, 1_i32));
    assert_eq!(v.wxy(), ivec3(4_i32, 1_i32, 2_i32));
    assert_eq!(v.wxz(), ivec3(4_i32, 1_i32, 3_i32));
    assert_eq!(v.wxw(), ivec3(4_i32, 1_i32, 4_i32));
    assert_eq!(v.wyx(), ivec3(4_i32, 2_i32, 1_i32));
    assert_eq!(v.wyy(), ivec3(4_i32, 2_i32, 2_i32));
    assert_eq!(v.wyz(), ivec3(4_i32, 2_i32, 3_i32));
    assert_eq!(v.wyw(), ivec3(4_i32, 2_i32, 4_i32));
    assert_eq!(v.wzx(), ivec3(4_i32, 3_i32, 1_i32));
    assert_eq!(v.wzy(), ivec3(4_i32, 3_i32, 2_i32));
    assert_eq!(v.wzz(), ivec3(4_i32, 3_i32, 3_i32));
    assert_eq!(v.wzw(), ivec3(4_i32, 3_i32, 4_i32));
    assert_eq!(v.wwx(), ivec3(4_i32, 4_i32, 1_i32));
    assert_eq!(v.wwy(), ivec3(4_i32, 4_i32, 2_i32));
    assert_eq!(v.wwz(), ivec3(4_i32, 4_i32, 3_i32));
    assert_eq!(v.www(), ivec3(4_i32, 4_i32, 4_i32));
    assert_eq!(v.with_xyz(rhs3), ivec4(11_i32, 12_i32, 13_i32, 4_i32));
    assert_eq!(v.with_xyw(rhs3), ivec4(11_i32, 12_i32, 3_i32, 13_i32));
    assert_eq!(v.with_xzy(rhs3), ivec4(11_i32, 13_i32, 12_i32, 4_i32));
    assert_eq!(v.with_xzw(rhs3), ivec4(11_i32, 2_i32, 12_i32, 13_i32));
    assert_eq!(v.with_xwy(rhs3), ivec4(11_i32, 13_i32, 3_i32, 12_i32));
    assert_eq!(v.with_xwz(rhs3), ivec4(11_i32, 2_i32, 13_i32, 12_i32));
    assert_eq!(v.with_yxz(rhs3), ivec4(12_i32, 11_i32, 13_i32, 4_i32));
    assert_eq!(v.with_yxw(rhs3), ivec4(12_i32, 11_i32, 3_i32, 13_i32));
    assert_eq!(v.with_yzx(rhs3), ivec4(13_i32, 11_i32, 12_i32, 4_i32));
    assert_eq!(v.with_yzw(rhs3), ivec4(1_i32, 11_i32, 12_i32, 13_i32));
    assert_eq!(v.with_ywx(rhs3), ivec4(13_i32, 11_i32, 3_i32, 12_i32));
    assert_eq!(v.with_ywz(rhs3), ivec4(1_i32, 11_i32, 13_i32, 12_i32));
    assert_eq!(v.with_zxy(rhs3), ivec4(12_i32, 13_i32, 11_i32, 4_i32));
    assert_eq!(v.with_zxw(rhs3), ivec4(12_i32, 2_i32, 11_i32, 13_i32));
    assert_eq!(v.with_zyx(rhs3), ivec4(13_i32, 12_i32, 11_i32, 4_i32));
    assert_eq!(v.with_zyw(rhs3), ivec4(1_i32, 12_i32, 11_i32, 13_i32));
    assert_eq!(v.with_zwx(rhs3), ivec4(13_i32, 2_i32, 11_i32, 12_i32));
    assert_eq!(v.with_zwy(rhs3), ivec4(1_i32, 13_i32, 11_i32, 12_i32));
    assert_eq!(v.with_wxy(rhs3), ivec4(12_i32, 13_i32, 3_i32, 11_i32));
    assert_eq!(v.with_wxz(rhs3), ivec4(12_i32, 2_i32, 13_i32, 11_i32));
    assert_eq!(v.with_wyx(rhs3), ivec4(13_i32, 12_i32, 3_i32, 11_i32));
    assert_eq!(v.with_wyz(rhs3), ivec4(1_i32, 12_i32, 13_i32, 11_i32));
    assert_eq!(v.with_wzx(rhs3), ivec4(13_i32, 2_i32, 12_i32, 11_i32));
    assert_eq!(v.with_wzy(rhs3), ivec4(1_i32, 13_i32, 12_i32, 11_i32));
    assert_eq!(v.xx(), ivec2(1_i32, 1_i32));
    assert_eq!(v.xy(), ivec2(1_i32, 2_i32));
    assert_eq!(v.xz(), ivec2(1_i32, 3_i32));
    assert_eq!(v.xw(), ivec2(1_i32, 4_i32));
    assert_eq!(v.yx(), ivec2(2_i32, 1_i32));
    assert_eq!(v.yy(), ivec2(2_i32, 2_i32));
    assert_eq!(v.yz(), ivec2(2_i32, 3_i32));
    assert_eq!(v.yw(), ivec2(2_i32, 4_i32));
    assert_eq!(v.zx(), ivec2(3_i32, 1_i32));
    assert_eq!(v.zy(), ivec2(3_i32, 2_i32));
    assert_eq!(v.zz(), ivec2(3_i32, 3_i32));
    assert_eq!(v.zw(), ivec2(3_i32, 4_i32));
    assert_eq!(v.wx(), ivec2(4_i32, 1_i32));
    assert_eq!(v.wy(), ivec2(4_i32, 2_i32));
    assert_eq!(v.wz(), ivec2(4_i32, 3_i32));
    assert_eq!(v.ww(), ivec2(4_i32, 4_i32));
    assert_eq!(v.with_xy(rhs2), ivec4(11_i32, 12_i32, 3_i32, 4_i32));
    assert_eq!(v.with_xz(rhs2), ivec4(11_i32, 2_i32, 12_i32, 4_i32));
    assert_eq!(v.with_xw(rhs2), ivec4(11_i32, 2_i32, 3_i32, 12_i32));
    assert_eq!(v.with_yx(rhs2), ivec4(12_i32, 11_i32, 3_i32, 4_i32));
    assert_eq!(v.with_yz(rhs2), ivec4(1_i32, 11_i32, 12_i32, 4_i32));
    assert_eq!(v.with_yw(rhs2), ivec4(1_i32, 11_i32, 3_i32, 12_i32));
    assert_eq!(v.with_zx(rhs2), ivec4(12_i32, 2_i32, 11_i32, 4_i32));
    assert_eq!(v.with_zy(rhs2), ivec4(1_i32, 12_i32, 11_i32, 4_i32));
    assert_eq!(v.with_zw(rhs2), ivec4(1_i32, 2_i32, 11_i32, 12_i32));
    assert_eq!(v.with_wx(rhs2), ivec4(12_i32, 2_i32, 3_i32, 11_i32));
    assert_eq!(v.with_wy(rhs2), ivec4(1_i32, 12_i32, 3_i32, 11_i32));
    assert_eq!(v.with_wz(rhs2), ivec4(1_i32, 2_i32, 12_i32, 11_i32));
});

glam_test!(test_ivec3_swizzles, {
    let v = ivec3(1_i32, 2_i32, 3_i32);
    let rhs2 = ivec2(11_i32, 12_i32);
    assert_eq!(v, v.xyz());
    assert_eq!(v.xxxx(), ivec4(1_i32, 1_i32, 1_i32, 1_i32));
    assert_eq!(v.xxxy(), ivec4(1_i32, 1_i32, 1_i32, 2_i32));
    assert_eq!(v.xxxz(), ivec4(1_i32, 1_i32, 1_i32, 3_i32));
    assert_eq!(v.xxyx(), ivec4(1_i32, 1_i32, 2_i32, 1_i32));
    assert_eq!(v.xxyy(), ivec4(1_i32, 1_i32, 2_i32, 2_i32));
    assert_eq!(v.xxyz(), ivec4(1_i32, 1_i32, 2_i32, 3_i32));
    assert_eq!(v.xxzx(), ivec4(1_i32, 1_i32, 3_i32, 1_i32));
    assert_eq!(v.xxzy(), ivec4(1_i32, 1_i32, 3_i32, 2_i32));
    assert_eq!(v.xxzz(), ivec4(1_i32, 1_i32, 3_i32, 3_i32));
    assert_eq!(v.xyxx(), ivec4(1_i32, 2_i32, 1_i32, 1_i32));
    assert_eq!(v.xyxy(), ivec4(1_i32, 2_i32, 1_i32, 2_i32));
    assert_eq!(v.xyxz(), ivec4(1_i32, 2_i32, 1_i32, 3_i32));
    assert_eq!(v.xyyx(), ivec4(1_i32, 2_i32, 2_i32, 1_i32));
    assert_eq!(v.xyyy(), ivec4(1_i32, 2_i32, 2_i32, 2_i32));
    assert_eq!(v.xyyz(), ivec4(1_i32, 2_i32, 2_i32, 3_i32));
    assert_eq!(v.xyzx(), ivec4(1_i32, 2_i32, 3_i32, 1_i32));
    assert_eq!(v.xyzy(), ivec4(1_i32, 2_i32, 3_i32, 2_i32));
    assert_eq!(v.xyzz(), ivec4(1_i32, 2_i32, 3_i32, 3_i32));
    assert_eq!(v.xzxx(), ivec4(1_i32, 3_i32, 1_i32, 1_i32));
    assert_eq!(v.xzxy(), ivec4(1_i32, 3_i32, 1_i32, 2_i32));
    assert_eq!(v.xzxz(), ivec4(1_i32, 3_i32, 1_i32, 3_i32));
    assert_eq!(v.xzyx(), ivec4(1_i32, 3_i32, 2_i32, 1_i32));
    assert_eq!(v.xzyy(), ivec4(1_i32, 3_i32, 2_i32, 2_i32));
    assert_eq!(v.xzyz(), ivec4(1_i32, 3_i32, 2_i32, 3_i32));
    assert_eq!(v.xzzx(), ivec4(1_i32, 3_i32, 3_i32, 1_i32));
    assert_eq!(v.xzzy(), ivec4(1_i32, 3_i32, 3_i32, 2_i32));
    assert_eq!(v.xzzz(), ivec4(1_i32, 3_i32, 3_i32, 3_i32));
    assert_eq!(v.yxxx(), ivec4(2_i32, 1_i32, 1_i32, 1_i32));
    assert_eq!(v.yxxy(), ivec4(2_i32, 1_i32, 1_i32, 2_i32));
    assert_eq!(v.yxxz(), ivec4(2_i32, 1_i32, 1_i32, 3_i32));
    assert_eq!(v.yxyx(), ivec4(2_i32, 1_i32, 2_i32, 1_i32));
    assert_eq!(v.yxyy(), ivec4(2_i32, 1_i32, 2_i32, 2_i32));
    assert_eq!(v.yxyz(), ivec4(2_i32, 1_i32, 2_i32, 3_i32));
    assert_eq!(v.yxzx(), ivec4(2_i32, 1_i32, 3_i32, 1_i32));
    assert_eq!(v.yxzy(), ivec4(2_i32, 1_i32, 3_i32, 2_i32));
    assert_eq!(v.yxzz(), ivec4(2_i32, 1_i32, 3_i32, 3_i32));
    assert_eq!(v.yyxx(), ivec4(2_i32, 2_i32, 1_i32, 1_i32));
    assert_eq!(v.yyxy(), ivec4(2_i32, 2_i32, 1_i32, 2_i32));
    assert_eq!(v.yyxz(), ivec4(2_i32, 2_i32, 1_i32, 3_i32));
    assert_eq!(v.yyyx(), ivec4(2_i32, 2_i32, 2_i32, 1_i32));
    assert_eq!(v.yyyy(), ivec4(2_i32, 2_i32, 2_i32, 2_i32));
    assert_eq!(v.yyyz(), ivec4(2_i32, 2_i32, 2_i32, 3_i32));
    assert_eq!(v.yyzx(), ivec4(2_i32, 2_i32, 3_i32, 1_i32));
    assert_eq!(v.yyzy(), ivec4(2_i32, 2_i32, 3_i32, 2_i32));
    assert_eq!(v.yyzz(), ivec4(2_i32, 2_i32, 3_i32, 3_i32));
    assert_eq!(v.yzxx(), ivec4(2_i32, 3_i32, 1_i32, 1_i32));
    assert_eq!(v.yzxy(), ivec4(2_i32, 3_i32, 1_i32, 2_i32));
    assert_eq!(v.yzxz(), ivec4(2_i32, 3_i32, 1_i32, 3_i32));
    assert_eq!(v.yzyx(), ivec4(2_i32, 3_i32, 2_i32, 1_i32));
    assert_eq!(v.yzyy(), ivec4(2_i32, 3_i32, 2_i32, 2_i32));
    assert_eq!(v.yzyz(), ivec4(2_i32, 3_i32, 2_i32, 3_i32));
    assert_eq!(v.yzzx(), ivec4(2_i32, 3_i32, 3_i32, 1_i32));
    assert_eq!(v.yzzy(), ivec4(2_i32, 3_i32, 3_i32, 2_i32));
    assert_eq!(v.yzzz(), ivec4(2_i32, 3_i32, 3_i32, 3_i32));
    assert_eq!(v.zxxx(), ivec4(3_i32, 1_i32, 1_i32, 1_i32));
    assert_eq!(v.zxxy(), ivec4(3_i32, 1_i32, 1_i32, 2_i32));
    assert_eq!(v.zxxz(), ivec4(3_i32, 1_i32, 1_i32, 3_i32));
    assert_eq!(v.zxyx(), ivec4(3_i32, 1_i32, 2_i32, 1_i32));
    assert_eq!(v.zxyy(), ivec4(3_i32, 1_i32, 2_i32, 2_i32));
    assert_eq!(v.zxyz(), ivec4(3_i32, 1_i32, 2_i32, 3_i32));
    assert_eq!(v.zxzx(), ivec4(3_i32, 1_i32, 3_i32, 1_i32));
    assert_eq!(v.zxzy(), ivec4(3_i32, 1_i32, 3_i32, 2_i32));
    assert_eq!(v.zxzz(), ivec4(3_i32, 1_i32, 3_i32, 3_i32));
    assert_eq!(v.zyxx(), ivec4(3_i32, 2_i32, 1_i32, 1_i32));
    assert_eq!(v.zyxy(), ivec4(3_i32, 2_i32, 1_i32, 2_i32));
    assert_eq!(v.zyxz(), ivec4(3_i32, 2_i32, 1_i32, 3_i32));
    assert_eq!(v.zyyx(), ivec4(3_i32, 2_i32, 2_i32, 1_i32));
    assert_eq!(v.zyyy(), ivec4(3_i32, 2_i32, 2_i32, 2_i32));
    assert_eq!(v.zyyz(), ivec4(3_i32, 2_i32, 2_i32, 3_i32));
    assert_eq!(v.zyzx(), ivec4(3_i32, 2_i32, 3_i32, 1_i32));
    assert_eq!(v.zyzy(), ivec4(3_i32, 2_i32, 3_i32, 2_i32));
    assert_eq!(v.zyzz(), ivec4(3_i32, 2_i32, 3_i32, 3_i32));
    assert_eq!(v.zzxx(), ivec4(3_i32, 3_i32, 1_i32, 1_i32));
    assert_eq!(v.zzxy(), ivec4(3_i32, 3_i32, 1_i32, 2_i32));
    assert_eq!(v.zzxz(), ivec4(3_i32, 3_i32, 1_i32, 3_i32));
    assert_eq!(v.zzyx(), ivec4(3_i32, 3_i32, 2_i32, 1_i32));
    assert_eq!(v.zzyy(), ivec4(3_i32, 3_i32, 2_i32, 2_i32));
    assert_eq!(v.zzyz(), ivec4(3_i32, 3_i32, 2_i32, 3_i32));
    assert_eq!(v.zzzx(), ivec4(3_i32, 3_i32, 3_i32, 1_i32));
    assert_eq!(v.zzzy(), ivec4(3_i32, 3_i32, 3_i32, 2_i32));
    assert_eq!(v.zzzz(), ivec4(3_i32, 3_i32, 3_i32, 3_i32));
    assert_eq!(v.xxx(), ivec3(1_i32, 1_i32, 1_i32));
    assert_eq!(v.xxy(), ivec3(1_i32, 1_i32, 2_i32));
    assert_eq!(v.xxz(), ivec3(1_i32, 1_i32, 3_i32));
    assert_eq!(v.xyx(), ivec3(1_i32, 2_i32, 1_i32));
    assert_eq!(v.xyy(), ivec3(1_i32, 2_i32, 2_i32));
    assert_eq!(v.xzx(), ivec3(1_i32, 3_i32, 1_i32));
    assert_eq!(v.xzy(), ivec3(1_i32, 3_i32, 2_i32));
    assert_eq!(v.xzz(), ivec3(1_i32, 3_i32, 3_i32));
    assert_eq!(v.yxx(), ivec3(2_i32, 1_i32, 1_i32));
    assert_eq!(v.yxy(), ivec3(2_i32, 1_i32, 2_i32));
    assert_eq!(v.yxz(), ivec3(2_i32, 1_i32, 3_i32));
    assert_eq!(v.yyx(), ivec3(2_i32, 2_i32, 1_i32));
    assert_eq!(v.yyy(), ivec3(2_i32, 2_i32, 2_i32));
    assert_eq!(v.yyz(), ivec3(2_i32, 2_i32, 3_i32));
    assert_eq!(v.yzx(), ivec3(2_i32, 3_i32, 1_i32));
    assert_eq!(v.yzy(), ivec3(2_i32, 3_i32, 2_i32));
    assert_eq!(v.yzz(), ivec3(2_i32, 3_i32, 3_i32));
    assert_eq!(v.zxx(), ivec3(3_i32, 1_i32, 1_i32));
    assert_eq!(v.zxy(), ivec3(3_i32, 1_i32, 2_i32));
    assert_eq!(v.zxz(), ivec3(3_i32, 1_i32, 3_i32));
    assert_eq!(v.zyx(), ivec3(3_i32, 2_i32, 1_i32));
    assert_eq!(v.zyy(), ivec3(3_i32, 2_i32, 2_i32));
    assert_eq!(v.zyz(), ivec3(3_i32, 2_i32, 3_i32));
    assert_eq!(v.zzx(), ivec3(3_i32, 3_i32, 1_i32));
    assert_eq!(v.zzy(), ivec3(3_i32, 3_i32, 2_i32));
    assert_eq!(v.zzz(), ivec3(3_i32, 3_i32, 3_i32));
    assert_eq!(v.xx(), ivec2(1_i32, 1_i32));
    assert_eq!(v.xy(), ivec2(1_i32, 2_i32));
    assert_eq!(v.xz(), ivec2(1_i32, 3_i32));
    assert_eq!(v.yx(), ivec2(2_i32, 1_i32));
    assert_eq!(v.yy(), ivec2(2_i32, 2_i32));
    assert_eq!(v.yz(), ivec2(2_i32, 3_i32));
    assert_eq!(v.zx(), ivec2(3_i32, 1_i32));
    assert_eq!(v.zy(), ivec2(3_i32, 2_i32));
    assert_eq!(v.zz(), ivec2(3_i32, 3_i32));
    assert_eq!(v.with_xy(rhs2), ivec3(11_i32, 12_i32, 3_i32));
    assert_eq!(v.with_xz(rhs2), ivec3(11_i32, 2_i32, 12_i32));
    assert_eq!(v.with_yx(rhs2), ivec3(12_i32, 11_i32, 3_i32));
    assert_eq!(v.with_yz(rhs2), ivec3(1_i32, 11_i32, 12_i32));
    assert_eq!(v.with_zx(rhs2), ivec3(12_i32, 2_i32, 11_i32));
    assert_eq!(v.with_zy(rhs2), ivec3(1_i32, 12_i32, 11_i32));
});

glam_test!(test_ivec2_swizzles, {
    let v = ivec2(1_i32, 2_i32);
    assert_eq!(v, v.xy());
    assert_eq!(v.xxxx(), ivec4(1_i32, 1_i32, 1_i32, 1_i32));
    assert_eq!(v.xxxy(), ivec4(1_i32, 1_i32, 1_i32, 2_i32));
    assert_eq!(v.xxyx(), ivec4(1_i32, 1_i32, 2_i32, 1_i32));
    assert_eq!(v.xxyy(), ivec4(1_i32, 1_i32, 2_i32, 2_i32));
    assert_eq!(v.xyxx(), ivec4(1_i32, 2_i32, 1_i32, 1_i32));
    assert_eq!(v.xyxy(), ivec4(1_i32, 2_i32, 1_i32, 2_i32));
    assert_eq!(v.xyyx(), ivec4(1_i32, 2_i32, 2_i32, 1_i32));
    assert_eq!(v.xyyy(), ivec4(1_i32, 2_i32, 2_i32, 2_i32));
    assert_eq!(v.yxxx(), ivec4(2_i32, 1_i32, 1_i32, 1_i32));
    assert_eq!(v.yxxy(), ivec4(2_i32, 1_i32, 1_i32, 2_i32));
    assert_eq!(v.yxyx(), ivec4(2_i32, 1_i32, 2_i32, 1_i32));
    assert_eq!(v.yxyy(), ivec4(2_i32, 1_i32, 2_i32, 2_i32));
    assert_eq!(v.yyxx(), ivec4(2_i32, 2_i32, 1_i32, 1_i32));
    assert_eq!(v.yyxy(), ivec4(2_i32, 2_i32, 1_i32, 2_i32));
    assert_eq!(v.yyyx(), ivec4(2_i32, 2_i32, 2_i32, 1_i32));
    assert_eq!(v.yyyy(), ivec4(2_i32, 2_i32, 2_i32, 2_i32));
    assert_eq!(v.xxx(), ivec3(1_i32, 1_i32, 1_i32));
    assert_eq!(v.xxy(), ivec3(1_i32, 1_i32, 2_i32));
    assert_eq!(v.xyx(), ivec3(1_i32, 2_i32, 1_i32));
    assert_eq!(v.xyy(), ivec3(1_i32, 2_i32, 2_i32));
    assert_eq!(v.yxx(), ivec3(2_i32, 1_i32, 1_i32));
    assert_eq!(v.yxy(), ivec3(2_i32, 1_i32, 2_i32));
    assert_eq!(v.yyx(), ivec3(2_i32, 2_i32, 1_i32));
    assert_eq!(v.yyy(), ivec3(2_i32, 2_i32, 2_i32));
    assert_eq!(v.xx(), ivec2(1_i32, 1_i32));
    assert_eq!(v.yx(), ivec2(2_i32, 1_i32));
    assert_eq!(v.yy(), ivec2(2_i32, 2_i32));
});
