var ALIASES = {};
ALIASES['libc'] = {};
ALIASES['macro_attr'] = {};
ALIASES['memchr'] = {};
ALIASES['nom'] = {"/=":[{'crate':'nom','ty':8,'name':'DivAssign','desc':'The division assignment operator `/=`.','p':'nom::lib::std::ops'}],"*":[{'crate':'nom','ty':8,'name':'Mul','desc':'The multiplication operator `*`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'DerefMut','desc':'Used for mutable dereferencing operations, like in `*v = 1;`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Deref','desc':'Used for immutable dereferencing operations, like `*v`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'MulAssign','desc':'The multiplication assignment operator `*=`.','p':'nom::lib::std::ops'}],"..=":[{'crate':'nom','ty':3,'name':'RangeInclusive','desc':'An range bounded inclusively below and above (`start..=end`).','p':'nom::lib::std::ops'},{'crate':'nom','ty':3,'name':'RangeToInclusive','desc':'A range only bounded inclusively above (`..=end`).','p':'nom::lib::std::ops'}],">=":[{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::prelude::v1::v1'}],"^=":[{'crate':'nom','ty':8,'name':'BitXorAssign','desc':'The bitwise XOR assignment operator `^=`.','p':'nom::lib::std::ops'}],">":[{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::prelude::v1::v1'}],"<=":[{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::prelude::v1::v1'}],"&=":[{'crate':'nom','ty':8,'name':'BitAndAssign','desc':'The bitwise AND assignment operator `&=`.','p':'nom::lib::std::ops'}],"^":[{'crate':'nom','ty':8,'name':'BitXor','desc':'The bitwise XOR operator `^`.','p':'nom::lib::std::ops'}],"[]":[{'crate':'nom','ty':8,'name':'IndexMut','desc':'Used for indexing operations (`container[index]`) in mutable contexts.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Index','desc':'Used for indexing operations (`container[index]`) in immutable contexts.','p':'nom::lib::std::ops'}],"+":[{'crate':'nom','ty':8,'name':'Add','desc':'The addition operator `+`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'AddAssign','desc':'The addition assignment operator `+=`.','p':'nom::lib::std::ops'}],">>=":[{'crate':'nom','ty':8,'name':'ShrAssign','desc':'The right shift assignment operator `>>=`.','p':'nom::lib::std::ops'}],"%":[{'crate':'nom','ty':8,'name':'Rem','desc':'The remainder operator `%`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'RemAssign','desc':'The remainder assignment operator `%=`.','p':'nom::lib::std::ops'}],"/":[{'crate':'nom','ty':8,'name':'DivAssign','desc':'The division assignment operator `/=`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Div','desc':'The division operator `/`.','p':'nom::lib::std::ops'}],"[":[{'crate':'nom','ty':8,'name':'IndexMut','desc':'Used for indexing operations (`container[index]`) in mutable contexts.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Index','desc':'Used for indexing operations (`container[index]`) in immutable contexts.','p':'nom::lib::std::ops'}],"..":[{'crate':'nom','ty':3,'name':'RangeFrom','desc':'A range only bounded inclusively below (`start..`).','p':'nom::lib::std::ops'},{'crate':'nom','ty':3,'name':'RangeFull','desc':'An unbounded range (`..`).','p':'nom::lib::std::ops'},{'crate':'nom','ty':3,'name':'Range','desc':'A (half-open) range bounded inclusively below and exclusively above (`start..end`).','p':'nom::lib::std::ops'},{'crate':'nom','ty':3,'name':'RangeTo','desc':'A range only bounded exclusively above (`..end`).','p':'nom::lib::std::ops'}],"<<":[{'crate':'nom','ty':8,'name':'Shl','desc':'The left shift operator `<<`. Note that because this trait is implemented for all integer types with multiple right-hand-side types, Rust\'s type checker has special handling for `_ << _`, setting the result type for integer operations to the type of the left-hand-side operand. This means that though `a << b` and `a.shl(b)` are one and the same from an evaluation standpoint, they are different when it comes to type inference.','p':'nom::lib::std::ops'}],"{}":[{'crate':'nom','ty':8,'name':'Display','desc':'Format trait for an empty format, `{}`.','p':'nom::lib::std::fmt'}],"<":[{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Ord','desc':'Trait for types that form a total order.','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'PartialOrd','desc':'Trait for values that can be compared for a sort-order.','p':'nom::lib::std::prelude::v1::v1'}],"&":[{'crate':'nom','ty':8,'name':'BitAnd','desc':'The bitwise AND operator `&`.','p':'nom::lib::std::ops'}],"*=":[{'crate':'nom','ty':8,'name':'MulAssign','desc':'The multiplication assignment operator `*=`.','p':'nom::lib::std::ops'}],"|=":[{'crate':'nom','ty':8,'name':'BitOrAssign','desc':'The bitwise OR assignment operator `|=`.','p':'nom::lib::std::ops'}],"|":[{'crate':'nom','ty':8,'name':'BitOr','desc':'The bitwise OR operator `|`.','p':'nom::lib::std::ops'}],"&*":[{'crate':'nom','ty':8,'name':'Deref','desc':'Used for immutable dereferencing operations, like `*v`.','p':'nom::lib::std::ops'}],">>":[{'crate':'nom','ty':8,'name':'Shr','desc':'The right shift operator `>>`. Note that because this trait is implemented for all integer types with multiple right-hand-side types, Rust\'s type checker has special handling for `_ >> _`, setting the result type for integer operations to the type of the left-hand-side operand. This means that though `a >> b` and `a.shr(b)` are one and the same from an evaluation standpoint, they are different when it comes to type inference.','p':'nom::lib::std::ops'}],"-=":[{'crate':'nom','ty':8,'name':'SubAssign','desc':'The subtraction assignment operator `-=`.','p':'nom::lib::std::ops'}],"?":[{'crate':'nom','ty':8,'name':'Try','desc':'A trait for customizing the behavior of the `?` operator.','p':'nom::lib::std::ops'}],"==":[{'crate':'nom','ty':8,'name':'PartialEq','desc':'Trait for equality comparisons which are partial equivalence relations.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Eq','desc':'Trait for equality comparisons which are equivalence relations.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Eq','desc':'Trait for equality comparisons which are equivalence relations.','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'PartialEq','desc':'Trait for equality comparisons which are partial equivalence relations.','p':'nom::lib::std::prelude::v1::v1'}],"!=":[{'crate':'nom','ty':8,'name':'PartialEq','desc':'Trait for equality comparisons which are partial equivalence relations.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Eq','desc':'Trait for equality comparisons which are equivalence relations.','p':'nom::lib::std::cmp'},{'crate':'nom','ty':8,'name':'Eq','desc':'Trait for equality comparisons which are equivalence relations.','p':'nom::lib::std::prelude::v1::v1'},{'crate':'nom','ty':8,'name':'PartialEq','desc':'Trait for equality comparisons which are partial equivalence relations.','p':'nom::lib::std::prelude::v1::v1'}],"-":[{'crate':'nom','ty':8,'name':'SubAssign','desc':'The subtraction assignment operator `-=`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Neg','desc':'The unary negation operator `-`.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Sub','desc':'The subtraction operator `-`.','p':'nom::lib::std::ops'}],"+=":[{'crate':'nom','ty':8,'name':'AddAssign','desc':'The addition assignment operator `+=`.','p':'nom::lib::std::ops'}],"%=":[{'crate':'nom','ty':8,'name':'RemAssign','desc':'The remainder assignment operator `%=`.','p':'nom::lib::std::ops'}],"<<=":[{'crate':'nom','ty':8,'name':'ShlAssign','desc':'The left shift assignment operator `<<=`.','p':'nom::lib::std::ops'}],"{:?}":[{'crate':'nom','ty':8,'name':'Debug','desc':'`?` formatting.','p':'nom::lib::std::fmt'}],"]":[{'crate':'nom','ty':8,'name':'IndexMut','desc':'Used for indexing operations (`container[index]`) in mutable contexts.','p':'nom::lib::std::ops'},{'crate':'nom','ty':8,'name':'Index','desc':'Used for indexing operations (`container[index]`) in immutable contexts.','p':'nom::lib::std::ops'}],};
ALIASES['tl_lang_syn'] = {};
