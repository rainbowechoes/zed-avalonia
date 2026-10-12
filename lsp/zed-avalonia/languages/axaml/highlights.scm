(Comment) @comment

(STag (Name) @tag)
(ETag (Name) @tag)
(EmptyElemTag (Name) @tag)

(Attribute (Name) @property)
(Attribute (AttValue) @string)

(EntityRef) @constant
(CharRef) @constant

(PI) @embedded
(doctypedecl (Name) @type)

[
  "<?" "?>"
  "<!" "]]>"
  "<" ">"
  "</" "/>"
] @punctuation.delimiter

[ "\"" "'" ] @punctuation.delimiter
[ "=" ] @operator
