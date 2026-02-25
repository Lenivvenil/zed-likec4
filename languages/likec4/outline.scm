(specification_block "specification" @context @name) @item

(model_block "model" @context @name) @item

(views_block "views" @context @name) @item

(deployment_block "deployment" @context @name) @item

(global_block "global" @context @name) @item

(element_kind_declaration
  "element" @context
  name: (identifier) @name) @item

(tag_declaration
  "tag" @context
  name: (identifier) @name) @item

(relationship_kind_declaration
  "relationship" @context
  name: (identifier) @name) @item

(element_declaration
  kind: (identifier) @context
  name: (identifier) @name) @item

(view_declaration
  "view" @context
  name: (identifier) @name) @item

(dynamic_view_declaration
  "dynamic" @context
  name: (identifier) @name) @item

(deployment_node
  kind: (identifier) @context
  name: (identifier) @name) @item
