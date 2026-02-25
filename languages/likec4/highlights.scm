; ── Top-level block keywords ──────────────────────────────────────
[
  "specification"
  "model"
  "views"
  "deployment"
  "global"
  "likec4lib"
] @keyword

; ── Specification keywords ───────────────────────────────────────
(element_kind_declaration "element" @keyword)
(tag_declaration "tag" @keyword)
(relationship_kind_declaration "relationship" @keyword)
(color_declaration "color" @keyword)
(deployment_node_kind_declaration "deploymentNode" @keyword)

; ── Model keywords ───────────────────────────────────────────────
(extend_element "extend" @keyword)
(extend_relation "extend" @keyword)
(instance_of "instanceOf" @keyword)

; ── View keywords ────────────────────────────────────────────────
(view_declaration "view" @keyword)
(view_declaration "deployment" @keyword)
(view_declaration "of" @keyword)
(view_declaration "extends" @keyword)
(dynamic_view_declaration "dynamic" @keyword)
(dynamic_view_declaration "view" @keyword)
(dynamic_view_declaration "of" @keyword)
(dynamic_view_declaration "extends" @keyword)

(include_statement "include" @keyword)
(exclude_statement "exclude" @keyword)
(auto_layout "autoLayout" @keyword)
(view_style_rule "style" @keyword)
(view_group "group" @keyword)
(view_rank "rank" @keyword)
(parallel_block "parallel" @keyword)
(parallel_block "par" @keyword)
(global_ref "global" @keyword)
(global_ref "predicate" @keyword)
(global_ref "style" @keyword)
(variant_property "variant" @keyword)

; ── Global keywords ──────────────────────────────────────────────
(predicate_group "predicateGroup" @keyword)
(dynamic_predicate_group "dynamicPredicateGroup" @keyword)
(global_style "style" @keyword)
(global_style_group "styleGroup" @keyword)

; ── Import ───────────────────────────────────────────────────────
(import_statement "import" @keyword)
(import_statement "from" @keyword)

; ── Where / with keywords ────────────────────────────────────────
(where_clause "where" @keyword)
(with_clause "with" @keyword)
(where_condition "is" @keyword)
(where_condition "not" @keyword)
(where_not "not" @keyword)
(where_and "and" @keyword)
(where_or "or" @keyword)

; ── Element filter keywords ──────────────────────────────────────
(element_filter "element.kind" @keyword)
(element_filter "element.tag" @keyword)

; ── Property keys ────────────────────────────────────────────────
[
  "title"
  "description"
  "technology"
  "notation"
  "notes"
  "summary"
  "shape"
  "border"
  "opacity"
  "icon"
  "iconColor"
  "iconSize"
  "iconPosition"
  "multiple"
  "size"
  "padding"
  "textSize"
  "line"
  "head"
  "tail"
] @property

(link_property "link" @property)
(icon_property "icon" @property)
(navigate_to "navigateTo" @property)
(metadata_block "metadata" @keyword)
(style_block "style" @keyword)

; ── Layout directions ────────────────────────────────────────────
[
  "TopBottom"
  "LeftRight"
  "BottomTop"
  "RightLeft"
] @constant

; ── Rank values ──────────────────────────────────────────────────
(view_rank ["same" "min" "max" "source" "sink"] @constant)

; ── Declarations ─────────────────────────────────────────────────
(element_kind_declaration name: (identifier) @type.class)
(tag_declaration name: (identifier) @type.class)
(relationship_kind_declaration name: (identifier) @type.class)
(color_declaration name: (identifier) @type.class)
(deployment_node_kind_declaration name: (identifier) @type.class)

; ── Element names ────────────────────────────────────────────────
(element_declaration kind: (identifier) @type)
(element_declaration name: (identifier) @entity.name)
(deployment_node kind: (identifier) @type)
(deployment_node name: (identifier) @entity.name)

; ── View names ───────────────────────────────────────────────────
(view_declaration name: (identifier) @entity.name)
(view_declaration extends: (identifier) @entity.name)
(dynamic_view_declaration name: (identifier) @entity.name)
(dynamic_view_declaration extends: (identifier) @entity.name)

; ── Global names ─────────────────────────────────────────────────
(predicate_group name: (identifier) @entity.name)
(dynamic_predicate_group name: (identifier) @entity.name)
(global_style name: (identifier) @entity.name)
(global_style_group name: (identifier) @entity.name)

; ── Relation arrows ──────────────────────────────────────────────
(arrow_directed) @operator
(arrow_backward) @operator
(arrow_bidirectional) @operator
(arrow_typed "-[" @operator)
(arrow_typed "]->" @operator)
(arrow_typed kind: (identifier) @type)
(dot_relation "." @operator)
(dot_relation kind: (identifier) @type)

; ── References ───────────────────────────────────────────────────
(fqn_ref (identifier) @variable)
(fqn_ref "." @punctuation.delimiter)
(descendant_ref [".*" ".**" "._"] @operator)

; ── Self-references ──────────────────────────────────────────────
["this" "it"] @variable.builtin

; ── Tags ─────────────────────────────────────────────────────────
(tag_ref "#" @punctuation.special)
(tag_ref (identifier) @tag)

; ── Wildcards ────────────────────────────────────────────────────
(wildcard) @operator

; ── Comparison operators ─────────────────────────────────────────
["=" "==" "!=" "!=="] @operator

; ── Literals ─────────────────────────────────────────────────────
(string) @string
(number) @number
(float) @number.float
(percentage) @number
(boolean) @constant.builtin
(hex_color "#" @punctuation.special)
(hex_digits) @number
(rgb_color "rgb" @function.builtin)
(rgba_color "rgba" @function.builtin)
(lib_icon) @string.special
(uri_with_schema) @string.special.url
(uri_relative) @string.special.url
(uri_alias) @string.special.url
"none" @constant.builtin

; ── Punctuation ──────────────────────────────────────────────────
["{" "}"] @punctuation.bracket
["(" ")"] @punctuation.bracket
["[" "]"] @punctuation.bracket
["," ";" ":"] @punctuation.delimiter

; ── Comments ─────────────────────────────────────────────────────
(comment) @comment
