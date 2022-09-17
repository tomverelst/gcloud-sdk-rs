/// Source information collected at parse time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceInfo {
    /// The location name. All position information attached to an expression is
    /// relative to this location.
    ///
    /// The location could be a file, UI element, or similar. For example,
    /// `acme/app/AnvilPolicy.cel`.
    #[prost(string, tag="2")]
    pub location: ::prost::alloc::string::String,
    /// Monotonically increasing list of character offsets where newlines appear.
    ///
    /// The line number of a given position is the index `i` where for a given
    /// `id` the `line_offsets\[i\] < id_positions\[id\] < line_offsets\[i+1\]`. The
    /// column may be derivd from `id_positions\[id\] - line_offsets\[i\]`.
    #[prost(int32, repeated, tag="3")]
    pub line_offsets: ::prost::alloc::vec::Vec<i32>,
    /// A map from the parse node id (e.g. `Expr.id`) to the character offset
    /// within source.
    #[prost(map="int32, int32", tag="4")]
    pub positions: ::std::collections::HashMap<i32, i32>,
}
/// A specific position in source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourcePosition {
    /// The soucre location name (e.g. file name).
    #[prost(string, tag="1")]
    pub location: ::prost::alloc::string::String,
    /// The character offset.
    #[prost(int32, tag="2")]
    pub offset: i32,
    /// The 1-based index of the starting line in the source text
    /// where the issue occurs, or 0 if unknown.
    #[prost(int32, tag="3")]
    pub line: i32,
    /// The 0-based index of the starting position within the line of source text
    /// where the issue occurs.  Only meaningful if line is nonzer..
    #[prost(int32, tag="4")]
    pub column: i32,
}
/// An expression together with source information as returned by the parser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParsedExpr {
    /// The parsed expression.
    #[prost(message, optional, tag="2")]
    pub expr: ::core::option::Option<Expr>,
    /// The source info derived from input that generated the parsed `expr`.
    #[prost(message, optional, tag="3")]
    pub source_info: ::core::option::Option<SourceInfo>,
    /// The syntax version of the source, e.g. `cel1`.
    #[prost(string, tag="4")]
    pub syntax_version: ::prost::alloc::string::String,
}
/// An abstract representation of a common expression.
///
/// Expressions are abstractly represented as a collection of identifiers,
/// select statements, function calls, literals, and comprehensions. All
/// operators with the exception of the '.' operator are modelled as function
/// calls. This makes it easy to represent new operators into the existing AST.
///
/// All references within expressions must resolve to a \[Decl][google.api.expr.v1beta1.Decl\] provided at
/// type-check for an expression to be valid. A reference may either be a bare
/// identifier `name` or a qualified identifier `google.api.name`. References
/// may either refer to a value or a function declaration.
///
/// For example, the expression `google.api.name.startsWith('expr')` references
/// the declaration `google.api.name` within a \[Expr.Select][google.api.expr.v1beta1.Expr.Select\] expression, and
/// the function declaration `startsWith`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expr {
    /// Required. An id assigned to this node by the parser which is unique in a
    /// given expression tree. This is used to associate type information and other
    /// attributes to a node in the parse tree.
    #[prost(int32, tag="2")]
    pub id: i32,
    /// Required. Variants of expressions.
    #[prost(oneof="expr::ExprKind", tags="3, 4, 5, 6, 7, 8, 9")]
    pub expr_kind: ::core::option::Option<expr::ExprKind>,
}
/// Nested message and enum types in `Expr`.
pub mod expr {
    /// An identifier expression. e.g. `request`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Ident {
        /// Required. Holds a single, unqualified identifier, possibly preceded by a
        /// '.'.
        ///
        /// Qualified names are represented by the \[Expr.Select][google.api.expr.v1beta1.Expr.Select\] expression.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
    }
    /// A field selection expression. e.g. `request.auth`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Select {
        /// Required. The target of the selection expression.
        ///
        /// For example, in the select expression `request.auth`, the `request`
        /// portion of the expression is the `operand`.
        #[prost(message, optional, boxed, tag="1")]
        pub operand: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
        /// Required. The name of the field to select.
        ///
        /// For example, in the select expression `request.auth`, the `auth` portion
        /// of the expression would be the `field`.
        #[prost(string, tag="2")]
        pub field: ::prost::alloc::string::String,
        /// Whether the select is to be interpreted as a field presence test.
        ///
        /// This results from the macro `has(request.auth)`.
        #[prost(bool, tag="3")]
        pub test_only: bool,
    }
    /// A call expression, including calls to predefined functions and operators.
    ///
    /// For example, `value == 10`, `size(map_value)`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Call {
        /// The target of an method call-style expression. For example, `x` in
        /// `x.f()`.
        #[prost(message, optional, boxed, tag="1")]
        pub target: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
        /// Required. The name of the function or method being called.
        #[prost(string, tag="2")]
        pub function: ::prost::alloc::string::String,
        /// The arguments.
        #[prost(message, repeated, tag="3")]
        pub args: ::prost::alloc::vec::Vec<super::Expr>,
    }
    /// A list creation expression.
    ///
    /// Lists may either be homogenous, e.g. `[1, 2, 3]`, or heterogenous, e.g.
    /// `dyn([1, 'hello', 2.0])`
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateList {
        /// The elements part of the list.
        #[prost(message, repeated, tag="1")]
        pub elements: ::prost::alloc::vec::Vec<super::Expr>,
    }
    /// A map or message creation expression.
    ///
    /// Maps are constructed as `{'key_name': 'value'}`. Message construction is
    /// similar, but prefixed with a type name and composed of field ids:
    /// `types.MyType{field_id: 'value'}`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateStruct {
        /// The type name of the message to be created, empty when creating map
        /// literals.
        #[prost(string, tag="1")]
        pub r#type: ::prost::alloc::string::String,
        /// The entries in the creation expression.
        #[prost(message, repeated, tag="2")]
        pub entries: ::prost::alloc::vec::Vec<create_struct::Entry>,
    }
    /// Nested message and enum types in `CreateStruct`.
    pub mod create_struct {
        /// Represents an entry.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Entry {
            /// Required. An id assigned to this node by the parser which is unique
            /// in a given expression tree. This is used to associate type
            /// information and other attributes to the node.
            #[prost(int32, tag="1")]
            pub id: i32,
            /// Required. The value assigned to the key.
            #[prost(message, optional, tag="4")]
            pub value: ::core::option::Option<super::super::Expr>,
            /// The `Entry` key kinds.
            #[prost(oneof="entry::KeyKind", tags="2, 3")]
            pub key_kind: ::core::option::Option<entry::KeyKind>,
        }
        /// Nested message and enum types in `Entry`.
        pub mod entry {
            /// The `Entry` key kinds.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum KeyKind {
                /// The field key for a message creator statement.
                #[prost(string, tag="2")]
                FieldKey(::prost::alloc::string::String),
                /// The key expression for a map creation statement.
                #[prost(message, tag="3")]
                MapKey(super::super::super::Expr),
            }
        }
    }
    /// A comprehension expression applied to a list or map.
    ///
    /// Comprehensions are not part of the core syntax, but enabled with macros.
    /// A macro matches a specific call signature within a parsed AST and replaces
    /// the call with an alternate AST block. Macro expansion happens at parse
    /// time.
    ///
    /// The following macros are supported within CEL:
    ///
    /// Aggregate type macros may be applied to all elements in a list or all keys
    /// in a map:
    ///
    /// *  `all`, `exists`, `exists_one` -  test a predicate expression against
    ///     the inputs and return `true` if the predicate is satisfied for all,
    ///     any, or only one value `list.all(x, x < 10)`.
    /// *  `filter` - test a predicate expression against the inputs and return
    ///     the subset of elements which satisfy the predicate:
    ///     `payments.filter(p, p > 1000)`.
    /// *  `map` - apply an expression to all elements in the input and return the
    ///     output aggregate type: `[1, 2, 3].map(i, i * i)`.
    ///
    /// The `has(m.x)` macro tests whether the property `x` is present in struct
    /// `m`. The semantics of this macro depend on the type of `m`. For proto2
    /// messages `has(m.x)` is defined as 'defined, but not set`. For proto3, the
    /// macro tests whether the property is set to its default. For map and struct
    /// types, the macro tests whether the property `x` is defined on `m`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Comprehension {
        /// The name of the iteration variable.
        #[prost(string, tag="1")]
        pub iter_var: ::prost::alloc::string::String,
        /// The range over which var iterates.
        #[prost(message, optional, boxed, tag="2")]
        pub iter_range: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
        /// The name of the variable used for accumulation of the result.
        #[prost(string, tag="3")]
        pub accu_var: ::prost::alloc::string::String,
        /// The initial value of the accumulator.
        #[prost(message, optional, boxed, tag="4")]
        pub accu_init: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
        /// An expression which can contain iter_var and accu_var.
        ///
        /// Returns false when the result has been computed and may be used as
        /// a hint to short-circuit the remainder of the comprehension.
        #[prost(message, optional, boxed, tag="5")]
        pub loop_condition: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
        /// An expression which can contain iter_var and accu_var.
        ///
        /// Computes the next value of accu_var.
        #[prost(message, optional, boxed, tag="6")]
        pub loop_step: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
        /// An expression which can contain accu_var.
        ///
        /// Computes the result.
        #[prost(message, optional, boxed, tag="7")]
        pub result: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
    }
    /// Required. Variants of expressions.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExprKind {
        /// A literal expression.
        #[prost(message, tag="3")]
        LiteralExpr(super::Literal),
        /// An identifier expression.
        #[prost(message, tag="4")]
        IdentExpr(Ident),
        /// A field selection expression, e.g. `request.auth`.
        #[prost(message, tag="5")]
        SelectExpr(::prost::alloc::boxed::Box<Select>),
        /// A call expression, including calls to predefined functions and operators.
        #[prost(message, tag="6")]
        CallExpr(::prost::alloc::boxed::Box<Call>),
        /// A list creation expression.
        #[prost(message, tag="7")]
        ListExpr(CreateList),
        /// A map or object creation expression.
        #[prost(message, tag="8")]
        StructExpr(CreateStruct),
        /// A comprehension expression.
        #[prost(message, tag="9")]
        ComprehensionExpr(::prost::alloc::boxed::Box<Comprehension>),
    }
}
/// Represents a primitive literal.
///
/// This is similar to the primitives supported in the well-known type
/// `google.protobuf.Value`, but richer so it can represent CEL's full range of
/// primitives.
///
/// Lists and structs are not included as constants as these aggregate types may
/// contain \[Expr][google.api.expr.v1beta1.Expr\] elements which require evaluation and are thus not constant.
///
/// Examples of literals include: `"hello"`, `b'bytes'`, `1u`, `4.2`, `-2`,
/// `true`, `null`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Literal {
    /// Required. The valid constant kinds.
    #[prost(oneof="literal::ConstantKind", tags="1, 2, 3, 4, 5, 6, 7")]
    pub constant_kind: ::core::option::Option<literal::ConstantKind>,
}
/// Nested message and enum types in `Literal`.
pub mod literal {
    /// Required. The valid constant kinds.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConstantKind {
        /// null value.
        #[prost(enumeration="::prost_types::NullValue", tag="1")]
        NullValue(i32),
        /// boolean value.
        #[prost(bool, tag="2")]
        BoolValue(bool),
        /// int64 value.
        #[prost(int64, tag="3")]
        Int64Value(i64),
        /// uint64 value.
        #[prost(uint64, tag="4")]
        Uint64Value(u64),
        /// double value.
        #[prost(double, tag="5")]
        DoubleValue(f64),
        /// string value.
        #[prost(string, tag="6")]
        StringValue(::prost::alloc::string::String),
        /// bytes value.
        #[prost(bytes, tag="7")]
        BytesValue(::prost::alloc::vec::Vec<u8>),
    }
}
/// A declaration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decl {
    /// The id of the declaration.
    #[prost(int32, tag="1")]
    pub id: i32,
    /// The name of the declaration.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The documentation string for the declaration.
    #[prost(string, tag="3")]
    pub doc: ::prost::alloc::string::String,
    /// The kind of declaration.
    #[prost(oneof="decl::Kind", tags="4, 5")]
    pub kind: ::core::option::Option<decl::Kind>,
}
/// Nested message and enum types in `Decl`.
pub mod decl {
    /// The kind of declaration.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// An identifier declaration.
        #[prost(message, tag="4")]
        Ident(super::IdentDecl),
        /// A function declaration.
        #[prost(message, tag="5")]
        Function(super::FunctionDecl),
    }
}
/// The declared type of a variable.
///
/// Extends runtime type values with extra information used for type checking
/// and dispatching.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeclType {
    /// The expression id of the declared type, if applicable.
    #[prost(int32, tag="1")]
    pub id: i32,
    /// The type name, e.g. 'int', 'my.type.Type' or 'T'
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// An ordered list of type parameters, e.g. `<string, int>`.
    /// Only applies to a subset of types, e.g. `map`, `list`.
    #[prost(message, repeated, tag="4")]
    pub type_params: ::prost::alloc::vec::Vec<DeclType>,
}
/// An identifier declaration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentDecl {
    /// Optional type of the identifier.
    #[prost(message, optional, tag="3")]
    pub r#type: ::core::option::Option<DeclType>,
    /// Optional value of the identifier.
    #[prost(message, optional, tag="4")]
    pub value: ::core::option::Option<Expr>,
}
/// A function declaration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionDecl {
    /// The function arguments.
    #[prost(message, repeated, tag="1")]
    pub args: ::prost::alloc::vec::Vec<IdentDecl>,
    /// Optional declared return type.
    #[prost(message, optional, tag="2")]
    pub return_type: ::core::option::Option<DeclType>,
    /// If the first argument of the function is the receiver.
    #[prost(bool, tag="3")]
    pub receiver_function: bool,
}
/// Represents a CEL value.
///
/// This is similar to `google.protobuf.Value`, but can represent CEL's full
/// range of values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// Required. The valid kinds of values.
    #[prost(oneof="value::Kind", tags="1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 15")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// Required. The valid kinds of values.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Null value.
        #[prost(enumeration="::prost_types::NullValue", tag="1")]
        NullValue(i32),
        /// Boolean value.
        #[prost(bool, tag="2")]
        BoolValue(bool),
        /// Signed integer value.
        #[prost(int64, tag="3")]
        Int64Value(i64),
        /// Unsigned integer value.
        #[prost(uint64, tag="4")]
        Uint64Value(u64),
        /// Floating point value.
        #[prost(double, tag="5")]
        DoubleValue(f64),
        /// UTF-8 string value.
        #[prost(string, tag="6")]
        StringValue(::prost::alloc::string::String),
        /// Byte string value.
        #[prost(bytes, tag="7")]
        BytesValue(::prost::alloc::vec::Vec<u8>),
        /// An enum value.
        #[prost(message, tag="9")]
        EnumValue(super::EnumValue),
        /// The proto message backing an object value.
        #[prost(message, tag="10")]
        ObjectValue(::prost_types::Any),
        /// Map value.
        #[prost(message, tag="11")]
        MapValue(super::MapValue),
        /// List value.
        #[prost(message, tag="12")]
        ListValue(super::ListValue),
        /// A Type value represented by the fully qualified name of the type.
        #[prost(string, tag="15")]
        TypeValue(::prost::alloc::string::String),
    }
}
/// An enum value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValue {
    /// The fully qualified name of the enum type.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// The value of the enum.
    #[prost(int32, tag="2")]
    pub value: i32,
}
/// A list.
///
/// Wrapped in a message so 'not set' and empty can be differentiated, which is
/// required for use in a 'oneof'.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValue {
    /// The ordered values in the list.
    #[prost(message, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
/// A map.
///
/// Wrapped in a message so 'not set' and empty can be differentiated, which is
/// required for use in a 'oneof'.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapValue {
    /// The set of map entries.
    ///
    /// CEL has fewer restrictions on keys, so a protobuf map represenation
    /// cannot be used.
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<map_value::Entry>,
}
/// Nested message and enum types in `MapValue`.
pub mod map_value {
    /// An entry in the map.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        /// The key.
        ///
        /// Must be unique with in the map.
        /// Currently only boolean, int, uint, and string values can be keys.
        #[prost(message, optional, tag="1")]
        pub key: ::core::option::Option<super::Value>,
        /// The value.
        #[prost(message, optional, tag="2")]
        pub value: ::core::option::Option<super::Value>,
    }
}
/// The state of an evaluation.
///
/// Can represent an initial, partial, or completed state of evaluation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvalState {
    /// The unique values referenced in this message.
    #[prost(message, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<ExprValue>,
    /// An ordered list of results.
    ///
    /// Tracks the flow of evaluation through the expression.
    /// May be sparse.
    #[prost(message, repeated, tag="3")]
    pub results: ::prost::alloc::vec::Vec<eval_state::Result>,
}
/// Nested message and enum types in `EvalState`.
pub mod eval_state {
    /// A single evaluation result.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        /// The expression this result is for.
        #[prost(message, optional, tag="1")]
        pub expr: ::core::option::Option<super::IdRef>,
        /// The index in `values` of the resulting value.
        #[prost(int32, tag="2")]
        pub value: i32,
    }
}
/// The value of an evaluated expression.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExprValue {
    /// An expression can resolve to a value, error or unknown.
    #[prost(oneof="expr_value::Kind", tags="1, 2, 3")]
    pub kind: ::core::option::Option<expr_value::Kind>,
}
/// Nested message and enum types in `ExprValue`.
pub mod expr_value {
    /// An expression can resolve to a value, error or unknown.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// A concrete value.
        #[prost(message, tag="1")]
        Value(super::Value),
        /// The set of errors in the critical path of evalution.
        ///
        /// Only errors in the critical path are included. For example,
        /// `(<error1> || true) && <error2>` will only result in `<error2>`,
        /// while `<error1> || <error2>` will result in both `<error1>` and
        /// `<error2>`.
        ///
        /// Errors cause by the presence of other errors are not included in the
        /// set. For example `<error1>.foo`, `foo(<error1>)`, and `<error1> + 1` will
        /// only result in `<error1>`.
        ///
        /// Multiple errors *might* be included when evaluation could result
        /// in different errors. For example `<error1> + <error2>` and
        /// `foo(<error1>, <error2>)` may result in `<error1>`, `<error2>` or both.
        /// The exact subset of errors included for this case is unspecified and
        /// depends on the implementation details of the evaluator.
        #[prost(message, tag="2")]
        Error(super::ErrorSet),
        /// The set of unknowns in the critical path of evaluation.
        ///
        /// Unknown behaves identically to Error with regards to propagation.
        /// Specifically, only unknowns in the critical path are included, unknowns
        /// caused by the presence of other unknowns are not included, and multiple
        /// unknowns *might* be included included when evaluation could result in
        /// different unknowns. For example:
        ///
        ///      (<unknown\[1\]> || true) && <unknown\[2\]> -> <unknown\[2\]>
        ///      <unknown\[1\]> || <unknown\[2\]> -> <unknown\[1,2\]>
        ///      <unknown\[1\]>.foo -> <unknown\[1\]>
        ///      foo(<unknown\[1\]>) -> <unknown\[1\]>
        ///      <unknown\[1\]> + <unknown\[2\]> -> <unknown\[1\]> or <unknown[2[>
        ///
        /// Unknown takes precidence over Error in cases where a `Value` can short
        /// circuit the result:
        ///
        ///      <error> || <unknown> -> <unknown>
        ///      <error> && <unknown> -> <unknown>
        ///
        /// Errors take precidence in all other cases:
        ///
        ///      <unknown> + <error> -> <error>
        ///      foo(<unknown>, <error>) -> <error>
        #[prost(message, tag="3")]
        Unknown(super::UnknownSet),
    }
}
/// A set of errors.
///
/// The errors included depend on the context. See `ExprValue.error`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorSet {
    /// The errors in the set.
    #[prost(message, repeated, tag="1")]
    pub errors: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// A set of expressions for which the value is unknown.
///
/// The unknowns included depend on the context. See `ExprValue.unknown`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnknownSet {
    /// The ids of the expressions with unknown values.
    #[prost(message, repeated, tag="1")]
    pub exprs: ::prost::alloc::vec::Vec<IdRef>,
}
/// A reference to an expression id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdRef {
    /// The expression id.
    #[prost(int32, tag="1")]
    pub id: i32,
}
